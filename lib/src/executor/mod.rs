use crate::client::AzeClient;
use miden_client::{
    client::transactions::transaction_request::TransactionRequest, store::TransactionFilter,
};

pub async fn execute_tx_and_sync(client: &mut AzeClient, tx_request: TransactionRequest) {
    let _ = match client.sync_state().await {
        Ok(_) => (),
        Err(e) => {
            println!("Error syncing state: {:?}", e);
            return;
        }
    };
    let transaction_execution_result = match client.new_transaction(tx_request.clone()) {
        Ok(result) => result,
        Err(e) => {
            return;
        }
    };
    let transaction_id = transaction_execution_result.executed_transaction().id();

    client
        .submit_transaction(transaction_execution_result)
        .await
        .unwrap();

    // wait until tx is committed
    loop {
        client.sync_state().await.unwrap();

        // Check if executed transaction got committed by the node
        let uncommited_transactions = client
            .get_transactions(TransactionFilter::Uncomitted)
            .unwrap();
        let is_tx_committed = uncommited_transactions
            .iter()
            .find(|uncommited_tx| uncommited_tx.id == transaction_id)
            .is_none();

        if is_tx_committed {
            break;
        }

        std::thread::sleep(std::time::Duration::new(3, 0));
    }
}
