impl pallet_utility::Config for crate::Runtime {
    type PalletsOrigin = crate::OriginCaller;
    type RuntimeCall = crate::RuntimeCall;
    type RuntimeEvent = crate::RuntimeEvent;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<crate::Runtime>;
}
