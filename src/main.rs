#[allow(unused_imports)]
use clap::{Arg, ArgAction, Command, Parser};
use ipnetwork::IpNetwork;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    range: String
}

fn main() {
    let args = Arguments::parse();
    let range = &args.range;
    let network: IpNetwork = IpNetwork::from_str(range).expect("Invalid CIDR notation");

    let start = network.network().clone();
    let end = network.broadcast().clone();

    println!("IP Addresses in range {} - {}:", start, end);
}