use aze_lib::client::{
    create_aze_client,
    AzeAccountTemplate,
    AzeClient,
    AzeGameMethods,
    AzeTransactionTemplate,
    PlayBetTransactionData,
    PlayCallTransactionData,
    PlayCheckTransactionData,
    PlayFoldTransactionData,
    PlayRaiseTransactionData,
};
use aze_lib::constants::{ SMALL_BUY_IN_AMOUNT, HIGHEST_BET_SLOT };
use aze_lib::executor::execute_tx_and_sync;
use aze_lib::storage::GameStorageSlotData;
use aze_types::actions::{ GameActionError, GameActionResponse };
use miden_client::client::{
    accounts::{ AccountStorageMode, AccountTemplate },
    transactions::transaction_request::TransactionTemplate,
};
use miden_objects::{ 
    assets::{ Asset, FungibleAsset, TokenSymbol }, 
    notes::NoteType,
    accounts::AccountId
};

pub async fn raise(
    player_id: u64,
    game_id: u64,
    amount: Option<u8>
) -> Result<GameActionResponse, String> {
    let mut client: AzeClient = create_aze_client();
    let player_account_id = AccountId::try_from(player_id).unwrap();
    let game_account_id = AccountId::try_from(game_id).unwrap();

    let (faucet_account, _) = client
        .new_account(AccountTemplate::FungibleFaucet {
            token_symbol: TokenSymbol::new("MATIC").unwrap(),
            decimals: 8,
            max_supply: 1_000_000_000,
            storage_mode: AccountStorageMode::Local,
        })
        .unwrap();
    let fungible_asset = FungibleAsset::new(faucet_account.id(), SMALL_BUY_IN_AMOUNT as u64).unwrap();
    
    // request highest bet from game account client
    let highest_bet = 5; // for now

    let playraise_txn_data = PlayRaiseTransactionData::new(
        Asset::Fungible(fungible_asset),
        player_account_id,
        game_account_id,
        highest_bet as u8 + amount.unwrap(),
    );

    let transaction_template = AzeTransactionTemplate::PlayRaise(playraise_txn_data);
    let txn_request = client.build_aze_play_raise_tx_request(transaction_template).unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;

    // note to be consumed by game account

    Ok(GameActionResponse { is_taken: true })
}

pub async fn call(
    player_id: u64,
    game_id: u64,
) -> Result<GameActionResponse, String> {
    let mut client: AzeClient = create_aze_client();
    let player_account_id = AccountId::try_from(player_id).unwrap();
    let game_account_id = AccountId::try_from(game_id).unwrap();

    let (faucet_account, _) = client
        .new_account(AccountTemplate::FungibleFaucet {
            token_symbol: TokenSymbol::new("MATIC").unwrap(),
            decimals: 8,
            max_supply: 1_000_000_000,
            storage_mode: AccountStorageMode::Local,
        })
        .unwrap();
    let fungible_asset = FungibleAsset::new(faucet_account.id(), SMALL_BUY_IN_AMOUNT as u64).unwrap();

    let playcall_txn_data = PlayCallTransactionData::new(
        Asset::Fungible(fungible_asset),
        player_account_id,
        game_account_id,
    );

    let transaction_template = AzeTransactionTemplate::PlayCall(playcall_txn_data);
    let txn_request = client.build_aze_play_call_tx_request(transaction_template).unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;

    // note to be consumed by game account

    Ok(GameActionResponse { is_taken: true })
}

pub async fn check(
    player_id: u64,
    game_id: u64,
) -> Result<GameActionResponse, String> {
    let mut client: AzeClient = create_aze_client();
    let player_account_id = AccountId::try_from(player_id).unwrap();
    let game_account_id = AccountId::try_from(game_id).unwrap();

    let (faucet_account, _) = client
        .new_account(AccountTemplate::FungibleFaucet {
            token_symbol: TokenSymbol::new("MATIC").unwrap(),
            decimals: 8,
            max_supply: 1_000_000_000,
            storage_mode: AccountStorageMode::Local,
        })
        .unwrap();
    let fungible_asset = FungibleAsset::new(faucet_account.id(), SMALL_BUY_IN_AMOUNT as u64).unwrap();

    let playcheck_txn_data = PlayCheckTransactionData::new(
        Asset::Fungible(fungible_asset),
        player_account_id,
        game_account_id,
    );

    let transaction_template = AzeTransactionTemplate::PlayCheck(playcheck_txn_data);
    let txn_request = client.build_aze_play_check_tx_request(transaction_template).unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;

    // note to be consumed by game account

    Ok(GameActionResponse { is_taken: true })
}

pub async fn fold(
    player_id: u64,
    game_id: u64,
) -> Result<GameActionResponse, String> {
    let mut client: AzeClient = create_aze_client();
    let player_account_id = AccountId::try_from(player_id).unwrap();
    let game_account_id = AccountId::try_from(game_id).unwrap();

    let (faucet_account, _) = client
        .new_account(AccountTemplate::FungibleFaucet {
            token_symbol: TokenSymbol::new("MATIC").unwrap(),
            decimals: 8,
            max_supply: 1_000_000_000,
            storage_mode: AccountStorageMode::Local,
        })
        .unwrap();
    let fungible_asset = FungibleAsset::new(faucet_account.id(), SMALL_BUY_IN_AMOUNT as u64).unwrap();

    let playfold_txn_data = PlayFoldTransactionData::new(
        Asset::Fungible(fungible_asset),
        player_account_id,
        game_account_id,
    );

    let transaction_template = AzeTransactionTemplate::PlayFold(playfold_txn_data);
    let txn_request = client.build_aze_play_fold_tx_request(transaction_template).unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;

    // note to be consumed by game account

    Ok(GameActionResponse { is_taken: true })
}

pub async fn bet(
    player_id: u64,
    game_id: u64,
    amount: u8
) -> Result<GameActionResponse, String> {
    let mut client: AzeClient = create_aze_client();
    let player_account_id = AccountId::try_from(player_id).unwrap();
    let game_account_id = AccountId::try_from(game_id).unwrap();

    let (faucet_account, _) = client
        .new_account(AccountTemplate::FungibleFaucet {
            token_symbol: TokenSymbol::new("MATIC").unwrap(),
            decimals: 8,
            max_supply: 1_000_000_000,
            storage_mode: AccountStorageMode::Local,
        })
        .unwrap();
    let fungible_asset = FungibleAsset::new(faucet_account.id(), SMALL_BUY_IN_AMOUNT as u64).unwrap();

    let playbet_txn_data = PlayBetTransactionData::new(
        Asset::Fungible(fungible_asset),
        player_account_id,
        game_account_id,
        amount,
    );

    let transaction_template = AzeTransactionTemplate::PlayBet(playbet_txn_data);
    let txn_request = client.build_aze_play_bet_tx_request(transaction_template).unwrap();
    execute_tx_and_sync(&mut client, txn_request.clone()).await;

    // note to be consumed by game account

    Ok(GameActionResponse { is_taken: true })
}

pub async fn small_blind(
    player_id: u64,
    game_id: u64,
) -> Result<GameActionResponse, String> {
    // request small blind amount from game account
    let small_blind = 5; // for now
    bet(player_id, game_id, small_blind).await
}

pub async fn big_blind(
    player_id: u64,
    game_id: u64,
) -> Result<GameActionResponse, String> {
    // request big blind amount from game account
    let big_blind = 10; // for now
    bet(player_id, game_id, big_blind).await
}