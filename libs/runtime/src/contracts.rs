pub const CONTRACTS_DEBUG_OUTPUT: pallet_contracts::DebugInfo =
    pallet_contracts::DebugInfo::UnsafeDebug;
pub const CONTRACTS_EVENTS: pallet_contracts::CollectEvents =
    pallet_contracts::CollectEvents::UnsafeCollect;

impl pallet_insecure_randomness_collective_flip::Config for crate::Runtime {}

impl pallet_contracts::Config for crate::Runtime {
    type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
    type CallFilter = frame_support::traits::Everything;
    type CallStack = [pallet_contracts::Frame<Self>; 5];
    type ChainExtension = ();
    type Currency = crate::Balances;
    type DefaultDepositLimit = crate::ConstU128<{ u128::MAX }>;
    type DepositPerByte = crate::ConstU128<{ crate::constants::DEPOSIT_PER_BYTE }>;
    type DepositPerItem = crate::ConstU128<{ crate::constants::DEPOSIT_PER_ITEM }>;
    type MaxCodeLen = crate::ConstU32<{ 123 * 1024 }>;
    type MaxDebugBufferLen = crate::ConstU32<{ 2 * 1024 * 1024 }>;
    type MaxStorageKeyLen = crate::ConstU32<128>;
    type Migrations = ();
    type Randomness = crate::RandomnessCollectiveFlip;
    type RuntimeCall = crate::RuntimeCall;
    type RuntimeEvent = crate::RuntimeEvent;
    type Schedule = crate::ContractSchedule;
    type Time = crate::Timestamp;
    type UnsafeUnstableInterface = crate::ConstBool<true>;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
}
