//! PreTxSim adapter for Privacy Virtual Hackathon / zkVerify Foundation - ZK / privacy dApps.
//!
//! Air-gapped structural inspection for the Privacy Virtual Hackathon / zkVerify Foundation hackathon.
//! Exposes the ZkAdapter adapter plus the shared Adapter core.

pub mod adapters;
pub use adapters::{Adapter, AdapterResult, AdapterVerdict};
pub use adapters::zk::ZkAdapter;
