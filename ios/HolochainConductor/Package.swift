// swift-tools-version:5.9
// Swift package for the Holochain conductor runtime on iOS (wasmi interpreter build).
// CI copies HolochainConductor.xcframework and the generated UniFFI Swift files in before building;
// see .github/workflows/ios-conductor.yml.
import PackageDescription

let package = Package(
    name: "HolochainConductor",
    platforms: [.iOS(.v15)],
    products: [
        .library(name: "HolochainConductor", targets: ["HolochainConductor"]),
    ],
    targets: [
        .binaryTarget(name: "HolochainConductorFFI", path: "HolochainConductor.xcframework"),
        .target(
            name: "HolochainConductor",
            dependencies: ["HolochainConductorFFI"],
            linkerSettings: [
                .linkedFramework("Security"),
                .linkedFramework("SystemConfiguration"),
                // iroh watches network changes through Network.framework (nw_path_monitor_*).
                .linkedFramework("Network"),
                .linkedFramework("CoreFoundation"),
                .linkedLibrary("resolv"),
            ]
        ),
        .testTarget(
            name: "HolochainConductorTests",
            dependencies: ["HolochainConductor"],
            resources: [.copy("Resources/forum.happ")]
        ),
    ]
)
