impl pallet_identity::Config for crate::Runtime {
    type BasicDeposit = crate::ConstU128<{ crate::constants::IDENTITY_BASIC_DEPOSIT }>;
    type Currency = crate::Balances;
    type FieldDeposit = crate::ConstU128<{ crate::constants::IDENTITY_BYTE_DEPOOSIT }>;
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type MaxAdditionalFields = crate::MaxAdditionalFields;
    type MaxRegistrars = crate::ConstU32<{ crate::constants::MAX_AUTHORITIES as u32 }>;
    type MaxSubAccounts = crate::ConstU32<{ crate::constants::IDENTITY_MAX_SUB_ACCOUNTS }>;
    type RegistrarOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type RuntimeEvent = crate::RuntimeEvent;
    type Slashed = ();
    type SubAccountDeposit = crate::ConstU128<{ crate::constants::IDENTITY_SUB_ACCOUNT_DEPOSIT }>;
    type WeightInfo = pallet_identity::weights::SubstrateWeight<crate::Runtime>;
}
