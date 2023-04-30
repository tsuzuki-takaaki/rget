use clap::Parser;
use indicatif::{ ProgressBar, ProgressStyle };

use crate::types::args::Args;

mod types;

fn main() {
  let args = Args::parse();
  println!("{:?}", args);
}
