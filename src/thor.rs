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
use std::collections::HashMap;
use reqwest::Error;

pub struct Thor {
    pub timeout: Duration,
    pub daemon_host: String,
    pub daemon_port: i8,
    pub should_stop: bool,
    pub local_daemon_block_count: u64,
    pub network_block_count: u64,
    pub peer_count: u64,
    pub last_known_hashrate: u64,
    pub node_fee_address: String,
    pub node_fee_amount: f64,
    pub background_thread: Option<thread::JoinHandle<()>>,
}

impl Thor {
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
            node_fee_amount: 0.0,
            background_thread: None,
        }
    }

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

    pub fn init(&mut self) {
        self.should_stop = false;

        self.get_daemon_info();
        self.get_fee_info();
    }

    pub fn get_fee(&mut self) -> f64 {
        self.node_fee_amount.clone()
    }

    pub fn get_address(&mut self) -> String {
        String::new()
    }

    pub fn get_host(&mut self) -> String {
        self.daemon_host.clone()
    }

    pub fn get_port(&mut self) -> i8 {
        self.daemon_port.clone()
    }

    pub async fn get_daemon_info(&mut self) -> Result<(), Error> {
        let resp = reqwest::get(format!("https://{}:{}/info", self.daemon_host, self.daemon_port))
            .await?
            .json::<HashMap<String, String>>()
            .await?;

        // let response_text = format("{:#?}", resp.);
        println!("{resp:#?}");

            //     const auto res = m_httpClient->Get("/info");
            //
            // if (res && res->status == 200)
            // {
            //     try
            //     {
            //         json j = json::parse(res->body);
            //
            //         m_localDaemonBlockCount = j.at("height").get<uint64_t>();
            //
            //         /* Height returned is one more than the current height - but we
            //            don't want to overflow is the height returned is zero */
            //         if (m_localDaemonBlockCount != 0)
            //         {
            //             m_localDaemonBlockCount--;
            //         }
            //
            //         m_networkBlockCount = j.at("network_height").get<uint64_t>();
            //
            //         /* Height returned is one more than the current height - but we
            //            don't want to overflow is the height returned is zero */
            //         if (m_networkBlockCount != 0)
            //         {
            //             m_networkBlockCount--;
            //         }
            //
            //         m_peerCount = j.at("incoming_connections_count").get<uint64_t>() + j.at("outgoing_connections_count").get<uint64_t>();
            //
            //         m_lastKnownHashrate = j.at("difficulty").get<uint64_t>() / cryptonote::parameters::DIFFICULTY_TARGET;
            //
            //         return true;
            //     }
            //     catch (const json::exception &)
            //     {
            //     }
            // }
            //
            // return false;
        Ok(())
    }

    pub fn get_fee_info(&mut self) -> String {
        String::new()
    }
}
