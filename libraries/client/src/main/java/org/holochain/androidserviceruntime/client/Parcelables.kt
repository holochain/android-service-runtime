package org.holochain.androidserviceruntime.client

import android.os.Parcelable
import kotlinx.parcelize.Parcelize
import kotlinx.parcelize.TypeParceler

@Parcelize
@TypeParceler<InstallAppPayloadFfi, InstallAppPayloadFfiParceler>
data class InstallAppPayloadFfiParcel(
    val inner: InstallAppPayloadFfi,
) : Parcelable

@Parcelize
@TypeParceler<AppInfoFfi, AppInfoFfiParceler>
data class AppInfoFfiParcel(
    val inner: AppInfoFfi,
) : Parcelable

@Parcelize
@TypeParceler<PausedAppReasonFfi, PausedAppReasonFfiParceler>
data class PausedAppReasonFfiParcel(
    val inner: PausedAppReasonFfi,
) : Parcelable

@Parcelize
@TypeParceler<DisabledAppReasonFfi, DisabledAppReasonFfiParceler>
data class DisabledAppReasonFfiParcel(
    val inner: DisabledAppReasonFfi,
) : Parcelable

@Parcelize
@TypeParceler<RoleSettingsFfi, RoleSettingsFfiParceler>
data class RoleSettingsFfiParcel(
    val inner: RoleSettingsFfi,
) : Parcelable

@Parcelize
@TypeParceler<CellInfoFfi, CellInfoFfiParceler>
data class CellInfoFfiParcel(
    val inner: CellInfoFfi,
) : Parcelable

@Parcelize
@TypeParceler<CellIdFfi, CellIdFfiParceler>
data class CellIdFfiParcel(
    val inner: CellIdFfi,
) : Parcelable

@Parcelize
@TypeParceler<ProvisionedCellFfi, ProvisionedCellFfiParceler>
data class ProvisionedCellFfiParcel(
    val inner: ProvisionedCellFfi,
) : Parcelable

@Parcelize
@TypeParceler<ClonedCellFfi, ClonedCellFfiParceler>
data class ClonedCellFfiParcel(
    val inner: ClonedCellFfi,
) : Parcelable

@Parcelize
@TypeParceler<StemCellFfi, StemCellFfiParceler>
data class StemCellFfiParcel(
    val inner: StemCellFfi,
) : Parcelable

@Parcelize
@TypeParceler<AppInfoStatusFfi, AppInfoStatusFfiParceler>
data class AppInfoStatusFfiParcel(
    val inner: AppInfoStatusFfi,
) : Parcelable

@Parcelize
@TypeParceler<DnaModifiersFfi, DnaModifiersFfiParceler>
data class DnaModifiersFfiParcel(
    val inner: DnaModifiersFfi,
) : Parcelable

@Parcelize
@TypeParceler<DnaModifiersOptFfi, DnaModifiersOptFfiParceler>
data class DnaModifiersOptFfiParcel(
    val inner: DnaModifiersOptFfi,
) : Parcelable

@Parcelize
@TypeParceler<AppAuthenticationTokenIssuedFfi, AppAuthenticationTokenIssuedFfiParceler>
data class AppAuthenticationTokenIssuedFfiParcel(
    val inner: AppAuthenticationTokenIssuedFfi,
) : Parcelable

@Parcelize
@TypeParceler<AppAuthFfi, AppAuthFfiParceler>
data class AppAuthFfiParcel(
    val inner: AppAuthFfi,
) : Parcelable

@Parcelize
@TypeParceler<ZomeCallParamsFfi, ZomeCallParamsFfiParceler>
data class ZomeCallParamsFfiParcel(
    val inner: ZomeCallParamsFfi,
) : Parcelable

@Parcelize
@TypeParceler<ZomeCallParamsSignedFfi, ZomeCallParamsSignedFfiParceler>
data class ZomeCallParamsSignedFfiParcel(
    val inner: ZomeCallParamsSignedFfi,
) : Parcelable

@Parcelize
@TypeParceler<AdminBinderUnauthorizedException, AdminBinderUnauthorizedExceptionParceler>
data class AdminBinderUnauthorizedExceptionParcel(
    var inner: AdminBinderUnauthorizedException,
) : Parcelable

@Parcelize
@TypeParceler<AppBinderUnauthorizedException, AppBinderUnauthorizedExceptionParceler>
data class AppBinderUnauthorizedExceptionParcel(
    var inner: AppBinderUnauthorizedException,
) : Parcelable

@Parcelize
@TypeParceler<RuntimeNetworkConfigFfi, RuntimeNetworkConfigFfiParceler>
data class RuntimeNetworkConfigFfiParcel(
    var inner: RuntimeNetworkConfigFfi,
) : Parcelable
