impl nagara_registry_servicers::Config for crate::Runtime {
    type BindingDepositAmount = crate::ConstU128<{ crate::constants::ATTESTER_BINDING_HOLD }>;
    type Currency = crate::Balances;
    type MaxMediators = crate::ConstU32<{ crate::constants::MAX_MEDIATORS }>;
    type RegistrationFeeAmount = crate::ConstU128<{ crate::constants::SERVICER_REGISTRATION_FEE }>;
    type RuntimeEvent = crate::RuntimeEvent;
    type RuntimeHoldReason = crate::RuntimeHoldReason;
}
