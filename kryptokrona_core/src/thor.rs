// Copyright (c) 2019-2024, The Kryptokrona Project
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

use std::{
    thread,
    time::Duration,
};

pub struct Thor {
    pub timeout: Duration,
    pub daemon_host: String,
    pub daemon_port: i8,
    // http_client: Arc<Mutex<Client>>,
    pub should_stop: bool,
    pub local_daemon_block_count: u64,
    pub network_block_count: u64,
    pub peer_count: u64,
    pub last_known_hashrate: u64,
    pub node_fee_address: String,
    pub node_fee_amount: u32,
    pub background_thread: Option<thread::JoinHandle<()>>,
}

impl Thor {
    #[allow(dead_code)]
    pub fn new(daemon_host: String, daemon_port: i8) -> Self {
        let timeout = Duration::from_secs(10);
        // TODO: create a new Hyper Client here
        // let http_client = Arc::new(Mutex::new(Client::new(
        //     format!("{}:{}", daemon_host, daemon_port),
        //     timeout,
        // )));

        Thor {
            timeout,
            daemon_host,
            daemon_port,
            should_stop: false,
            local_daemon_block_count: 0,
            network_block_count: 0,
            peer_count: 0,
            last_known_hashrate: 0,
            node_fee_address: String::new(),
            node_fee_amount: 0,
            background_thread: None,
        }
    }

    #[allow(dead_code)]
    pub fn swap_node(&mut self, daemon_host: String, daemon_port: i8) {
        self.stop();

        self.local_daemon_block_count = 0;
        self.network_block_count = 0;
        self.peer_count = 0;
        self.last_known_hashrate = 0;

        self.daemon_host = daemon_host;
        self.daemon_port = daemon_port;

        // let http_client = Arc::new(Mutex::new(httplib::Client::new(
        //     format!("{}:{}", daemon_host, daemon_port),
        //     self.timeout,
        // )));

        // self.http_client = http_client;

        self.init();
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.should_stop = true;
        if let Some(handle) = thread::current().name() {
            if handle == "background_thread" {
                if let Some(background_thread) = self.background_thread.take() {
                    background_thread.join().unwrap();
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn init(&mut self) {
        self.should_stop = false;

        self.get_daemon_info();
        self.get_fee_info();

        // let http_client = Arc::clone(&self.http_client);
        #[allow(unused_variables)]
        let timeout = self.timeout;
        // thread::Builder::new()
        //     .name("background_thread".to_string())
        //     .spawn(move || {
        //         while !self.should_stop {
        //             self.get_daemon_info();
        //             thread::sleep(timeout);
        //         }
        //     })
        //     .unwrap();
    }

    #[allow(dead_code)]
    pub fn get_fee(&mut self) -> f64 {
        0.0
    }

    #[allow(dead_code)]
    pub fn get_address(&mut self) -> String {
        String::new()
    }

    #[allow(dead_code)]
    pub fn get_host(&mut self) -> String {
        self.daemon_host.clone()
    }

    #[allow(dead_code)]
    pub fn get_port(&mut self) -> i8 {
        self.daemon_port.clone()
    }

    #[allow(dead_code)]
    pub fn get_daemon_info(&mut self) -> String {
        String::new()
    }

    #[allow(dead_code)]
    pub fn get_fee_info(&mut self) -> String {
        String::new()
    }
}
