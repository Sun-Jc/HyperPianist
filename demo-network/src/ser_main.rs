use deNetwork::{DeMultiNet as Net, DeNet, DeSerNet, Stats};
use structopt::StructOpt;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use ark_bn254::{Fr, G1Projective};
use ark_std::Zero;
use ark_ec::Group;

const NUM_COMMITMENTS_PER_NODE: usize = 1000;

#[derive(Debug, StructOpt)]
#[structopt(name = "demo-ser", about = "KZG aggregation using DeSerNet (High-level API)")]
struct Opt {
    id: usize,
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let my_id = opt.id;
    let ip_file_path = opt.input.to_str().expect("Invalid path");

    Net::init_from_file(ip_file_path, my_id);
    
    // 1. Prep data
    let generator = G1Projective::generator();
    let commitments: Vec<G1Projective> = (0..NUM_COMMITMENTS_PER_NODE)
        .map(|j| generator * Fr::from((my_id + j) as u64))
        .collect();

    // 2. High-level Distributed Logic using DeSerNet
    // Note: Net::send_to_master and Net::recv_from_master handle 
    // serialization and network logic internally.

    // --- SEND PHASE (Includes Internal Serialization) ---
    let s1 = Net::stats();
    let send_start = Instant::now();
    
    // DeSerNet call
    let gathered: Option<Vec<Vec<G1Projective>>> = Net::send_to_master(&commitments);
    
    let send_duration = send_start.elapsed();
    let s2 = Net::stats();
    println!(
        "[Node {}] DeSerNet SEND took: {:?}, Sent: {} bytes, Recv: {} bytes", 
        my_id, send_duration, s2.bytes_sent - s1.bytes_sent, s2.bytes_recv - s1.bytes_recv
    );

    // --- MASTER LOGIC (Aggregation) ---
    let master_response = gathered.map(|all_node_data| {
        let comp_start = Instant::now();
        let mut sum = G1Projective::zero();
        for node_vec in all_node_data {
            for c in node_vec { sum += c; }
        }
        println!("[Master] Local computation took: {:?}", comp_start.elapsed());
        
        // Return result wrapped for all parties
        vec![vec![sum]; Net::n_parties()]
    });

    // --- RECEIVE PHASE (Includes Internal Deserialization) ---
    let s3 = Net::stats();
    let recv_start = Instant::now();
    
    // DeSerNet call
    let combined_vec: Vec<G1Projective> = Net::recv_from_master(master_response);
    
    let recv_duration = recv_start.elapsed();
    let s4 = Net::stats();
    println!(
        "[Node {}] DeSerNet RECEIVE took: {:?}, Sent: {} bytes, Recv: {} bytes", 
        my_id, recv_duration, s4.bytes_sent - s3.bytes_sent, s4.bytes_recv - s3.bytes_recv
    );

    println!("[Node {}] Final Sum result: {}", my_id, combined_vec[0]);

    thread::sleep(Duration::from_millis(500));
    Net::deinit();
}
