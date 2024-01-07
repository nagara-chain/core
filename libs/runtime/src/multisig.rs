impl pallet_multisig::Config for crate::Runtime {
    type Currency = crate::Balances;
    type DepositBase = crate::DepositBase;
    type DepositFactor = crate::DepositFactor;
    type MaxSignatories = crate::ConstU32<{ crate::constants::MULTISIG_MAX_PARTICIPANTS }>;
    type RuntimeCall = crate::RuntimeCall;
    type RuntimeEvent = crate::RuntimeEvent;
    type WeightInfo = pallet_multisig::weights::SubstrateWeight<crate::Runtime>;
}
