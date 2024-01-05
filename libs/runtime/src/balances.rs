impl pallet_balances::Config for crate::Runtime {
    type AccountStore = crate::System;
    type Balance = crate::Balance;
    type DustRemoval = ();
    type ExistentialDeposit = crate::ConstU128<{ crate::constants::EXISTENTIAL_DEPOSIT }>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type MaxHolds = ();
    type MaxLocks = crate::ConstU32<{ crate::constants::MAX_ACCOUNT_LOCKS }>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeEvent = crate::RuntimeEvent;
    type RuntimeHoldReason = ();
    type WeightInfo = pallet_balances::weights::SubstrateWeight<crate::Runtime>;
}
