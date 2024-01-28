#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 512.
#![recursion_limit = "512"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod assets;
pub mod aura;
pub mod balances;
pub mod constants;
pub mod contracts;
pub mod grandpa;
pub mod identity;
pub mod multisig;
pub mod opaque;
pub mod servicer_registry;
pub mod sudo;
pub mod system;
pub mod timestamp;
pub mod transaction_payment;
pub mod utility;
pub mod validator_set;

pub type AccountId = <<crate::Signature as sp_runtime::traits::Verify>::Signer as sp_runtime::traits::IdentifyAccount>::AccountId;
pub type Address = sp_runtime::MultiAddress<crate::AccountId, ()>;
pub type AuraId = sp_consensus_aura::sr25519::AuthorityId;
pub type Balance = u128;
pub type Block = sp_runtime::generic::Block<crate::Header, crate::UncheckedExtrinsic>;
pub type BlockNumber = u32;
pub type ConstBool<const T: bool> = frame_support::traits::ConstBool<T>;
pub type ConstU128<const T: u128> = frame_support::traits::ConstU128<T>;
pub type ConstU32<const T: u32> = frame_support::traits::ConstU32<T>;
pub type ConstU64<const T: u64> = frame_support::traits::ConstU64<T>;
pub type ConstU8<const T: u8> = frame_support::traits::ConstU8<T>;
pub type Executive = frame_executive::Executive<
    crate::Runtime,
    crate::Block,
    frame_system::ChainContext<Runtime>,
    crate::Runtime,
    crate::AllPalletsWithSystem,
>;
pub type EventRecord = frame_system::EventRecord<
    <crate::Runtime as frame_system::Config>::RuntimeEvent,
    <crate::Runtime as frame_system::Config>::Hash,
>;
pub type GrandpaId = sp_consensus_grandpa::AuthorityId;
pub type Hash = sp_core::H256;
pub type Header = sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;
pub type Nonce = u32;
pub type Signature = sp_runtime::MultiSignature;
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<crate::Runtime>,
    frame_system::CheckSpecVersion<crate::Runtime>,
    frame_system::CheckTxVersion<crate::Runtime>,
    frame_system::CheckGenesis<crate::Runtime>,
    frame_system::CheckEra<crate::Runtime>,
    frame_system::CheckNonce<crate::Runtime>,
    frame_system::CheckWeight<crate::Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<crate::Runtime>,
);
pub type SignedPayload = sp_runtime::generic::SignedPayload<crate::RuntimeCall, crate::SignedExtra>;
pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<
    crate::Address,
    crate::RuntimeCall,
    crate::Signature,
    crate::SignedExtra,
>;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{KeyOwnerProofSystem, Randomness, StorageInfo},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, ParityDbWeight, WEIGHT_REF_TIME_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

use sp_std::prelude::*;

#[sp_version::runtime_version]
pub const VERSION: sp_version::RuntimeVersion = sp_version::RuntimeVersion {
    impl_name: sp_runtime::create_runtime_str!("nagara-core"),
    spec_name: sp_runtime::create_runtime_str!("nagara-core"),
    apis: crate::RUNTIME_API_VERSIONS,
    authoring_version: 3,
    impl_version: 4,
    spec_version: 126,
    state_version: 2,
    transaction_version: 7,
};

#[cfg(feature = "std")]
pub fn native_version() -> sp_version::NativeVersion {
    sp_version::NativeVersion {
        runtime_version: crate::VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const ApprovalDeposit: Balance = constants::ERC20_APPROVAL_DEPOSIT;
    pub const AssetDeposit: Balance = constants::ERC20_CREATION_DEPOSIT;
    pub const BlockHashCount: crate::BlockNumber = 2400;
    pub const DepositBase: Balance = constants::MULTISIG_DEPOSIT_BASE;
    pub const DepositFactor: Balance = constants::MULTISIG_DEPOSIT_FACTOR;
    pub const MaxAdditionalFields: u32 = constants::IDENTITY_MAX_ADDITIONAL_FIELDS;
    pub const MetadataDepositBase: Balance = constants::ERC20_METADATA_DEPOSIT_PER_ITEM;
    pub const MetadataDepositPerByte: Balance = constants::ERC20_METADATA_DEPOSIT_PER_BYTE;
    pub const SS58Prefix: u16 = ss58_registry::Ss58AddressFormatRegistry::NagaraAccount as u16;
    pub const StringLimit: u32 = constants::ERC20_STRING_LIMIT;
    pub const Version: sp_version::RuntimeVersion = crate::VERSION;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::with_sensible_defaults(
            Weight::from_parts(crate::constants::BLOCKTIME_COMPUTE_BUDGET, u64::MAX),
            crate::constants::NORMAL_DISPATCH_RATIO,
        );
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength::max_with_normal_ratio(8 * 1024 * 1024, crate::constants::NORMAL_DISPATCH_RATIO);
    pub ContractSchedule: pallet_contracts::Schedule<Runtime> = Default::default();
    pub FeeMultiplier: pallet_transaction_payment::Multiplier = <pallet_transaction_payment::Multiplier as sp_runtime::traits::One>::one();
}

frame_support::construct_runtime!(
    pub enum Runtime {
        System: frame_system = 0,
        Timestamp: pallet_timestamp = 1,
        Balances: pallet_balances = 2,
        ValidatorSet: substrate_validator_set = 3,
        Session: pallet_session = 4,
        Aura: pallet_aura = 5,
        Grandpa: pallet_grandpa = 6,
        TransactionPayment: pallet_transaction_payment = 7,
        Sudo: pallet_sudo = 8,
        Utility: pallet_utility = 9,
        RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip = 10,
        Contracts: pallet_contracts = 11,
        Assets: pallet_assets = 12,
        Multisig: pallet_multisig = 13,
        Identity: pallet_identity = 14,
        ServicerRegistry: nagara_core_servicer_registry = 15,
    }
);

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    define_benchmarks!(
        [frame_benchmarking, BaselineBench::<crate::Runtime>]
        [frame_system, SystemBench::<crate::Runtime>]
        [pallet_timestamp, crate::Timestamp]
        [pallet_balances, crate::Balances]
        [pallet_sudo, crate::Sudo]
        [pallet_contracts, Contracts]
    );
}

sp_api::impl_runtime_apis! {
    impl sp_api::Core<Block> for crate::Runtime {
        fn version() -> sp_version::RuntimeVersion {
            crate::VERSION
        }

        fn execute_block(block: crate::Block) {
            crate::Executive::execute_block(block);
        }

        fn initialize_block(header: &<crate::Block as sp_runtime::traits::Block>::Header) {
            crate::Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<crate::Block> for crate::Runtime {
        fn metadata() -> sp_core::OpaqueMetadata {
            sp_core::OpaqueMetadata::new(crate::Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<sp_core::OpaqueMetadata> {
            crate::Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            crate::Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<crate::Block> for crate::Runtime {
        fn apply_extrinsic(extrinsic: <crate::Block as sp_runtime::traits::Block>::Extrinsic) -> sp_runtime::ApplyExtrinsicResult {
            crate::Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <crate::Block as sp_runtime::traits::Block>::Header {
            crate::Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<crate::Block as sp_runtime::traits::Block>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: crate::Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<crate::Block> for crate::Runtime {
        fn validate_transaction(
            source: sp_runtime::transaction_validity::TransactionSource,
            tx: <crate::Block as sp_runtime::traits::Block>::Extrinsic,
            block_hash: <crate::Block as sp_runtime::traits::Block>::Hash,
        ) -> sp_runtime::transaction_validity::TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<crate::Block> for crate::Runtime {
        fn offchain_worker(header: &<crate::Block as sp_runtime::traits::Block>::Header) {
            crate::Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<crate::Block, crate::AuraId> for crate::Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(crate::Aura::slot_duration())
        }

        fn authorities() -> Vec<crate::AuraId> {
            crate::Aura::authorities().into_inner()
        }
    }

    impl sp_session::SessionKeys<Block> for crate::Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            crate::opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
            crate::opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl sp_consensus_grandpa::GrandpaApi<crate::Block> for crate::Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            crate::Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            crate::Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <crate::Block as sp_runtime::traits::Block>::Hash,
                sp_runtime::traits::NumberFor<crate::Block>,
            >,
            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: sp_consensus_grandpa::SetId,
            _authority_id: crate::GrandpaId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<crate::Block, crate::AccountId, crate::Nonce> for crate::Runtime {
        fn account_nonce(account: crate::AccountId) -> crate::Nonce {
            crate::System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<crate::Block, crate::Balance> for crate::Runtime {
        fn query_info(
            uxt: <crate::Block as sp_runtime::traits::Block>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<crate::Balance> {
            crate::TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <crate::Block as sp_runtime::traits::Block>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<crate::Balance> {
            crate::TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: crate::Weight) -> crate::Balance {
            crate::TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> crate::Balance {
            crate::TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<crate::Block, crate::Balance, crate::RuntimeCall>
        for crate::Runtime
    {
        fn query_call_info(
            call: crate::RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::RuntimeDispatchInfo<crate::Balance> {
            crate::TransactionPayment::query_call_info(call, len)
        }

        fn query_call_fee_details(
            call: crate::RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<crate::Balance> {
            crate::TransactionPayment::query_call_fee_details(call, len)
        }

        fn query_weight_to_fee(weight: crate::Weight) -> crate::Balance {
            crate::TransactionPayment::weight_to_fee(weight)
        }

        fn query_length_to_fee(length: u32) -> crate::Balance {
            crate::TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_contracts::ContractsApi<crate::Block, crate::AccountId, crate::Balance, crate::BlockNumber, crate::Hash, crate::EventRecord>
        for crate::Runtime
    {
        fn call(
            origin: crate::AccountId,
            dest: crate::AccountId,
            value: crate::Balance,
            gas_limit: Option<Weight>,
            storage_deposit_limit: Option<crate::Balance>,
            input_data: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractExecResult<crate::Balance, crate::EventRecord> {
            let gas_limit = gas_limit.unwrap_or(crate::BlockWeights::get().max_block);
            crate::Contracts::bare_call(
                origin,
                dest,
                value,
                gas_limit,
                storage_deposit_limit,
                input_data,
                contracts::CONTRACTS_DEBUG_OUTPUT,
                contracts::CONTRACTS_EVENTS,
                pallet_contracts::Determinism::Enforced,
            )
        }

        fn instantiate(
            origin: crate::AccountId,
            value: crate::Balance,
            gas_limit: Option<Weight>,
            storage_deposit_limit: Option<crate::Balance>,
            code: pallet_contracts_primitives::Code<crate::Hash>,
            data: Vec<u8>,
            salt: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractInstantiateResult<crate::AccountId, crate::Balance, crate::EventRecord>
        {
            let gas_limit = gas_limit.unwrap_or(crate::BlockWeights::get().max_block);
            crate::Contracts::bare_instantiate(
                origin,
                value,
                gas_limit,
                storage_deposit_limit,
                code,
                data,
                salt,
                contracts::CONTRACTS_DEBUG_OUTPUT,
                contracts::CONTRACTS_EVENTS,
            )
        }

        fn upload_code(
            origin: crate::AccountId,
            code: Vec<u8>,
            storage_deposit_limit: Option<crate::Balance>,
            determinism: pallet_contracts::Determinism,
        ) -> pallet_contracts_primitives::CodeUploadResult<crate::Hash, crate::Balance>
        {
            Contracts::bare_upload_code(origin, code, storage_deposit_limit, determinism)
        }

        fn get_storage(
            address: crate::AccountId,
            key: Vec<u8>,
        ) -> pallet_contracts_primitives::GetStorageResult {
            Contracts::get_storage(address, key)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<crate::Block> for crate::Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = crate::AllPalletsWithSystem::storage_info();

            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};
            use sp_storage::TrackedStorageKey;
            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;

            impl frame_system_benchmarking::Config for crate::Runtime {}
            impl baseline::Config for crate::Runtime {}

            use frame_support::traits::WhitelistedStorageKeys;
            let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            Ok(batches)
        }
    }
}
