use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use deNetwork::{DeMultiNet, DeNet, DeSerNet, Stats};
use log::info;
use structopt::StructOpt;
use std::thread;
use std::time::Duration;

#[derive(Debug, StructOpt)]
#[structopt(name = "demo_net", about = "Demo for deNetwork")]
struct Opt {
    /// Party ID
    #[structopt(short, long)]
    id: usize,

    /// Number of parties
    #[structopt(short, long, default_value = "3")]
    n: usize,

    /// Path to hosts file
    #[structopt(short = "h", long, default_value = "hosts.txt")]
    hosts: String,
}

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug, Default, PartialEq)]
struct Msg {
    sender: usize,
    val: u64,
}

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    let opt = Opt::from_args();

    info!("Party {} initializing from {}", opt.id, opt.hosts);
    DeMultiNet::init_from_file(&opt.hosts, opt.id);

    // 1. Basic Info
    assert!(DeMultiNet::is_init(), "Network should be initialized");
    info!("Initialized. ID: {}, N: {}", DeMultiNet::party_id(), DeMultiNet::n_parties());
    assert_eq!(DeMultiNet::party_id(), opt.id);

    // 2. Stats (initial)
    let initial_stats = DeMultiNet::stats();
    info!("Initial stats: {:?}", initial_stats);

    // 3. Channels Demo
    // We will use 2 channels: 0 and 1.

    // --- Channel 0 ---
    DeMultiNet::set_channel_id(0);
    info!("Switched to Channel 0");

    let msg = Msg { sender: opt.id, val: 1000 + opt.id as u64 };

    // Test: send_to_master / recv_from_master (Echo)
    // Note: send_bytes_to_master and recv_bytes_from_master are covered by these typed wrappers.
    info!("Channel 0: Testing send_to_master -> recv_from_master (Echo)");
    let collected = DeMultiNet::send_to_master(&msg);
    if DeMultiNet::am_master() {
        info!("Master received collected messages on Ch 0");
        // Verify we got messages from everyone (just a length check here)
        assert_eq!(collected.as_ref().unwrap().len(), opt.n);
    }

    // Master distributes the collected messages back to everyone.
    // recv_from_master returns the message *intended for this party*.
    // Since we passed 'collected' (which is [msg0, msg1, ...]), each party gets their own message back.
    let echo = DeMultiNet::recv_from_master(collected);
    info!("Party {} received echo: {:?}", opt.id, echo);
    assert_eq!(echo, msg);

    // Test: king_compute (Sum)
    // Note: master_compute is covered by king_compute.
    info!("Channel 0: Testing king_compute (Sum)");
    let val_to_sum = opt.id as u64;
    let sum_res = DeMultiNet::king_compute(&val_to_sum, |vals| {
        let sum: u64 = vals.iter().sum();
        // Return the sum to everyone
        vec![sum; vals.len()]
    });
    info!("Party {} received sum: {}", opt.id, sum_res);
    // Sum of 0..N is N*(N-1)/2. For N=3: 0+1+2 = 3.
    let expected_sum = (opt.n * (opt.n - 1) / 2) as u64;
    assert_eq!(sum_res, expected_sum);


    // --- Channel 1 ---
    DeMultiNet::set_channel_id(1);
    info!("Switched to Channel 1");

    // Test: recv_from_master_uniform (Broadcast from Master)
    info!("Channel 1: Testing recv_from_master_uniform");
    let broadcast_val = if DeMultiNet::am_master() {
        Some(Msg { sender: 0, val: 9999 })
    } else {
        None
    };
    let received_uniform = DeMultiNet::recv_from_master_uniform(broadcast_val);
    info!("Party {} received uniform: {:?}", opt.id, received_uniform);
    assert_eq!(received_uniform.val, 9999);


    // Note: broadcast and atomic_broadcast are unimplemented in DeMultiNet and panic if called.
    // Note: exchange and atomic_exchange are incompatible with DeMultiNet (depend on DeTwoNet).

    // 4. Reset Stats
    info!("Resetting stats");
    DeMultiNet::reset_stats();
    let stats = DeMultiNet::stats();
    assert_eq!(stats.bytes_sent, 0);

    // 5. Deinit
    info!("Deinitializing");
    DeMultiNet::deinit();
    info!("Done");
}
