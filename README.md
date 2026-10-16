PreTxSim - Privacy Virtual Hackathon / zkVerify (ZK Simulation)

**Adapter:** zk -> ZkAdapter
**Hackathon:** Privacy Virtual Hackathon / zkVerify Foundation
**Track:** ZK / privacy dApps
**Repo:** Skywalkingzulu1/pretxsim_zk

PreTxSim is an air-gapped local EVM simulation proxy (Rust, revm). This
repository contains the single zk adapter, vendored with the shared
Adapter core, so it builds and tests as an independent crate.

## What it does

PreTxSim - Privacy Virtual Hackathon / zkVerify (ZK Simulation) runs a pure structural inspection of a transaction payload and
returns ALLOW / WARN / BLOCK with reasons, without broadcasting or
exposing keys to the network.

## Build and test

cargo build
cargo test

## Integration

Wire the adapter into your own binary via AdapterRegistry:

use pretxsim_*::adapters::{Adapter, AdapterRegistry};
use pretxsim_*::zk::__TYPE;

let reg = AdapterRegistry::new().register(Arc::new(ZkAdapter));
let result = reg.inspect("zk", &tx).expect("adapter registered");

See the parent preTxSim repo (Skywalkingzulu1/pretxsim_poc) for the
full --target/--ci/--sim-file CLI and the agent.md multi-agent
coordination board.

## License

Proof of concept. License TBD.
