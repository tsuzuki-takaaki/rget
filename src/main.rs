use clap::Parser;

use crate::types::args::Args;

mod types;
mod download;

#[tokio::main]
async fn main() {
  let args = Args::parse();
  download::download(&args.url).await.unwrap();
}
