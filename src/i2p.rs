use std::{thread,fs::File, io::{self, BufRead}, path::Path};

use j4i2prs::router_wrapper as rw;
use j4i2prs::tunnel_control as tc;
use std::sync::mpsc::{
    Receiver,
    Sender,
};

struct Listener {
    is_running: bool,
    run_tx: Sender<bool>,
    run_rx: Receiver<bool>,
}

impl Default for Listener {
    fn default() -> Self {
        let is_running = false;
        let (run_tx, run_rx) = std::sync::mpsc::channel();
        Listener {
            is_running,
            run_tx,
            run_rx,
        }
    }
}

/// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Start router and automatic i2p tunnel creation
pub fn start_tunnel() -> Result<(), j4rs::errors::J4RsError> {
    let str_app_port = std::env::var("I2GPT_PORT").unwrap_or(String::from("3141"));
    let app_port: u16 = str_app_port.parse::<u16>().unwrap_or(3141);
    let str_http_proxy = std::env::var("I2PGPT_PROXY_PORT").unwrap_or(String::from("4455"));
    let http_proxy: u16 = str_http_proxy.parse::<u16>().unwrap_or(4455);
    log::info!("starting j4i2prs...");
    let r = rw::Wrapper::create_router()?;
    let mut l: Listener = Default::default();
    let run_tx = l.run_tx.clone();
    let _ = thread::spawn(move || {
        log::info!("run thread started");
        run_tx.send(true).unwrap_or_else(|_| log::error!("failed to run router"));
    });
    // run the main thread forever unless we get a router shutdown signal
    let _ = thread::spawn(move || {
        // ctrl+c signal handler
        let _ = ctrlc::set_handler(move || {
            std::process::exit(0);
        });
        std::thread::sleep(std::time::Duration::from_secs(10));
        loop {
            if let Ok(run) = l.run_rx.try_recv() {
                if run {
                    log::info!("starting router");
                    r.invoke_router(rw::METHOD_RUN).unwrap_or_else(|_| log::error!("failed to run router"));
                }
            }
            if !l.is_running {
                let is_router_on = r.is_running().unwrap_or_default();
                if !is_router_on {
                    log::info!("router is warming up, please wait...");
                }
                std::thread::sleep(std::time::Duration::from_secs(60));
                if is_router_on {
                    // check router config
                    if let Ok(lines) = read_lines("./router.config") {
                        for line in lines.map_while(Result::ok) {
                            if line.contains("i2np.udp.port") {
                                let port = line.split("=").collect::<Vec<&str>>()[1];
                                log::info!("router is running on external port = {}", port);
                                log::info!("open this port for better connectivity");
                                log::info!("this port was randomly assigned, keep it private");
                                l.is_running = true;
                                // start the http proxy
                                let http_proxy: tc::Tunnel = tc::Tunnel::new("127.0.0.1".to_string(), http_proxy, tc::TunnelType::Http)
                                    .unwrap_or_default();
                                let _ = http_proxy.start();
                                log::info!("http proxy on port {}", http_proxy.get_port());
                                // start the tunnel
                                let app_tunnel: tc::Tunnel = tc::Tunnel::new("127.0.0.1".to_string(), app_port, tc::TunnelType::Server)
                                    .unwrap_or_default();
                                log::info!("destination: {}", app_tunnel.get_destination());
                                let _ = app_tunnel.start();
                            }
                        }
                    }
                }
            }
        }
    });
    Ok(())
}
