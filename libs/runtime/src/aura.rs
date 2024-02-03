impl pallet_aura::Config for super::Runtime {
    type AllowMultipleBlocksPerSlot =
        crate::ConstBool<{ crate::constants::ALLOW_MULTIPLE_BLOCKS_PER_SLOT }>;
    type AuthorityId = crate::AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = crate::ConstU32<{ crate::constants::MAX_AUTHORITIES as u32 }>;
    #[cfg(feature = "experimental")]
    type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Runtime>;
}
