pub const UDT_LEN: usize = 16;
pub const CKB_UNITS: u64 = 100_000_000;
pub const PLEDGE: u64 = 10000 * CKB_UNITS;
pub const XT_CELL_CAPACITY: u64 = 200 * CKB_UNITS;

pub const COLLATERAL_PERCENT: u8 = 150;

pub const TX_PROOF_DIFFICULTY_FACTOR: u8 = 1;
// default value is 2/1000
pub const SIGNER_FEE_RATE: (u128, u128) = (2, 1000);
pub const SUDT_CODE_HASH: [u8; 32] = [
    225, 227, 84, 214, 214, 67, 173, 66, 114, 77, 64, 150, 126, 51, 73, 132, 83, 78, 3, 103, 64,
    92, 90, 228, 42, 157, 125, 99, 215, 125, 244, 25,
];

// since
pub const LOCK_TYPE_FLAG: u64 = 1 << 63;
pub const SINCE_TYPE_TIMESTAMP: u64 = 0x4000_0000_0000_0000;
pub const METRIC_TYPE_FLAG_MASK: u64 = 0x6000_0000_0000_0000;
pub const VALUE_MASK: u64 = 0x00ff_ffff_ffff_ffff;
pub const REMAIN_FLAGS_BITS: u64 = 0x1f00_0000_0000_0000;

// 24 * 3600 means 1 day, the unit is second
pub const SINCE_SIGNER_TIMEOUT: u64 = LOCK_TYPE_FLAG | SINCE_TYPE_TIMESTAMP | 24 * 3600;
pub const SINCE_AT_TERM_REDEEM: u64 = LOCK_TYPE_FLAG | SINCE_TYPE_TIMESTAMP | 180 * 24 * 3600;

pub const SINCE_WITHDRAW_PLEDGE_COLLATERAL: u64 = LOCK_TYPE_FLAG | SINCE_TYPE_TIMESTAMP | 24 * 3600;
pub const SINCE_WITHDRAW_PLEDGE: u64 = LOCK_TYPE_FLAG | SINCE_TYPE_TIMESTAMP | 7 * 24 * 3600;

// LIQUIDATION_COLLATERAL_PERCENT means min liquidation threshold of collateral/lot_amount
pub const LIQUIDATION_COLLATERAL_PERCENT: u8 = 115;

// PRE_UNDERCOLLATERAL_RATE represents max collateral rate that signer can start redeeming
pub const PRE_UNDERCOLLATERAL_RATE: u8 = 120;

/*
If auction tx executed after AUCTION_MAX_TIME, the bidder need to pay the
lowest price that equals 'lot_amount/collateral' to get the collateral.
The time unit is second.
*/
pub const AUCTION_MAX_TIME: u64 = 3 * 24 * 3600;
pub const AUCTION_INIT_PERCENT: u8 = 67;

pub const BTC_ADDRESS_PREFIX: &str = "bcrt";
