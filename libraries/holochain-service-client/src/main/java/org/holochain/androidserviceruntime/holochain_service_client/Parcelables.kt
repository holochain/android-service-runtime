package org.holochain.androidserviceruntime.holochain_service_client

import android.os.Parcelable
import kotlinx.parcelize.Parcelize
import kotlinx.parcelize.TypeParceler

@Parcelize
@TypeParceler<InstallAppPayloadFfi, InstallAppPayloadFfiParceler>
data class InstallAppPayloadFfiParcel(val inner: InstallAppPayloadFfi) : Parcelable

@Parcelize
@TypeParceler<AppInfoFfi, AppInfoFfiParceler>
data class AppInfoFfiParcel(val inner: AppInfoFfi) : Parcelable

@Parcelize
@TypeParceler<PausedAppReasonFfi, PausedAppReasonFfiParceler>
data class PausedAppReasonFfiParcel(val inner: PausedAppReasonFfi) : Parcelable

@Parcelize
@TypeParceler<DisabledAppReasonFfi, DisabledAppReasonFfiParceler>
data class DisabledAppReasonFfiParcel(val inner: DisabledAppReasonFfi) : Parcelable

@Parcelize
@TypeParceler<RoleSettingsFfi, RoleSettingsFfiParceler>
data class RoleSettingsFfiParcel(val inner: RoleSettingsFfi) : Parcelable

@Parcelize
@TypeParceler<CellInfoFfi, CellInfoFfiParceler>
data class CellInfoFfiParcel(val inner: CellInfoFfi) : Parcelable

@Parcelize
@TypeParceler<CellIdFfi, CellIdFfiParceler>
data class CellIdFfiParcel(val inner: CellIdFfi) : Parcelable

@Parcelize
@TypeParceler<ProvisionedCellFfi, ProvisionedCellFfiParceler>
data class ProvisionedCellFfiParcel(val inner: ProvisionedCellFfi) : Parcelable

@Parcelize
@TypeParceler<ClonedCellFfi, ClonedCellFfiParceler>
data class ClonedCellFfiParcel(val inner: ClonedCellFfi) : Parcelable

@Parcelize
@TypeParceler<StemCellFfi, StemCellFfiParceler>
data class StemCellFfiParcel(val inner: StemCellFfi) : Parcelable

@Parcelize
@TypeParceler<AppInfoStatusFfi, AppInfoStatusFfiParceler>
data class AppInfoStatusFfiParcel(val inner: AppInfoStatusFfi) : Parcelable

@Parcelize
@TypeParceler<DnaModifiersFfi, DnaModifiersFfiParceler>
data class DnaModifiersFfiParcel(val inner: DnaModifiersFfi) : Parcelable

@Parcelize
@TypeParceler<DnaModifiersOptFfi, DnaModifiersOptFfiParceler>
data class DnaModifiersOptFfiParcel(val inner: DnaModifiersOptFfi) : Parcelable

@Parcelize
@TypeParceler<DurationFfi, DurationFfiParceler>
data class DurationFfiParcel(val inner: DurationFfi) : Parcelable

@Parcelize
@TypeParceler<AppAuthenticationTokenIssuedFfi, AppAuthenticationTokenIssuedFfiParceler>
data class AppAuthenticationTokenIssuedFfiParcel(val inner: AppAuthenticationTokenIssuedFfi) :
    Parcelable

@Parcelize
@TypeParceler<AppAuthFfi, AppAuthFfiParceler>
data class AppAuthFfiParcel(val inner: AppAuthFfi) : Parcelable

@Parcelize
@TypeParceler<ZomeCallUnsignedFfi, ZomeCallUnsignedFfiParceler>
data class ZomeCallUnsignedFfiParcel(val inner: ZomeCallUnsignedFfi) : Parcelable

@Parcelize
@TypeParceler<ZomeCallFfi, ZomeCallFfiParceler>
data class ZomeCallFfiParcel(val inner: ZomeCallFfi) : Parcelable
