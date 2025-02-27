package com.holochain_apps.holochain_service_common

import kotlinx.parcelize.Parcelize
import kotlinx.parcelize.RawValue
import android.os.Parcelable
import android.os.SharedMemory

@Parcelize
data class InstallAppRequestAidl(
  val appId: String,
  val appBundleSharedMemory: SharedMemory,
  val roleSettings: Map<String, @RawValue RoleSettingsFfi>,
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
  val cellInfo: Map<String, List<@RawValue CellInfoFfi>>,
  val status: AppInfoStatusFfiAidl,
  val agentPubKey: ByteArray,
): Parcelable

@Parcelize
data class AppWebsocketAuthFfiAidl @OptIn(ExperimentalUnsignedTypes::class) constructor(
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