impl pallet_balances::Config for crate::Runtime {
    type AccountStore = crate::System;
    type Balance = crate::Balance;
    type DustRemoval = ();
    type ExistentialDeposit = crate::ConstU128<{ crate::constants::EXISTENTIAL_DEPOSIT }>;
    type FreezeIdentifier = crate::RuntimeFreezeReason;
    type MaxFreezes = crate::ConstU32<{ crate::constants::MAX_ACCOUNT_FREEZES }>;
    type MaxHolds = crate::ConstU32<{ crate::constants::MAX_ACCOUNT_HOLDS }>;
    type MaxLocks = crate::ConstU32<{ crate::constants::MAX_ACCOUNT_LOCKS }>;
    type MaxReserves = crate::ConstU32<{ crate::constants::MAX_ACCOUNT_RESERVES }>;
    type ReserveIdentifier = [u8; 24];
    type RuntimeEvent = crate::RuntimeEvent;
    type RuntimeHoldReason = crate::RuntimeHoldReason;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<crate::Runtime>;
}
