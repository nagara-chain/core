impl nagara_registry_servicers::Config for crate::Runtime {
    type BindingFee = crate::ConstU128<{ 2 * crate::constants::TOKEN }>;
    type Currency = crate::Balances;
    type RuntimeEvent = crate::RuntimeEvent;
}
