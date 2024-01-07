// region: consensus

pub const ALLOW_MULTIPLE_BLOCKS_PER_SLOT: bool = false;
pub const AUTHORITY_SESSION_OFFSET: u32 = 0;
pub const AUTHORITY_SESSION_PERIOD: u32 = 2 * MINUTES;
pub const CONSENSUS_SLOT_DURATION: u64 = BLOCKTIME_MS;
pub const MAX_AUTHORITIES: u32 = 16;
pub const MAX_NOMINATORS: u32 = 0;
pub const MAX_SET_ID_SESSION_ENTRIES: u64 = 0;
pub const MIN_AUTHORITIES: u32 = 1;
pub const NORMAL_DISPATCH_RATIO: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(90);

// endregion

// region: time const

pub const BLOCKTIME_COMPUTE_BUDGET: u64 = BLOCKTIME_WEIGHT * 2 / 3;
pub const BLOCKTIME_MS: u64 = BLOCKTIME * 1_000;
pub const BLOCKTIME_WEIGHT: u64 = BLOCKTIME * WEIGHT_TIME_S;
pub const BLOCKTIME: u64 = 3;
pub const DAYS: crate::BlockNumber = HOURS * 24;
pub const HOURS: crate::BlockNumber = MINUTES * 60;
pub const MILLISECS_PER_BLOCK: u64 = BLOCKTIME_MS;
pub const MINUTES: crate::BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as crate::BlockNumber);
pub const WEIGHT_TIME_MS: u64 = WEIGHT_TIME_US * 1_000;
pub const WEIGHT_TIME_NS: u64 = WEIGHT_TIME * 1_000;
pub const WEIGHT_TIME_S: u64 = WEIGHT_TIME_MS * 1_000;
pub const WEIGHT_TIME_US: u64 = WEIGHT_TIME_NS * 1_000;
pub const WEIGHT_TIME: u64 = 1;

// endregion

// region: balance const

pub const DEPOSIT_PER_BYTE: crate::Balance = get_fee(0, 1);
pub const DEPOSIT_PER_ITEM: crate::Balance = get_fee(1, 0);
pub const ERC20_APPROVAL_DEPOSIT: crate::Balance = 32 * TOKEN;
pub const ERC20_CREATION_DEPOSIT: crate::Balance = 64 * TOKEN;
pub const ERC20_METADATA_DEPOSIT_PER_BYTE: crate::Balance = DEPOSIT_PER_BYTE;
pub const ERC20_METADATA_DEPOSIT_PER_ITEM: crate::Balance = DEPOSIT_PER_ITEM;
pub const ERC20_STRING_LIMIT: u32 = 256;
pub const EXISTENTIAL_DEPOSIT: crate::Balance = TOKEN_MICROS;
pub const MAX_ACCOUNT_LOCKS: u32 = 50;
pub const MIN_GAS_FEE: crate::Balance = get_fee(1, 0);
pub const MULTISIG_DEPOSIT_BASE: crate::Balance = get_fee(1, MULTISIG_SIZE);
pub const MULTISIG_DEPOSIT_FACTOR: crate::Balance = get_fee(0, MULTISIG_KEY_SIZE);
pub const MULTISIG_EXT_SIZE: u32 = 4 + 4 + 16 + 32;
pub const MULTISIG_KEY_SIZE: u32 = 32;
pub const MULTISIG_MAX_PARTICIPANTS: u32 = 32;
pub const MULTISIG_SIZE: u32 = MULTISIG_KEY_SIZE + MULTISIG_EXT_SIZE;
pub const OPERATIONAL_FEE_MULTIPLIER: u8 = 5;
pub const REF_TIME_GAS_FEE_DIVIDER: u64 = 16 * 1024;
pub const TOKEN_CENTS: crate::Balance = TOKEN / 100;
pub const TOKEN_DIGIT: u32 = 9;
pub const TOKEN_MICROS: crate::Balance = TOKEN_MILLIS / 1_000;
pub const TOKEN_MILLICENTS: crate::Balance = TOKEN_CENTS / 1_000;
pub const TOKEN_MILLIS: crate::Balance = TOKEN / 1_000;
pub const TOKEN_NANOS: crate::Balance = TOKEN_MICROS / 1_000;
pub const TOKEN_PREFIX: u16 = ss58_registry::Ss58AddressFormatRegistry::NagaraAccount as u16;
pub const TOKEN_REGISTRY: ss58_registry::TokenRegistry = ss58_registry::TokenRegistry::Ngr;
pub const TOKEN: crate::Balance = 10_u128.pow(TOKEN_DIGIT);
pub const TRANSACTION_BYTE_FEE: crate::Balance = get_fee(0, 1);

// endregion

// region: helper functions (const)

pub const fn get_fee(items_count: u32, bytes_length: u32) -> crate::Balance {
    const TOKEN_PER_BYTE: crate::Balance = 14 * TOKEN_NANOS;
    const TOKEN_PER_ITEM: crate::Balance = TOKEN_MICROS;

    let items = items_count as crate::Balance;
    let bytes = bytes_length as crate::Balance;
    let item_cost = items * TOKEN_PER_ITEM;
    let byte_cost = bytes * TOKEN_PER_BYTE;

    item_cost + byte_cost
}

// endregion
