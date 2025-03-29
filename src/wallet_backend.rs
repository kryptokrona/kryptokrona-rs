// Copyright (c) 2025, The Kryptokrona Project
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification, are
// permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of
//    conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list
//    of conditions and the following disclaimer in the documentation and/or other
//    materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be
//    used to endorse or promote products derived from this software without specific
//    prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL
// THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
// THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::thor::Thor;
use std::sync::{Arc, Mutex};

pub struct WalletBackend {
    pub filename: String,
    pub password: String,
    pub private_spend_key: String,
    pub private_view_key: String,
    pub scan_height: u64,
    pub new_wallet: bool,
    pub daemon: Arc<Mutex<Thor>>,
}

impl WalletBackend {
    #[allow(dead_code)]
    fn get_node_fee(&self) -> f64 {
        match self.daemon.lock() {
            Ok(mut guard) => guard.get_fee(),
            Err(_) => 0.0,
        }
    }

    #[allow(dead_code)]
    fn get_node_address(&self) -> String {
        match self.daemon.lock() {
            Ok(mut guard) => guard.get_address(),
            Err(_) => String::new(),
        }
    }

    #[allow(dead_code)]
    fn get_node_host(&self) -> String {
        match self.daemon.lock() {
            Ok(mut guard) => guard.get_host(),
            Err(_) => String::new(),
        }
    }

    #[allow(dead_code)]
    fn get_node_port(&self) -> i8 {
        match self.daemon.lock() {
            Ok(mut guard) => guard.get_port(),
            Err(_) => 0,
        }
    }

    #[allow(dead_code)]
    fn get_wallet_filename(&self) -> String {
        self.filename.clone()
    }
}
