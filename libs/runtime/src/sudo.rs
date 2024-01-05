impl pallet_sudo::Config for crate::Runtime {
    type RuntimeCall = crate::RuntimeCall;
    type RuntimeEvent = crate::RuntimeEvent;
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<crate::Runtime>;
}
