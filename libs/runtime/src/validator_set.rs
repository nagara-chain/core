impl substrate_validator_set::Config for crate::Runtime {
    type AddRemoveOrigin = frame_system::EnsureRoot<crate::AccountId>;
    type MinAuthorities = crate::ConstU32<{ crate::constants::MIN_AUTHORITIES }>;
    type RuntimeEvent = crate::RuntimeEvent;
    type WeightInfo = substrate_validator_set::weights::SubstrateWeight<crate::Runtime>;
}

impl pallet_session::Config for crate::Runtime {
    type Keys = crate::opaque::SessionKeys;
    type NextSessionRotation = pallet_session::PeriodicSessions<
        crate::ConstU32<{ crate::constants::AUTHORITY_SESSION_PERIOD }>,
        crate::ConstU32<{ crate::constants::AUTHORITY_SESSION_OFFSET }>,
    >;
    type RuntimeEvent = crate::RuntimeEvent;
    type SessionHandler = <crate::opaque::SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
    type SessionManager = crate::ValidatorSet;
    type ShouldEndSession = pallet_session::PeriodicSessions<
        crate::ConstU32<{ crate::constants::AUTHORITY_SESSION_PERIOD }>,
        crate::ConstU32<{ crate::constants::AUTHORITY_SESSION_OFFSET }>,
    >;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = substrate_validator_set::ValidatorOf<Self>;
    type WeightInfo = ();
}
