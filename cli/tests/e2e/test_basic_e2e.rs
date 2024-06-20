mod utils;
use utils::{ 
    create_test_client,
    create_faucet_account,
    create_player_account,
    create_game_account,
    mask_cards,
    remask_cards,
    peek_hand,
    unmask_community_cards,
};
use aze_lib::client::{
    AzeClient,
    AzeAccountTemplate,
};
use aze_lib::constants::{
    SECRET_KEY_SLOT,
    DEFAULT_SKEY,
    MASKING_FACTOR_SLOT,
    DEFAULT_MASKING_FACTOR,
    PLAYER_DATA_SLOT,
    DEFAULT_ACTION_TYPE,
    PLAYER_CARD1_SLOT,
    PLAYER_CARD2_SLOT,
    REQUESTER_SLOT,
    TEMP_CARD_SLOT,
};
use miden_client::{
    client::accounts::{ AccountTemplate, AccountStorageMode },
    errors::ClientError,
};
use miden_objects::{
    accounts::Account,
    crypto::hash::rpo::RpoDigest,
    Felt, FieldElement
};

#[tokio::test]
async fn test_e2e() {
    let mut client: AzeClient = create_test_client();
    let faucet_account_id = create_faucet_account(&mut client);

    // Create player accounts
    let player1_id = create_player_account(&mut client, faucet_account_id).await;
    let player2_id = create_player_account(&mut client, faucet_account_id).await;
    let player3_id = create_player_account(&mut client, faucet_account_id).await;
    let player4_id = create_player_account(&mut client, faucet_account_id).await;
    let player_ids = vec![player1_id, player2_id, player3_id, player4_id];

    for player_id in player_ids.iter() {
        let (player_account, _) = client.get_account(*player_id).unwrap();
        assert_eq!(player_account.storage().get_item(SECRET_KEY_SLOT), RpoDigest::new([Felt::from(DEFAULT_SKEY), Felt::ZERO, Felt::ZERO, Felt::ZERO]));
        assert_eq!(player_account.storage().get_item(MASKING_FACTOR_SLOT), RpoDigest::new([Felt::from(DEFAULT_MASKING_FACTOR), Felt::ZERO, Felt::ZERO, Felt::ZERO]));
    }

    // Create an game account
    let game_account_id = create_game_account(&mut client, faucet_account_id).await;

    // Mask the cards
    mask_cards(&mut client, game_account_id, faucet_account_id, player_ids.clone()).await;
    remask_cards(&mut client, game_account_id, faucet_account_id, player_ids.clone(), DEFAULT_ACTION_TYPE + 1).await;
    remask_cards(&mut client, game_account_id, faucet_account_id, player_ids.clone(), DEFAULT_ACTION_TYPE + 2).await;
    remask_cards(&mut client, game_account_id, faucet_account_id, player_ids.clone(), DEFAULT_ACTION_TYPE + 3).await;

    for (i, player_id) in player_ids.iter().enumerate() {
        let (player_account, _) = client.get_account(*player_id).unwrap();
        let mut exp_player_ids = player_ids.clone();
        exp_player_ids.remove(i);
        let player_data = [Felt::new(DEFAULT_ACTION_TYPE + i as u64), Felt::new(exp_player_ids[0].into()), Felt::new(exp_player_ids[1].into()), Felt::new(exp_player_ids[2].into())];
        assert_eq!(player_account.storage().get_item(PLAYER_DATA_SLOT), RpoDigest::new(player_data));
    }

    // Distribute the cards
    // Peek hand
    for player_id in player_ids.iter() {
        peek_hand(&mut client, faucet_account_id, *player_id).await;
        let (player_account, _) = client.get_account(*player_id).unwrap();
        let player_card1 = player_account.storage().get_item(PLAYER_CARD1_SLOT);
        let player_card2 = player_account.storage().get_item(PLAYER_CARD2_SLOT);
        assert_eq!(player_card1, RpoDigest::new([Felt::from(17_u8), Felt::ZERO, Felt::ZERO, Felt::ZERO]));
        assert_eq!(player_card2, RpoDigest::new([Felt::from(18_u8), Felt::ZERO, Felt::ZERO, Felt::ZERO]));
    }

    // Unmask community cards
    unmask_community_cards(&mut client, faucet_account_id, game_account_id, player1_id).await;

    // Commit hand
}