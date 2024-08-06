use bitcoincore_rpc::{Client, RpcApi};

pub fn number_of_transactions(rpc_client: &Client, lock_height: u64) -> u16 {
    
    let block_info = rpc_client.get_block_stats(lock_height).unwrap();

    block_info.txs as u16
}
