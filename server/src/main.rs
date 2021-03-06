#![feature(test)]
#![feature(hash_raw_entry)]

use crate::server::Server;
use crate::simple_sublist::SimpleSubList;
use crate::sublist::TrieSubList;
use std::error::Error;
use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod client;
mod error;
mod parser;
mod server;
mod simple_sublist;
mod sublist;
mod sublist2;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("server start..");
    let s: Server<TrieSubList> = Server::default();
    s.start().await
}
