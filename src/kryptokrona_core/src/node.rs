use std::{
    thread,
    time::Duration,
};

pub struct Node {
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

impl Node {
    fn new(daemon_host: String, daemon_port: i8) -> Self {
        let timeout = Duration::from_secs(10);
        // TODO: create a new Hyper Client here
        // let http_client = Arc::new(Mutex::new(Client::new(
        //     format!("{}:{}", daemon_host, daemon_port),
        //     timeout,
        // )));

        Node {
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

    fn swap_node(&mut self, daemon_host: String, daemon_port: i8) {
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

    fn stop(&mut self) {
        self.should_stop = true;
        if let Some(handle) = thread::current().name() {
            if handle == "background_thread" {
                if let Some(background_thread) = self.background_thread.take() {
                    background_thread.join().unwrap();
                }
            }
        }
    }

    fn init(&mut self) {
        self.should_stop = false;

        self.get_daemon_info();
        self.get_fee_info();

        // let http_client = Arc::clone(&self.http_client);
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

    pub fn get_fee(&mut self) -> f64 {
        0.0
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

    pub fn get_daemon_info(&mut self) -> String {
        String::new()
    }

    pub fn get_fee_info(&mut self) -> String {
        String::new()
    }
}
