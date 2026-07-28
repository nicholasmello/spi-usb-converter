use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, conflicts_with = "mode", conflicts_with = "speed")]
    ping: bool,

    #[arg(short, long, value_enum, default_value_t = Mode::Mode0, help = "SPI Mode")]
    mode: Mode,

    #[arg(short, long, default_value_t = 10000, help = "SPI Frequency in HZ")]
    speed: u32,

    data: String,
}

fn main() {
    let args = Args::parse();
    let data = args
        .data
        .strip_prefix("0x")
        .or_else(|| args.data.strip_prefix("0X"))
        .unwrap_or(&args.data);

    let bytes = hex::decode(data).expect("invalid hex");

    println!("{bytes:?}");
}
