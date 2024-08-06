use bitcoincore_rpc::{Auth, Client};

lazy_static! {
    pub static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = std::env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = std::env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String = std::env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        
        return Client::new(
            &rpc_url, Auth::UserPass(rpc_user, rpc_password)
        ).expect("Error while creating RPC Client");
    };
}
