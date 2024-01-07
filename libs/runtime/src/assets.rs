impl pallet_assets::Config for crate::Runtime {
    type ApprovalDeposit = crate::ApprovalDeposit;
    type AssetAccountDeposit = crate::ConstU128<{ crate::constants::TOKEN }>;
    type AssetDeposit = crate::AssetDeposit;
    type AssetId = u32;
    type AssetIdParameter = codec::Compact<u32>;
    type Balance = crate::Balance;
    type CallbackHandle = ();
    type CreateOrigin =
        frame_support::traits::AsEnsureOriginWithArg<frame_system::EnsureSigned<crate::AccountId>>;
    type Currency = crate::Balances;
    type Extra = ();
    type ForceOrigin = frame_system::EnsureRoot<crate::AccountId>;
    type Freezer = ();
    type MetadataDepositBase = crate::MetadataDepositBase;
    type MetadataDepositPerByte = crate::MetadataDepositPerByte;
    type RemoveItemsLimit = crate::ConstU32<1024>;
    type RuntimeEvent = crate::RuntimeEvent;
    type StringLimit = crate::StringLimit;
    type WeightInfo = pallet_assets::weights::SubstrateWeight<crate::Runtime>;
}
