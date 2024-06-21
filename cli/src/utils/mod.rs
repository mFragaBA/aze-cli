use aze_lib::constants::{ FAUCET_ACCOUNT_ID, SMALL_BUY_IN_AMOUNT };
use miden_objects::{
    accounts::AccountId,
    assets::{ Asset, FungibleAsset },
};

pub fn get_faucet_id() -> AccountId {
    AccountId::try_from(FAUCET_ACCOUNT_ID).unwrap()
}

pub fn get_note_asset() -> Asset {
    let fungible_asset = FungibleAsset::new(get_faucet_id(), SMALL_BUY_IN_AMOUNT as u64).unwrap();
    Asset::Fungible(fungible_asset)
}