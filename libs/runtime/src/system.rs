impl frame_system::Config for crate::Runtime {
    type AccountData = pallet_balances::AccountData<crate::Balance>;
    type AccountId = crate::AccountId;
    type BaseCallFilter = frame_support::traits::Everything;
    type Block = crate::Block;
    type BlockHashCount = crate::BlockHashCount;
    type BlockLength = crate::BlockLength;
    type BlockWeights = crate::BlockWeights;
    type DbWeight = frame_support::weights::constants::RocksDbWeight;
    type Hash = crate::Hash;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type Lookup = sp_runtime::traits::AccountIdLookup<crate::AccountId, ()>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type Nonce = crate::Nonce;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type PalletInfo = crate::PalletInfo;
    type RuntimeCall = crate::RuntimeCall;
    type RuntimeEvent = crate::RuntimeEvent;
    type RuntimeOrigin = crate::RuntimeOrigin;
    type SS58Prefix = crate::SS58Prefix;
    type SystemWeightInfo = ();
    type Version = crate::Version;
}
