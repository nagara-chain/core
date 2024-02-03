impl pallet_grandpa::Config for crate::Runtime {
    type EquivocationReportSystem = ();
    type KeyOwnerProof = sp_core::Void;
    type MaxAuthorities = crate::ConstU32<{ crate::constants::MAX_AUTHORITIES as u32 }>;
    type MaxSetIdSessionEntries = crate::ConstU64<{ crate::constants::MAX_SET_ID_SESSION_ENTRIES }>;
    type RuntimeEvent = crate::RuntimeEvent;
    type WeightInfo = ();
}
