use aze_enc::{ keygen, mask, remask, inter_unmask, final_unmask, CardCipher };
use aze_lib::accounts::create_basic_aze_player_account;
use aze_lib::client::{
    self, create_aze_client, AzeAccountTemplate, AzeClient, AzeGameMethods, AzeTransactionTemplate,
    SendCardTransactionData,
    GenPrivateKeyTransactionData,
    ShuffleCardTransactionData,
    RemaskTransactionData,
    SetCardsTransactionData,
};
use aze_lib::constants::{
    FIRST_PLAYER_INDEX, HIGHEST_BET, NO_OF_PLAYERS, PLAYER_INITIAL_BALANCE, SMALL_BUY_IN_AMOUNT,
    PLAYER_DATA_SLOT, DEFAULT_ACTION_TYPE
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
    Felt, FieldElement
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

    // Send note for shuffling and encryption
    let sender_account_id = game_account_id;
    let target_account_id = AccountId::try_from(player_account_ids[0]).unwrap();
    let shuffle_card_data = ShuffleCardTransactionData::new(   
        Asset::Fungible(fungible_asset),
        sender_account_id,
        target_account_id,
        [DEFAULT_ACTION_TYPE, player_account_ids[1], player_account_ids[2], player_account_ids[3]]
    );

    let transaction_template = AzeTransactionTemplate::ShuffleCard(shuffle_card_data);
    let txn_request = client
        .build_aze_shuffle_card_tx_request(transaction_template)
        .unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;
    println!("Note sent!");
    
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

    // keygen
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
        player_account.id(),
        faucet_account.id(),
        NoteType::Public,
    )
    .await;
    consume_notes(&mut client, player_account.id(), &[note]).await;

    let gen_key_data = GenPrivateKeyTransactionData::new(  
        Asset::Fungible(fungible_asset),
        player_account.id(),
        player_account.id(),
    );
    let transaction_template = AzeTransactionTemplate::GenKey(gen_key_data);
    let txn_request = client
        .build_aze_key_gen_tx_request(transaction_template)
        .unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;
    println!("Note sent!");
    let note_id = txn_request.expected_output_notes()[0].id();
    let note = client.get_input_note(note_id).unwrap();
    consume_notes(&mut client, player_account.id(), &[note.try_into().unwrap()]).await;

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

pub async fn enc_action(action_type: u64, account_id: AccountId, target_account: AccountId) {
    let mut client: AzeClient = create_aze_client();
    let (player_account, _) = client.get_account(account_id).unwrap();
    let mut cards: [[Felt; 4]; 52] = [[Felt::ZERO; 4]; 52];
    for (i, slot) in (1..53).enumerate() {
        let card_digest = player_account.storage().get_item(slot);
        cards[i] = card_digest.into();
    }

    // fund account
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
        player_account.id(),
        faucet_account.id(),
        NoteType::Public,
    )
    .await;
    consume_notes(&mut client, player_account.id(), &[note]).await;

    if action_type == 4 {
        // send set cards note to game account
        let set_cards_data = SetCardsTransactionData::new(   
            Asset::Fungible(fungible_asset),
            player_account.id(),
            target_account,
            &cards,
        );
        let transaction_template = AzeTransactionTemplate::SetCards(set_cards_data);
        let txn_request = client
            .build_aze_set_cards_tx_request(transaction_template)
            .unwrap();
        execute_tx_and_sync(&mut client, txn_request.clone()).await;
        println!("Note sent!");
        return
    }

    // send remask note
    let player_data = player_account.storage().get_item(PLAYER_DATA_SLOT).as_elements().to_vec();
    let remask_data = RemaskTransactionData::new(   
        Asset::Fungible(fungible_asset),
        account_id,
        target_account,
        &cards,
        [action_type + 1, player_data[1].as_int(), player_data[2].as_int(), player_data[3].as_int()]
    );
    let transaction_template = AzeTransactionTemplate::Remask(remask_data);
    let txn_request = client
        .build_aze_remask_tx_request(transaction_template)
        .unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;
    println!("Note sent!");
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