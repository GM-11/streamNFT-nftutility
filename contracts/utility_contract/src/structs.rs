use soroban_sdk::{contracterror, contracttype, Address, String, Vec};

#[repr(u32)]
#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExpiryType {
    None = 0,
    TimeBased = 1,
    DateBased = 2,
}

#[repr(u32)]
#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UtilityType {
    NftBenefit = 0,
    Giveaway = 1,
}

#[repr(u32)]
#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UsageType {
    Unlimited = 0,
    Limited = 1,
}

#[repr(u32)]
#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EligibleType {
    None = 0,
    NftOwner = 1,
    Whitelist = 2,
}

#[repr(u32)]
#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    All = 0,
    Raffle = 1,
}

#[repr(u32)]
#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Receipt {
    None = 0,
    MintToken = 1,
    External = 2,
    HTSToken = 3,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq)]
pub struct Utility {
    pub provider: Address,
    pub expiry: u64,
    pub usage: u64,
    pub offer_expiry: u64,
    pub partner: Address,
    pub utility_uri: String,
    pub expiry_type: ExpiryType,
    pub usage_type: UsageType,
    pub raffle: Raffle,
    pub selection: Selection,
    pub reward: Reward,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq)]
pub struct Reward {
    pub receipt: Receipt,
    pub token_addresses: Vec<Address>,
    pub total_amount: u64,
    pub amount_per_win: u64,
    pub no_of_winners: u64,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq)]
pub struct Raffle {
    pub start_time: u64,
    pub ended: bool,
    // pub winners_merkle: Bytes,
}

#[contracterror]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum UtilityError {
    InvalidTime = 1,
    InsufficientBalance = 2,
    InvalidExpiry = 3,
    NotAuthorized = 4,
    RaffleExpired = 5,
    InvalidRaffleSelection = 6,
    RaffleAlreadyEnded = 7,
    RaffleNotEnded = 8,
    AllRewardsClaimed = 9,
    AlreadyClaimed = 10,
    InvalidReceiptType = 11,
    UtilityNotFound = 12,
    UtilityExpired = 13,
    UsageExceeded = 14,
}
