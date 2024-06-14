use aze_enc::{ keygen, mask, remask, inter_unmask, final_unmask, CardCipher };
use aze_lib::accounts::create_basic_aze_player_account;
use aze_lib::client::{
    self, create_aze_client, AzeAccountTemplate, AzeClient, AzeGameMethods, AzeTransactionTemplate,
    SendCardTransactionData,
    GenPrivateKeyTransactionData,
    ShuffleCardTransactionData
};
use aze_lib::constants::{
    FIRST_PLAYER_INDEX, HIGHEST_BET, NO_OF_PLAYERS, PLAYER_INITIAL_BALANCE, SMALL_BUY_IN_AMOUNT,
};
use aze_lib::executor::execute_tx_and_sync;
use aze_lib::notes::{consume_notes, mint_note};
use aze_lib::storage::GameStorageSlotData;
use aze_types::accounts::{
    AccountCreationError, AccountCreationRequest, AccountCreationResponse,
    PlayerAccountCreationRequest, PlayerAccountCreationResponse,
};
use miden_client::client::{
    accounts::{AccountStorageMode, AccountTemplate},
    transactions::transaction_request::TransactionTemplate,
};
use miden_lib::AuthScheme;
use miden_objects::{
    accounts::{AccountId, AuthSecretKey},
    assets::TokenSymbol,
    assets::{Asset, FungibleAsset},
    crypto::dsa::rpo_falcon512::{PublicKey, SecretKey},
    notes::NoteType,
};
use tokio::time::{ sleep, Duration };
use ecgfp5::scalar::Scalar;

pub async fn create_aze_game_account(
    player_account_ids: Vec<u64>,
    small_blind: u8,
    buy_in: u64,
) -> Result<AccountId, AccountCreationError> {
    let mut client: AzeClient = create_aze_client();
    let slot_data = GameStorageSlotData::new(
        small_blind,
        buy_in as u8,
        NO_OF_PLAYERS,
        FIRST_PLAYER_INDEX,
        HIGHEST_BET,
        PLAYER_INITIAL_BALANCE,
    );

    let (faucet_account, _) = client
        .new_account(AccountTemplate::FungibleFaucet {
            token_symbol: TokenSymbol::new("MATIC").unwrap(),
            decimals: 8,
            max_supply: 1_000_000_000,
            storage_mode: AccountStorageMode::Local,
        })
        .unwrap();

    let faucet_account_id = faucet_account.id();
    let fungible_asset = FungibleAsset::new(faucet_account_id, SMALL_BUY_IN_AMOUNT as u64).unwrap();

    let (game_account, _) = client
        .new_game_account(
            AzeAccountTemplate::GameAccount {
                mutable_code: false,
                storage_mode: AccountStorageMode::Local,
            },
            Some(slot_data),
        )
        .unwrap();

    let game_account_id = game_account.id();

    println!("Account created: {:?}", game_account_id);

    println!("First client consuming note");
    let note = mint_note(
        &mut client,
        game_account_id,
        faucet_account_id,
        NoteType::Public,
    )
    .await;
    println!("Minted note");
    consume_notes(&mut client, game_account_id, &[note]).await;

    // here we will send note for shuffling and encryption
    let sender_account_id = game_account_id;
    let target_account_id = create_aze_player_account("player".to_string()).await.unwrap();
    let shuffle_card_data = ShuffleCardTransactionData::new(   // for now as shuffling is not ready
        Asset::Fungible(fungible_asset),
        sender_account_id,
        target_account_id,
    );

    let transaction_template = AzeTransactionTemplate::ShuffleCard(shuffle_card_data);
    let txn_request = client
        .build_aze_shuffle_card_tx_request(transaction_template)
        .unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;
    println!("Note sent!");
    let note_id = txn_request.expected_output_notes()[0].id();
    let note = client.get_input_note(note_id).unwrap();
    consume_notes(&mut client, target_account_id, &[note.try_into().unwrap()]).await;

    let (player_account, _) = client.get_account(target_account_id).unwrap();
    for slot in 1..57 {
        println!("Slot {}: {:?}", slot, player_account.storage().get_item(slot));
    }
    
    Ok(game_account_id)
}

pub async fn create_aze_player_account(
    identifier: String,
) -> Result<AccountId, AccountCreationError> {
    use miden_objects::accounts::AccountType;
    let key_pair = SecretKey::new();
    let pub_key: PublicKey = key_pair.public_key();
    let auth_scheme: AuthScheme = AuthScheme::RpoFalcon512 { pub_key };

    // initial seed to create the wallet account
    let init_seed: [u8; 32] = [
        95, 113, 209, 94, 84, 105, 250, 242, 223, 203, 216, 124, 22, 159, 14, 132, 215, 85, 183,
        204, 149, 90, 166, 68, 100, 73, 106, 168, 125, 237, 138, 16,
    ];

    let (player_account, seed) = create_basic_aze_player_account(
        init_seed,
        auth_scheme,
        AccountType::RegularAccountImmutableCode,
    )
    .unwrap();

    let mut client: AzeClient = create_aze_client();
    client.insert_account(
        &player_account,
        Some(seed),
        &AuthSecretKey::RpoFalcon512(key_pair),
    );

    Ok(player_account.id())
}

pub async fn consume_game_notes(account_id: AccountId) {
    let mut client: AzeClient = create_aze_client();
    client.sync_state().await.unwrap();
    let account = client.get_account(account_id).unwrap();
    let consumable_notes = client.get_consumable_notes(Some(account_id)).unwrap();
    println!("Consumable notes: {:?}", consumable_notes.len());

    for consumable_note in consumable_notes {
        let tx_template = TransactionTemplate::ConsumeNotes(account_id, vec![consumable_note.note.id()]);
        let tx_request = client.build_transaction_request(tx_template).unwrap();
        execute_tx_and_sync(&mut client, tx_request).await;
        println!("Waiting...");
        sleep(Duration::from_secs(5)).await;
    }
}

pub async fn enc_action(account_id: AccountId) {
    // check account storage for a slot
    // if slot == 1, mask. if slot == 2/3/4, remask

    // 2 mock cards
    let mut card_points = vec![]; // 2 cards for now
    card_points.push(keygen(11)); // for now 
    card_points.push(keygen(12));
    let pub_key_agg = keygen(123); // for now
    let masking_factor: u32 = 123;// for now

    if account_id == AccountId::try_from(1085128954612151006).unwrap() { // for now
        // Mask
        let cipher_card_1 = mask(pub_key_agg, card_points[0], masking_factor);
        println!("Cipher card 1 --> {:?}\n", cipher_card_1);
        let cipher_card_2 = mask(pub_key_agg, card_points[1], masking_factor);
        println!("Cipher card 2 --> {:?}", cipher_card_2);
        return
    }

    // 2 mock masked cards
    let cipher_card_1 = mask(pub_key_agg, card_points[0], masking_factor);
    let cipher_card_2 = mask(pub_key_agg, card_points[1], masking_factor);

    // remask
    let remasked_card_1 = remask(pub_key_agg, cipher_card_1, masking_factor);
    println!("Remasked card 1 --> {:?}\n", remasked_card_1);
    let remasked_card_2 = remask(pub_key_agg, cipher_card_2, masking_factor);
    println!("Remasked card 2 --> {:?}", remasked_card_2);

    // if slot == 2/3, send note to next player
    // if slot == 4, do nothing as it is the last player
    send_note(account_id, account_id).await;
}

pub async fn dec_action(account_id: AccountId) {
    let players_ids = [317826241474458840, 359196095275670923, 359196095275670923, 359196095275670923];
    let pub_key_agg = keygen(123); // for now
    let masking_factor: u32 = 123;// for now
    let masked_cards = get_mock_cards(); // 2 cards for now
    // check account storage for a slot
    // if slot == 1, do final-unmask else inter-unmask
    if account_id == AccountId::try_from(players_ids[0]).unwrap() {
        // read masked cards from account storage
        // 2 mock masked cards for now
        let final_unmasked_card_1 = final_unmask(pub_key_agg, masked_cards[0].clone(), masking_factor);
        println!("Final unmasked card 1 --> {:?}\n", final_unmasked_card_1);
        let final_unmasked_card_2 = final_unmask(pub_key_agg, masked_cards[1].clone(), masking_factor);
        println!("Final unmasked card 2 --> {:?}", final_unmasked_card_2);
        return
    }

    // read masked cards from account storage
    let unmasked_card_1 = inter_unmask(pub_key_agg, masked_cards[0].clone(), masking_factor);
    println!("Inter unmasked card 1 --> {:?}\n", unmasked_card_1);
    let unmasked_card_2 = inter_unmask(pub_key_agg, masked_cards[1].clone(), masking_factor);
    println!("Inter unmasked card 2 --> {:?}", unmasked_card_2);

    // send note back to the player with the inter unmasked card for further decryption
    let target_account_id = AccountId::try_from(players_ids[0]).unwrap();
    send_note(account_id, target_account_id).await;
}

fn get_mock_cards() -> Vec<CardCipher> {
    // 2 mock cards
    let pub_key_agg = keygen(123); // for now
    let masking_factor: u32 = 123;// for now
    let cipher_card_1 = mask(pub_key_agg, keygen(11), masking_factor);
    let cipher_card_2 = mask(pub_key_agg, keygen(12), masking_factor);
    let mut masked_cards = vec![]; // 2 cards for now
    masked_cards.push(cipher_card_1);
    masked_cards.push(cipher_card_2);
    masked_cards
}

pub async fn send_note(sender_account_id: AccountId, target_account_id: AccountId) {
    let mut client: AzeClient = create_aze_client();
    let (faucet_account, _) = client
        .new_account(AccountTemplate::FungibleFaucet {
            token_symbol: TokenSymbol::new("MATIC").unwrap(),
            decimals: 8,
            max_supply: 1_000_000_000,
            storage_mode: AccountStorageMode::Local,
        })
        .unwrap();
    let fungible_asset = FungibleAsset::new(faucet_account.id(), SMALL_BUY_IN_AMOUNT as u64).unwrap();

    let note = mint_note(
        &mut client,
        sender_account_id,
        faucet_account.id(),
        NoteType::Public,
    )
    .await;
    println!("Minted note");
    consume_notes(&mut client, sender_account_id, &[note]).await;

    let gen_key_data = GenPrivateKeyTransactionData::new(   // for now as shuffling is not ready
        Asset::Fungible(fungible_asset),
        sender_account_id,
        target_account_id,
    );
    let transaction_template = AzeTransactionTemplate::GenKey(gen_key_data);
    let txn_request = client
        .build_aze_key_gen_tx_request(transaction_template)
        .unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;
    println!("Note sent!");
}

pub async fn p2p_unmask_flow(sender_account_id: AccountId) -> Result<(), String> {
    let players_ids = [359196095275670923, 359196095275670923, 317826241474458840];

    for player_id in players_ids.iter() {
        let receiver_account_id = AccountId::try_from(*player_id).unwrap();
        send_note(sender_account_id, receiver_account_id).await;
    }

    Ok(())
}