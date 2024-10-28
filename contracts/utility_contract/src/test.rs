#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Address, Env};
use structs::{Raffle, Reward};

#[test]
fn test_setup_config() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UtilityContract);
    let client = UtilityContractClient::new(&env, &contract_id);

    let admin = &String::from_str(
        &env,
        "GAIIG7PGQYFG4AZK36YWKE3GPH6GXU4BIDZZASMP5DSVZL2DZ75W34JS",
    );

    client.setup_config(&admin);

    assert_eq!(&client.get_config(), admin);
}

#[test]
pub fn test_config() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UtilityContract);
    let client = UtilityContractClient::new(&env, &contract_id);

    let config = client.get_config();

    assert_eq!(config, String::from_str(&env, &"No Admin Set"))
}

#[test]
pub fn test_nft_ownership() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UtilityContract);
    let client = UtilityContractClient::new(&env, &contract_id);

    let token_address_string = String::from_str(
        &env,
        "GBT4VVTDPCNA45MNWX5G6LUTLIEENSTUHDVXO2AQHAZ24KUZUPLPGJZH",
    );

    let user_address_string = String::from_str(
        &env,
        "GDQOSTXBBGFO5N26DOGBIDDOLSSYJPA7DNCQ2V46B5D24JYIH2H3B54H",
    );

    let result = client.check_nft_ownership(
        &Address::from_string(&token_address_string),
        &Address::from_string(&user_address_string),
    );

    assert_eq!(result, true);
}

#[test]
pub fn test_create_utility() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UtilityContract);
    let client = UtilityContractClient::new(&env, &contract_id);
    let provider = Address::from_string(&String::from_str(
        &env,
        "GAIIG7PGQYFG4AZK36YWKE3GPH6GXU4BIDZZASMP5DSVZL2DZ75W34JS",
    ));

    let partner = Address::from_string(&String::from_str(
        &env,
        "GAIIG7PGQYFG4AZK36YWKE3GPH6GXU4BIDZZASMP5DSVZL2DZ75W34JS",
    ));

    let utility = Utility {
        provider: provider.clone(),
        expiry: env.ledger().timestamp() + 1000,
        usage: 10,
        offer_expiry: env.ledger().timestamp() + 2000,
        partner,
        utility_uri: String::from_str(&env, "https://example.com"),
        expiry_type: ExpiryType::TimeBased,
        usage_type: UsageType::Limited,
        raffle: Raffle {
            start_time: env.ledger().timestamp() + 500,
            ended: false,
            // winners_merkle: Bytes::from_array(&env, &[0u8; 32]),
        },
        selection: Selection::All,
        reward: Reward {
            receipt: Receipt::None,
            token_addresses: vec![&env],
            total_amount: 1000,
            amount_per_win: 100,
            no_of_winners: 10,
        },
    };

    client.create_utility(&utility, &provider);

    // Check if the utility was created
    let created_utility = client.get_utility(&0);
    assert_eq!(created_utility.provider, provider);
}

// #[test]
// fn test_init_token() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, UtilityContract);
//     let client = UtilityContractClient::new(&env, &contract_id);

//     let token_admin = Address::generate(&env);
//     let token_address = create_token_contract(&env, &token_admin);

//     client.init_token(&token_address);

//     // You might want to check the token balance here if possible
// }

// #[test]
// fn test_check_nft_ownership() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, UtilityContract);
//     let client = UtilityContractClient::new(&env, &contract_id);

//     let token_admin = Address::generate(&env);
//     let token_address = create_token_contract(&env, &token_admin);
//     let user = Address::generate(&env);

//     // Assume the user doesn't own the NFT initially
//     assert!(!client.check_nft_ownership(&token_address, &user));

//     // You would need to mint an NFT to the user here
//     // Then check again
//     // assert!(client.check_nft_ownership(&token_address, &user));
// }

// #[test]
// fn test_create_utility() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, UtilityContract);
//     let client = UtilityContractClient::new(&env, &contract_id);

//     let admin = Address::generate(&env);
//     client.setup_config(&admin);

//     let provider = Address::generate(&env);
//     let partner = Address::generate(&env);

//     let utility = Utility {
//         provider: provider.clone(),
//         expiry: env.ledger().timestamp() + 1000,
//         usage: 10,
//         offer_expiry: env.ledger().timestamp() + 2000,
//         partner,
//         utility_uri: "https://example.com".into_val(&env),
//         expiry_type: ExpiryType::TimeBased,
//         usage_type: UsageType::Limited,
//         raffle: Raffle {
//             start_time: env.ledger().timestamp() + 500,
//             ended: false,
//             winners_merkle: Bytes::from_array(&env, &[0u8; 32]),
//         },
//         selection: Selection::Raffle,
//         reward: Reward {
//             receipt: Receipt::None,
//             token_addresses: vec![&env],
//             total_amount: 1000,
//             amount_per_win: 100,
//             no_of_winners: 10,
//         },
//     };

//     client.create_utility(&utility, &provider);

//     // Check if the utility was created
//     let created_utility = client.get_utility(&0);
//     assert_eq!(created_utility.provider, provider);
// }

// #[test]
// #[should_panic(expected = "Invalid expiry value")]
// fn test_create_utility_invalid_expiry() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, UtilityContract);
//     let client = UtilityContractClient::new(&env, &contract_id);

//     let admin = Address::generate(&env);
//     client.setup_config(&admin);

//     let provider = Address::generate(&env);
//     let partner = Address::generate(&env);

//     let utility = Utility {
//         provider: provider.clone(),
//         expiry: env.ledger().timestamp() - 1000, // Invalid expiry
//         usage: 10,
//         offer_expiry: env.ledger().timestamp() + 2000,
//         partner,
//         utility_uri: "https://example.com".into_val(&env),
//         expiry_type: ExpiryType::TimeBased,
//         usage_type: UsageType::Limited,
//         raffle: Raffle {
//             start_time: env.ledger().timestamp() + 500,
//             ended: false,
//             winners_merkle: Bytes::from_array(&env, &[0u8; 32]),
//         },
//         selection: Selection::Raffle,
//         reward: Reward {
//             receipt: Receipt::None,
//             token_addresses: vec![&env],
//             total_amount: 1000,
//             amount_per_win: 100,
//             no_of_winners: 10,
//         },
//     };

//     client.create_utility(&utility, &provider);
// }

// #[test]
// fn test_join_raffle() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, UtilityContract);
//     let client = UtilityContractClient::new(&env, &contract_id);

//     let admin = Address::generate(&env);
//     client.setup_config(&admin);

//     let provider = Address::generate(&env);
//     let user = Address::generate(&env);

//     // Create a utility with a raffle
//     let utility = Utility {
//         provider: provider.clone(),
//         expiry: env.ledger().timestamp() + 1000,
//         usage: 10,
//         offer_expiry: env.ledger().timestamp() + 2000,
//         partner: Address::generate(&env),
//         utility_uri: "https://example.com".into_val(&env),
//         expiry_type: ExpiryType::TimeBased,
//         usage_type: UsageType::Limited,
//         raffle: Raffle {
//             start_time: env.ledger().timestamp() + 500,
//             ended: false,
//             winners_merkle: Bytes::from_array(&env, &[0u8; 32]),
//         },
//         selection: Selection::Raffle,
//         reward: Reward {
//             receipt: Receipt::None,
//             token_addresses: vec![&env],
//             total_amount: 1000,
//             amount_per_win: 100,
//             no_of_winners: 10,
//         },
//     };

//     client.create_utility(&utility, &provider);

//     // Join the raffle
//     client.join_raffle(&0, &provider, &user);

//     // Check for the event (this depends on how you've implemented event checking)
//     // assert!(env.events().includes(...));
// }

// #[test]
// fn test_end_raffle() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, UtilityContract);
//     let client = UtilityContractClient::new(&env, &contract_id);

//     let admin = Address::generate(&env);
//     client.setup_config(&admin);

//     let provider = Address::generate(&env);

//     // Create a utility with a raffle
//     let utility = Utility {
//         provider: provider.clone(),
//         expiry: env.ledger().timestamp() + 1000,
//         usage: 10,
//         offer_expiry: env.ledger().timestamp() + 2000,
//         partner: Address::generate(&env),
//         utility_uri: "https://example.com".into_val(&env),
//         expiry_type: ExpiryType::TimeBased,
//         usage_type: UsageType::Limited,
//         raffle: Raffle {
//             start_time: env.ledger().timestamp() + 500,
//             ended: false,
//             winners_merkle: Bytes::from_array(&env, &[0u8; 32]),
//         },
//         selection: Selection::Raffle,
//         reward: Reward {
//             receipt: Receipt::None,
//             token_addresses: vec![&env],
//             total_amount: 1000,
//             amount_per_win: 100,
//             no_of_winners: 10,
//         },
//     };

//     client.create_utility(&utility, &provider);

//     // Fast forward time
//     // env.ledger().set_timestamp(env.ledger().timestamp() + 3000);

//     // End the raffle
//     let merkle_root = Bytes::from_array(&env, &[1u8; 32]);
//     client.end_raffle(&0, &provider, &merkle_root);

//     // Check if the raffle ended
//     let ended_utility = client.get_utility(&0);
//     assert!(ended_utility.raffle.ended);
// }
