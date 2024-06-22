use std::sync::{Arc, Mutex};
use crate::node::Node;

pub struct WalletBackend {
    pub filename: String,
    pub password: String,
    pub daemon: Arc<Mutex<Node>>,
    // m_event_handler: Arc<EventHandler>,
    // m_sub_wallets: Arc<SubWallets>,
}

impl WalletBackend {
    fn get_node_fee(&self) -> f64 {
        match self.daemon.lock() {
            Ok(mut guard) => guard.get_fee(),
            Err(_) => 0.0,
        }
    }

    fn get_node_address(&self) -> String {
        match self.daemon.lock() {
            Ok(mut guard) => guard.get_address(),
            Err(_) => String::new(),
        }
    }

    fn get_node_host(&self) -> String {
        match self.daemon.lock() {
            Ok(mut guard) => guard.get_host(),
            Err(_) => String::new(),
        }
    }

    fn get_node_port(&self) -> i8 {
        match self.daemon.lock() {
            Ok(mut guard) => guard.get_port(),
            Err(_) => 0,
        }
    }

    fn get_wallet_filename(&self) -> String {
        self.filename.clone()
    }
}
