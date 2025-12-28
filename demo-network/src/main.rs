use deNetwork::{DeMultiNet as Net, DeNet, Stats};
use structopt::StructOpt;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use ark_bn254::{Fr, G1Projective};
use ark_std::Zero;
use ark_ec::Group;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};

const NUM_COMMITMENTS_PER_NODE: usize = 10000;

#[derive(Debug, StructOpt)]
#[structopt(name = "demo-low", about = "Low-level KZG aggregation with Inline Phase Reporting")]
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
    
    // --- DATA PREPARATION ---
    let generator = G1Projective::generator();
    let commitments: Vec<G1Projective> = (0..NUM_COMMITMENTS_PER_NODE)
        .map(|j| generator * Fr::from((my_id + j) as u64))
        .collect();
    let mut bytes = Vec::new();
    commitments.serialize_uncompressed(&mut bytes).unwrap();

    // =========================================================================
    // PHASE 1: UPLOAD (All Nodes -> Master)
    // =========================================================================
    let s1 = Net::stats();
    let start = Instant::now();
    let gathered_bytes = Net::send_bytes_to_master(bytes);
    let api_duration = start.elapsed();
    
    // Wait 200ms for TCP retransmissions to be triggered/detected by kernel
    thread::sleep(Duration::from_millis(200));
    let s2 = Net::stats();
    
    println!("[Node {}][Phase 1] API Call took: {:?}", my_id, api_duration);
    println!("[Node {}][Phase 1] Sent:     {} bytes", my_id, s2.bytes_sent - s1.bytes_sent);
    println!("[Node {}][Phase 1] Loss Info: {} TCP Retransmissions detected", my_id, s2.retransmissions - s1.retransmissions);

    // --- MASTER AGGREGATION (Node 0 Phase 1) ---
    let master_response_bytes = gathered_bytes.map(|all_bytes| {
        println!("[Node 0][Phase 1] Deserializing and Aggregating 40,000 points...");
        let comp_start = Instant::now();
        let all_node_data: Vec<Vec<G1Projective>> = all_bytes.iter().map(|b| {
            Vec::<G1Projective>::deserialize_uncompressed_unchecked(&b[..]).unwrap()
        }).collect();

        let mut sum = G1Projective::zero();
        for node_vec in all_node_data {
            for c in node_vec {
                sum += c;
            }
        }
        println!("[Node 0][Phase 1] Computation took: {:?}", comp_start.elapsed());

        let expected = G1Projective::generator() * Fr::from(2004000u64);
        if sum == expected {
            println!("[Node 0][Phase 1] VERIFICATION SUCCESS");
        } else {
            println!("[Node 0][Phase 1] VERIFICATION FAILED");
        }

        let mut res_bytes = Vec::new();
        sum.serialize_uncompressed(&mut res_bytes).unwrap();
        vec![res_bytes; Net::n_parties()]
    });

    // =========================================================================
    // PHASE 2: DOWNLOAD (Master -> All Nodes)
    // =========================================================================
    let s3 = Net::stats();
    let start = Instant::now();
    let final_bytes = Net::recv_bytes_from_master(master_response_bytes);
    let api_duration = start.elapsed();

    // Wait 200ms for TCP retransmissions
    thread::sleep(Duration::from_millis(200));
    let s4 = Net::stats();

    println!("[Node {}][Phase 2] API Call took: {:?}", my_id, api_duration);
    println!("[Node {}][Phase 2] Recv:     {} bytes", my_id, s4.bytes_recv - s3.bytes_recv);
    println!("[Node {}][Phase 2] Loss Info: {} TCP Retransmissions detected", my_id, s4.retransmissions - s3.retransmissions);

    // Final check
    let _final_sum = G1Projective::deserialize_uncompressed_unchecked(&final_bytes[..]).unwrap();
    println!("[Node {}][Phase 2] Protocol completed successfully.", my_id);

    thread::sleep(Duration::from_millis(500));
    Net::deinit();
    println!("[Node {}][Phase 2] Exiting.", my_id);
}