# deNetwork

`deNetwork` is a library for distributed networking, designed to support multi-party computation scenarios where one party acts as a "master" and others as "workers" (or parties). It provides abstractions for establishing connections and exchanging data.

## Key Features

- **Initialization**: Initialize the network from a configuration file listing the addresses of all parties.
- **Master-Worker Communication**: Optimized patterns for workers sending data to the master and the master sending data back to workers.
- **Global State**: Uses a singleton pattern (`lazy_static`) to manage connections, simplifying usage in the application code but limiting to one node per process.

## Usage

### 1. Configuration File
Create a file (e.g., `hosts.txt`) containing the `IP:PORT` of all participating nodes, one per line. The line number corresponds to the party ID (0-indexed). Party 0 is the Master.

Example `hosts.txt` for 3 parties running locally:
```
127.0.0.1:9000
127.0.0.1:9001
127.0.0.1:9002
```

### 2. Initialization
Call `DeMultiNet::init_from_file` with the path to the hosts file and the party ID of the current process.

```rust
use deNetwork::{DeMultiNet, DeNet};

let party_id = 0; // or 1, 2, etc.
DeMultiNet::init_from_file("hosts.txt", party_id);
```

### 3. Communication
The primary communication primitives are:

- `send_bytes_to_master(bytes: Vec<u8>) -> Option<Vec<Vec<u8>>>`:
    - Workers send `bytes`. Returns `None`.
    - Master calls this with its own data. Returns `Some(Vec<Vec<u8>>)` containing data from *all* parties (ordered by party ID).

- `recv_bytes_from_master(bytes: Option<Vec<Vec<u8>>>) -> Vec<u8>`:
    - Master provides the data to send to each worker (a `Vec` where index `i` is for party `i`). Returns its own share.
    - Workers pass `None`. Returns the data sent by the master.

- `master_compute(bytes: Vec<u8>, f: impl Fn(Vec<Vec<u8>>) -> Vec<Vec<u8>>)`:
    - A helper that combines sending to master, master computing a function `f` on all inputs, and master sending results back.

## Example

An example is provided in `examples/demo.rs`. To run it, you need to spawn multiple processes.

### Running the Example

1. Create a `hosts.txt` file as shown above.
2. Run the master (Party 0):
   ```bash
   cargo run --example demo hosts.txt 0
   ```
3. Run the workers (Party 1 and 2) in separate terminals:
   ```bash
   cargo run --example demo hosts.txt 1
   cargo run --example demo hosts.txt 2
   ```

Or use a script to run them all:

```bash
#!/bin/bash
# Build first
cargo build --example demo

# Run in background
target/debug/examples/demo hosts.txt 1 &
target/debug/examples/demo hosts.txt 2 &
sleep 1
target/debug/examples/demo hosts.txt 0 &

wait
```
