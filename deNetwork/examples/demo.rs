use deNetwork::{DeMultiNet, DeNet};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <hosts_file> <party_id>", args[0]);
        std::process::exit(1);
    }

    let hosts_file = &args[1];
    let party_id: usize = args[2].parse().expect("Party ID must be a number");

    println!("Party {}: Initializing...", party_id);
    DeMultiNet::init_from_file(hosts_file, party_id);
    println!("Party {}: Connected!", party_id);

    // Step 1: Send data to master
    let my_data = vec![party_id as u8];
    println!("Party {}: Sending {:?} to master", party_id, my_data);

    // Master receives a Vec<Vec<u8>> (one Vec<u8> from each party).
    // Others receive None.
    let gathered_data = DeMultiNet::send_bytes_to_master(my_data);

    // Step 2: Master processes data and sends back result
    let response_data = if DeMultiNet::am_master() {
        let all_data = gathered_data.unwrap();
        println!("Master: Received data from {} parties: {:?}", all_data.len(), all_data);

        let sum: u8 = all_data.iter().map(|v| v[0]).sum();
        println!("Master: Sum is {}", sum);

        // Prepare response for each party (sending the sum to everyone)
        // We need to return Option<Vec<Vec<u8>>> to recv_bytes_from_master.
        // The outer Vec has length n_parties.
        let n = DeMultiNet::n_parties();
        let mut response = Vec::new();
        for _ in 0..n {
            response.push(vec![sum]);
        }
        Some(response)
    } else {
        None
    };

    // Step 3: Receive data from master
    // recv_bytes_from_master takes Option<Vec<Vec<u8>>> (Some for master, None for others)
    // and returns Vec<u8> (the data for this specific party).
    let result_bytes = DeMultiNet::recv_bytes_from_master(response_data);
    let result_sum = result_bytes[0];

    println!("Party {}: Received sum {} from master", party_id, result_sum);

    // Demonstrate master_compute helper
    // Let's do another round where we multiply the value by 2
    let val_to_send = vec![result_sum];
    println!("Party {}: Starting master_compute with {:?}", party_id, val_to_send);

    let computed_result = DeMultiNet::master_compute(val_to_send, |all_data| {
        // This closure only runs on master
        println!("Master (compute): Received {:?}", all_data);
        let val = all_data[0][0]; // Just take the first one's value (they are all the same sum)
        let new_val = val * 2;
        let n = all_data.len();
        vec![vec![new_val]; n]
    });

    println!("Party {}: Result from master_compute: {:?}", party_id, computed_result);

    DeMultiNet::deinit();
}
