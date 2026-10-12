import Foundation
import XCTest
@testable import HolochainConductor

/// iOS smoke test for the wasmi-interpreter conductor build: start the conductor in-process, install the
/// forum.happ fixture, have the conductor sign `posts.get_all_posts`, send it over the app websocket and expect
/// `zome_called`.
final class ConductorSmokeTests: XCTestCase {
    func testStartInstallSignAndCallZome() async throws {
        let dataRoot = FileManager.default.temporaryDirectory
            .appendingPathComponent("hc-smoke-\(UUID().uuidString)").path
        try FileManager.default.createDirectory(atPath: dataRoot, withIntermediateDirectories: true)

        // 1. Start the conductor (lair keystore + conductor, in-process).
        let runtime = try await RuntimeFfi.start(
            passphrase: Data("ios-smoke-test".utf8),
            runtimeConfig: RuntimeConfigFfi(
                dataRootPath: dataRoot,
                network: RuntimeNetworkConfigFfi(
                    bootstrapUrl: "https://dev-test-bootstrap2.holochain.org",
                    relayUrl: "https://use1-1.relay.n0.iroh-canary.iroh.link./")))

        // 2. Install + enable forum.happ (runs genesis WASM through wasmi) and get an app websocket.
        let happURL = try XCTUnwrap(Bundle.module.url(forResource: "forum", withExtension: "happ"))
        let appId = "forum-ios-smoke"
        let auth = try await runtime.setupApp(
            payload: InstallAppPayloadFfi(
                source: try Data(contentsOf: happURL), installedAppId: appId,
                networkSeed: UUID().uuidString, rolesSettings: nil, agentKey: nil),
            enableAfterInstall: true)
        XCTAssertGreaterThan(auth.port, 0)

        // 3. Cell id from the app info.
        let apps = try await runtime.listApps()
        let app = try XCTUnwrap(apps.first { $0.installedAppId == appId })
        var cellId: CellIdFfi?
        for cells in app.cellInfo.values {
            for cell in cells { if case .provisioned(let provisioned) = cell { cellId = provisioned.cellId } }
        }
        let cell = try XCTUnwrap(cellId)
        XCTAssertEqual(cell.agentPubKey.count, 39)

        // 4. The conductor signs get_all_posts(()) with the agent key it holds.
        var nonce = Data(count: 32)
        _ = nonce.withUnsafeMutableBytes { SecRandomCopyBytes(kSecRandomDefault, 32, $0.baseAddress!) }
        let expiresAt = Int64((Date().timeIntervalSince1970 + 300) * 1_000_000)
        let signed = try await runtime.signZomeCall(zomeCallUnsigned: ZomeCallParamsFfi(
            provenance: cell.agentPubKey, cellId: cell, zomeName: "posts", fnName: "get_all_posts",
            capSecret: nil, payload: Data([0xc0]), nonce: nonce, expiresAt: expiresAt))
        XCTAssertEqual(signed.signature.count, 64)

        // 5. Authenticate on the app websocket and send call_zome.
        var request = URLRequest(url: URL(string: "ws://127.0.0.1:\(auth.port)")!)
        request.setValue("http://localhost", forHTTPHeaderField: "Origin")
        let socket = URLSession.shared.webSocketTask(with: request)
        socket.resume()

        var authInner = MsgPack(); authInner.map(1); authInner.str("token"); authInner.intArray(auth.authentication.token)
        var authOuter = MsgPack(); authOuter.map(2)
        authOuter.str("type"); authOuter.str("authenticate"); authOuter.str("data"); authOuter.bin(authInner.data)
        try await socket.send(.data(authOuter.data))

        var call = MsgPack(); call.map(2)
        call.str("type"); call.str("call_zome"); call.str("value")
        call.map(2); call.str("bytes"); call.bin(signed.bytes); call.str("signature"); call.bin(signed.signature)
        var envelope = MsgPack(); envelope.map(3)
        envelope.str("type"); envelope.str("request"); envelope.str("id"); envelope.uint(1)
        envelope.str("data"); envelope.bin(call.data)
        try await socket.send(.data(envelope.data))

        // The response frame carries the inner {"type": ...} envelope as msgpack bin; look for its type tag.
        let deadline = Date().addingTimeInterval(120)
        var outcome = ""
        while Date() < deadline {
            guard case .data(let frame) = try await socket.receive() else { continue }
            if frame.range(of: Data("zome_called".utf8)) != nil { outcome = "zome_called"; break }
            if frame.range(of: Data("response".utf8)) != nil {
                outcome = String(decoding: frame, as: UTF8.self); break
            }
        }
        socket.cancel(with: .normalClosure, reason: nil)
        XCTAssertEqual(outcome, "zome_called")

        try await runtime.stop()
    }
}

/// Minimal msgpack writer: just what the app-websocket envelopes need.
struct MsgPack {
    private(set) var data = Data()

    mutating func map(_ count: Int) { data.append(0x80 | UInt8(count)) } // fixmap (< 16 entries)

    mutating func str(_ value: String) {
        let bytes = Array(value.utf8)
        if bytes.count < 32 { data.append(0xa0 | UInt8(bytes.count)) } else { data.append(0xd9); data.append(UInt8(bytes.count)) }
        data.append(contentsOf: bytes)
    }

    mutating func bin(_ value: Data) {
        data.append(0xc6) // bin 32
        withUnsafeBytes(of: UInt32(value.count).bigEndian) { data.append(contentsOf: $0) }
        data.append(value)
    }

    mutating func uint(_ value: UInt64) {
        data.append(0xcf)
        withUnsafeBytes(of: value.bigEndian) { data.append(contentsOf: $0) }
    }

    /// Vec<u8> app tokens travel as an array of integers, not as bin.
    mutating func intArray(_ value: Data) {
        data.append(0xdc) // array 16
        withUnsafeBytes(of: UInt16(value.count).bigEndian) { data.append(contentsOf: $0) }
        for byte in value { if byte < 0x80 { data.append(byte) } else { data.append(0xcc); data.append(byte) } }
    }
}
