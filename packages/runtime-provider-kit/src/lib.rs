//! Shared MCP runtime-provider server. Runtime, durable storage and gate are supplied by callers.
pub mod backend;
mod channel;
pub mod names;
pub mod schemas;
mod server;
mod transport;
pub use backend::*;
pub use server::{Options, RuntimeServer, drain_owed_stops};
