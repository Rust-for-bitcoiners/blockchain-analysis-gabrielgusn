#![allow(unused)]
mod rpc;
mod errors;
mod utils;

use std::{env, time};
use bitcoincore_rpc::{json, jsonrpc::{self}, Auth, Client, RpcApi};
use chrono::Duration;
use rpc::RPC_CLIENT;
use crate::utils::block::number_of_transactions;

#[macro_use]
extern crate lazy_static;

// TODO: Task 1
// fn time_to_mine(block_height: u64) -> Duration {
//     // * is a deref operator which invokes the Deref trait of the type RPC_CLIENT which was created
//     // when the lazy macro is expanded
//     // if a value has a static lifetime then it means that value lives as long as the program lives
//     let rpc_client: &Client = &*RPC_CLIENT;
//     rpc_client.get_block_hash(234);
//     todo!()
// }

fn main() {
    // you can use rpc_client here as if it was a global variable
    // println!("{:?}", res);
    dotenv::dotenv().ok();
    const TIMEOUT_UTXO_SET_SCANS: time::Duration = time::Duration::from_secs(60 * 8); // 8 minutes

    let rpc_client: &Client = &*RPC_CLIENT;
    print!("{}", number_of_transactions(&rpc_client, 0));

    // let custom_timeout_transport = jsonrpc::simple_http::Builder::new()
    //     .url(&rpc_url)
    //     .expect("invalid rpc url")
    //     .auth(rpc_user, Some(rpc_password))
    //     .timeout(TIMEOUT_UTXO_SET_SCANS)
    //     .build();
    // let custom_timeout_rpc_client =
    //     jsonrpc::client::Client::with_transport(custom_timeout_transport);

    // let rpc_client = Client::from_jsonrpc(custom_timeout_rpc_client);
    // let res: json::GetTxOutSetInfoResult =
    //     rpc_client.get_tx_out_set_info(None, None, None).unwrap();
    // println!("{:?}", res);
}
