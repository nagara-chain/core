impl pallet_insecure_randomness_collective_flip::Config for crate::Runtime {}

impl pallet_contracts::Config for crate::Runtime {
    type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
    type CallFilter = frame_support::traits::Everything;
    type CallStack = [pallet_contracts::Frame<Self>; 5];
    type ChainExtension = ();
    type CodeHashLockupDepositPercent = crate::CodeHashLockupDepositPercent;
    type Currency = crate::Balances;
    type Debug = ();
    type DefaultDepositLimit = crate::ConstU128<{ crate::constants::get_fee(1024, 1024 * 1024) }>;
    type DepositPerByte = crate::ConstU128<{ crate::constants::DEPOSIT_PER_BYTE }>;
    type DepositPerItem = crate::ConstU128<{ crate::constants::DEPOSIT_PER_ITEM }>;
    type Environment = ();
    type MaxCodeLen = crate::ConstU32<{ 123 * 1024 }>;
    type MaxDebugBufferLen = crate::ConstU32<{ 2 * 1024 * 1024 }>;
    type MaxDelegateDependencies = crate::ConstU32<32>;
    type MaxStorageKeyLen = crate::ConstU32<128>;
    type Migrations = ();
    type Randomness = crate::RandomnessCollectiveFlip;
    type RuntimeCall = crate::RuntimeCall;
    type RuntimeEvent = crate::RuntimeEvent;
    type RuntimeHoldReason = crate::RuntimeHoldReason;
    type Schedule = crate::ContractSchedule;
    type Time = crate::Timestamp;
    type UnsafeUnstableInterface = crate::ConstBool<true>;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
}
