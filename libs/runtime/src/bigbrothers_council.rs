impl nagara_council_bigbrothers::Config for crate::Runtime {
    type BurnAddress = crate::ChainBurnAddress;
    type Currency = crate::Balances;
    type InitialMinimumTransactionFee =
        crate::ConstU128<{ crate::constants::INITIAL_MINIMUM_TRANSACTION_FEE }>;
    type InitialWeightToFeeDivider =
        crate::ConstU64<{ crate::constants::INITIAL_WEIGHT_TO_FEE_DIVIDER }>;
    type InitialWeightToFeeMultiplier =
        crate::ConstU64<{ crate::constants::INITIAL_WEIGHT_TO_FEE_MULTIPLIER }>;
    type MaxMembers = crate::ConstU32<{ crate::constants::MAX_AUTHORITIES as u32 }>;
    type RegistrationDepositAmount = crate::ConstU128<{ 200 * crate::constants::TOKEN }>;
    type RuntimeEvent = crate::RuntimeEvent;
    type RuntimeHoldReason = crate::RuntimeHoldReason;
}
