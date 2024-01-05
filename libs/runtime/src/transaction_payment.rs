impl pallet_transaction_payment::Config for crate::Runtime {
    type FeeMultiplierUpdate = pallet_transaction_payment::ConstFeeMultiplier<crate::FeeMultiplier>;
    type LengthToFee = frame_support::weights::IdentityFee<crate::Balance>;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<crate::Balances, ()>;
    type OperationalFeeMultiplier = crate::ConstU8<{ crate::constants::OPERATIONAL_FEE_MULTIPLIER }>;
    type RuntimeEvent = crate::RuntimeEvent;
    type WeightToFee = frame_support::weights::IdentityFee<crate::Balance>;
}
