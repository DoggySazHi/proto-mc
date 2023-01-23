#![crate_type = "lib"]
#![crate_name = "proto_mc"]

pub mod ping;
pub mod query;
pub mod rcon;
mod utilities;

#[cfg(test)]
mod tests;