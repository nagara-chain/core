impl pallet_timestamp::Config for crate::Runtime {
    type MinimumPeriod = crate::ConstU64<{ crate::constants::CONSENSUS_SLOT_DURATION / 2 }>;
    type Moment = u64;
    type OnTimestampSet = crate::Aura;
    type WeightInfo = ();
}
