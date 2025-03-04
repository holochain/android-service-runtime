package com.plugin.holochain_service

import kotlinx.parcelize.Parcelize
import kotlinx.parcelize.RawValue
import android.os.Parcelable
import android.os.SharedMemory
import uniffi.holochain_conductor_runtime_ffi.CellInfoFfi

@Parcelize
data class InstallAppRequestAidl(
  val appId: String,
  val appBundleSharedMemory: SharedMemory,
  val membraneProofs: Map<String, ByteArray>,
  val agent: ByteArray?,
  val networkSeed: String?,
): Parcelable

@Parcelize
data class AppInfoStatusFfiAidl(
  val type: String,
// val reason: String,
): Parcelable

@Parcelize
data class AppInfoFfiAidl(
  val installedAppId: String,
  val cellInfo: @RawValue Map<String, List<CellInfoFfi>>,
  val status: AppInfoStatusFfiAidl,
  val agentPubKey: ByteArray,
): Parcelable

@Parcelize
data class AppWebsocketAuthFfiAidl(
  val appId: String,
  val port: Int,
  val token: UByteArray,
): Parcelable

@Parcelize
data class SignZomeCallRequestAidl(
  val provenance: ByteArray,
  val cellIdDnaHash: ByteArray,
  val cellIdAgentPubKey: ByteArray,
  val zomeName: String,
  val fnName: String,
  val capSecret: ByteArray?,
  val payload: ByteArray,
  val nonce: ByteArray,
  val expiresAt: Long,
): Parcelable

@Parcelize
data class ZomeCallSignedFfiAidl(
  val cellIdDnaHash: ByteArray,
  val cellIdAgentPubKey: ByteArray,
  val zomeName: String,
  val fnName: String,
  val payload: ByteArray,
  val capSecret: ByteArray?,
  val provenance: ByteArray,
  val signature: ByteArray,
  val nonce: ByteArray,
  val expiresAt: Long,
) : Parcelable