#![no_std]

use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, token, Address, Env, Map, String,
    Symbol, Vec,
};
pub mod structs;

use soroban_token_sdk::TokenUtils;
use structs::{ExpiryType, Receipt, Selection, UsageType, Utility, UtilityError};

const ADMIN: Symbol = symbol_short!("ADMIN");
const U_STORAGE: Symbol = symbol_short!("U_STORAGE");
const ELIGIBLE: Symbol = symbol_short!("ELIGIBLE");
const TOKEN_UTILITY: Symbol = symbol_short!("TOKEN_U");

mod test;

#[contract]
pub struct UtilityContract;

#[contractimpl]
impl UtilityContract {
    pub fn setup_config(env: Env, admin: String) -> String {
        // let admin = Address::
        // if env
        //     .storage()
        //     .persistent()
        //     .get::<Symbol, String>(&ADMIN)
        //     .is_none()
        // {
        env.storage().persistent().set(&ADMIN, &admin);
        let utilities: Vec<Utility> = Vec::new(&env);
        env.storage().persistent().set(&U_STORAGE, &utilities);
        // }?
        admin

        // env.storage().persistent().get(&ADMIN, &admin)
    }

    // pub fn init_token(env: Env, addr: Address) {
    //     TokenUtils::new(&env)
    //         .events()
    //         .mint(addr.clone(), addr, 2 as i128);
    // }

    pub fn get_config(env: Env) -> String {
        let addr = env.storage().persistent().get::<Symbol, String>(&ADMIN);

        match addr {
            Some(a) => a,
            None => String::from_str(&env, &"No Admin Set"),
        }
    }

    pub fn check_nft_ownership(env: Env, token_address: Address, user: Address) -> bool {
        let token_client = token::StellarAssetClient::new(&env, &token_address);
        token_client.authorized(&user)
    }

    pub fn create_utility(env: Env, u: Utility, sender: Address) {
        let mut u = u;
        u.provider = sender.clone();

        // if u.selection == Selection::Raffle {
        //     if u.raffle.start_time < env.ledger().timestamp()
        //         || u.raffle.start_time > u.offer_expiry
        //     {
        //         panic_with_error!(env, UtilityError::InvalidTime);
        //     }
        // }

        // if u.reward.receipt == Receipt::External {
        //     for addr in u.reward.token_addresses.clone() {
        //         let token_client = token::TokenClient::new(&env, &addr);
        //         u.provider.require_auth();
        //         if token_client.balance(&u.provider) < 1 {
        //             panic_with_error!(&env, UtilityError::InsufficientBalance);
        //         }
        //         token_client.transfer_from(
        //             &u.provider,
        //             &sender,
        //             &env.current_contract_address(),
        //             &(1 as i128),
        //         );
        //     }
        // }

        // if u.offer_expiry < env.ledger().timestamp() {
        //     panic_with_error!(&env, UtilityError::InvalidExpiry);
        // }
        // if u.usage_type == UsageType::Limited && u.usage < 1 {
        //     panic_with_error!(&env, UtilityError::InvalidExpiry);
        // }

        // // fetch nft collections

        // if (u.expiry_type == ExpiryType::TimeBased || u.expiry_type == ExpiryType::DateBased)
        //     && u.expiry < env.ledger().timestamp()
        // {
        //     panic_with_error!(&env, UtilityError::InvalidExpiry);
        // }

        if env.storage().persistent().has(&U_STORAGE) {
            let mut utilities = env
                .storage()
                .persistent()
                .get::<Symbol, Vec<Utility>>(&U_STORAGE)
                .unwrap_or(Vec::new(&env));

            utilities.push_back(u);

            env.storage().persistent().set(&U_STORAGE, &utilities);
        } else {
            let mut utilities: Vec<Utility> = Vec::new(&env);
            utilities.push_back(u);
            env.storage().persistent().set(&U_STORAGE, &utilities);
        }
    }

    pub fn join_raffle(env: Env, utility_id: u64, sender: Address, user: Address) {
        if env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Utility>>(&U_STORAGE)
            .is_none()
        {
            panic_with_error!(&env, UtilityError::UtilityNotFound);
        }
        let utility = env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Utility>>(&U_STORAGE)
            .unwrap();
        let u = utility.get(utility_id as u32).unwrap();

        if u.provider != sender.clone() {
            panic_with_error!(&env, UtilityError::NotAuthorized);
        }

        if u.offer_expiry < env.ledger().timestamp() {
            panic_with_error!(&env, UtilityError::RaffleExpired);
        }

        if u.selection != Selection::Raffle || u.raffle.ended == true {
            panic_with_error!(&env, UtilityError::InvalidRaffleSelection);
        }

        env.events()
            .publish((symbol_short!("raffle"), symbol_short!("joined")), user);
    }

    pub fn end_raffle(env: Env, utility_id: u64, sender: Address) {
        if env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Utility>>(&U_STORAGE)
            .is_none()
        {
            panic_with_error!(&env, UtilityError::UtilityNotFound);
        }
        let utility = env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Utility>>(&U_STORAGE)
            .unwrap();
        let mut u = utility.get(utility_id as u32).unwrap();

        if u.raffle.ended == true {
            panic_with_error!(&env, UtilityError::RaffleAlreadyEnded)
        }

        if u.provider != sender.clone() {
            panic_with_error!(&env, UtilityError::NotAuthorized)
        }

        if u.selection != Selection::Raffle {
            panic_with_error!(&env, UtilityError::InvalidRaffleSelection)
        }

        if env.ledger().timestamp() < u.offer_expiry {
            panic_with_error!(&env, UtilityError::RaffleNotEnded)
        }

        u.raffle.ended = true;

        env.events().publish(
            (symbol_short!("raffle"), symbol_short!("ended")),
            symbol_short!("ended"),
        );
    }
    pub fn claim_reward(env: Env, utility_id: u64, user: Address, sender: Address) {
        // Check if the sender is the admin
        let admin = env
            .storage()
            .persistent()
            .get::<Symbol, Address>(&ADMIN)
            .unwrap();
        if sender != admin {
            panic_with_error!(&env, UtilityError::NotAuthorized)
        }

        let mut utilities = env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Utility>>(&U_STORAGE)
            .unwrap();
        let mut utility = utilities.get(utility_id as u32).unwrap();

        if utility.reward.total_amount < utility.reward.amount_per_win {
            panic_with_error!(&env, UtilityError::AllRewardsClaimed)
        }

        utility.reward.total_amount -= utility.reward.amount_per_win;

        if utility.selection == Selection::Raffle {
            if !utility.raffle.ended {
                panic_with_error!(&env, UtilityError::RaffleNotEnded)
            }
        }

        // Check if already claimed
        let claim_key = (symbol_short!("claimed"), utility_id, user.clone());
        if env
            .storage()
            .persistent()
            .get::<_, bool>(&claim_key)
            .unwrap_or(false)
        {
            panic_with_error!(env, UtilityError::AlreadyClaimed)
        }

        // Mark as claimed
        env.storage().persistent().set(&claim_key, &true);

        let token_address_to_transfer: Address = utility.reward.token_addresses.get(0).unwrap();

        match utility.reward.receipt {
            Receipt::MintToken => {
                let token_client = token::StellarAssetClient::new(&env, &token_address_to_transfer);
                token_client.mint(&user, &(utility.reward.amount_per_win as i128));
            }
            Receipt::External => {
                let token_client = token::TokenClient::new(&env, &token_address_to_transfer);
                token_client.transfer_from(
                    &env.current_contract_address(),
                    &env.current_contract_address(),
                    &user,
                    &(utility.reward.amount_per_win as i128),
                );
            }

            Receipt::None => {
                panic_with_error!(&env, UtilityError::InvalidReceiptType)
            }
            Receipt::HTSToken => {
                panic_with_error!(&env, UtilityError::InvalidReceiptType)
            }
        }

        // Update the utility in storage
        utilities.set(utility_id as u32, utility);
        env.storage().persistent().set(&U_STORAGE, &utilities);

        // Emit event
        env.events().publish(
            (symbol_short!("claim"), symbol_short!("reward")),
            (utility_id, user, token_address_to_transfer),
        );
    }
    pub fn mark_eligible(env: Env, utility_id: u64, user: Address, sender: Address) {
        // Check if the sender is the admin
        let admin = env
            .storage()
            .persistent()
            .get::<Symbol, Address>(&ADMIN)
            .unwrap();
        if sender != admin {
            panic_with_error!(&env, UtilityError::NotAuthorized)
        }

        let mut eligible = env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Map<Address, u64>>>(&ELIGIBLE)
            .unwrap_or(Vec::new(&env));
        let mut utility_map = Map::new(&env);
        utility_map.set(user, utility_id);
        eligible.push_back(utility_map);
        env.storage().persistent().set(&ELIGIBLE, &eligible);
    }
    pub fn get_utility(env: Env, utility_id: u64) -> Utility {
        let utilities = env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Utility>>(&U_STORAGE)
            .unwrap_or_else(|| panic_with_error!(&env, UtilityError::UtilityNotFound));

        utilities
            .get(utility_id as u32)
            .unwrap_or_else(|| panic_with_error!(&env, UtilityError::UtilityNotFound))
    }

    pub fn get_token_utility(env: Env, token_address: Address) -> Utility {
        let token_utilities = env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Map<Address, Utility>>>(&TOKEN_UTILITY)
            .unwrap();

        for m in token_utilities {
            if m.contains_key(token_address.clone()) {
                return m.get(token_address).unwrap();
            }
        }

        panic_with_error!(&env, UtilityError::UtilityNotFound);
    }

    pub fn claim_utitlity_on_nft(
        env: Env,
        token_address: Address,
        utility_id: u64,
        user: Address,
        sender: Address,
    ) {
        let admin = env
            .storage()
            .persistent()
            .get::<Symbol, Address>(&ADMIN)
            .unwrap();
        if sender != admin {
            panic!("Not authorized");
        }

        let utility = env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Utility>>(&U_STORAGE)
            .unwrap();
        let u = utility.get(utility_id as u32).unwrap();

        if u.provider != sender.clone() {
            panic_with_error!(&env, UtilityError::NotAuthorized)
        }

        let mut token_utility = Self::get_token_utility(env.clone(), token_address.clone());

        if u.selection == Selection::Raffle {
            // check merkle proof
        }

        if token_utility.usage_type == UsageType::Limited {
            token_utility.usage = u.usage;
        }

        if u.expiry_type == ExpiryType::TimeBased {
            token_utility.expiry = u.expiry + env.ledger().timestamp();
        } else if u.expiry_type == ExpiryType::DateBased {
            token_utility.expiry = u.expiry;
        }

        env.events().publish(
            (symbol_short!("utility"), symbol_short!("claimed")),
            (token_address, user, utility_id),
        );
    }

    pub fn redeem_utility_on_nft(env: Env, token_address: Address, utility_id: u64, user: Address) {
        let owner = Self::check_nft_ownership(env.clone(), token_address.clone(), user.clone());

        if !owner {
            panic_with_error!(&env, UtilityError::NotAuthorized)
        }

        let utility = env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Utility>>(&U_STORAGE)
            .unwrap();
        let mut u = utility.get(utility_id as u32).unwrap();

        let token_utility = Self::get_token_utility(env.clone(), token_address.clone());

        if u.expiry_type == ExpiryType::None && token_utility.expiry < env.ledger().timestamp() {
            panic_with_error!(&env, UtilityError::UtilityExpired)
        }

        if token_utility.usage_type == UsageType::Limited {
            // token_utility.usage = u.usage;
            if u.usage == 0 {
                panic_with_error!(&env, UtilityError::UsageExceeded)
            } else {
                u.usage -= 1;
            }
        }

        env.events().publish(
            (symbol_short!("utility"), symbol_short!("redeemed")),
            (token_address, user, utility_id),
        );
    }

    pub fn check_utility(env: Env, token_address: Address, utility_id: u64) -> bool {
        let mut response = true;
        if let Some(utility) = env
            .storage()
            .persistent()
            .get::<Symbol, Vec<Utility>>(&U_STORAGE)
        {
            let u = utility.get(utility_id as u32);

            match u {
                Some(u) => {
                    let token_utility = Self::get_token_utility(env.clone(), token_address.clone());

                    if u.expiry_type == ExpiryType::None {
                        if token_utility.expiry < env.ledger().timestamp() {
                            response = false;
                        }
                    }

                    if u.usage_type != UsageType::Limited {
                        if token_utility.usage == 0 {
                            response = false;
                        }
                    }

                    response
                }
                None => {
                    panic_with_error!(&env, UtilityError::UtilityNotFound)
                }
            }
        } else {
            // panic_with_error!(&env, UtilityError::UtilityNotFound)
            return false;
        }
    }
}
