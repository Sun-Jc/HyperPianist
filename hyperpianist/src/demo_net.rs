use deNetwork::{DeMultiNet as Net, DeNet};
use log::info;

fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].clone();
    let id = args[2].parse::<usize>().unwrap();
    info!("Initializing network for party {}", id);
    Net::init_from_file(filename.as_str(), id);
    info!("Network initialized for party {}", id);
    if id == 0 {
        info!("Party {} going to sleep", id);
        std::thread::sleep(std::time::Duration::from_secs(3));
        info!("Party {} woke up", id);
    }
    Net::deinit();
    info!("Network deinitialized for party {}", id);
}
