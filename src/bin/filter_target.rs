use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use aetherus_events::{filter_seq, ledger::Ledger};
use aetherus_events::SrcId;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ledger_path = args[1].parse::<PathBuf>().unwrap();

    let file = File::open(ledger_path).expect("Unable to create file");
    let json_data = {
        let mut buf_reader = std::io::BufReader::new(file);
        let mut contents = String::new();
        buf_reader
            .read_to_string(&mut contents)
            .expect("Unable to read ledger file");
        contents
    };
    let ledger: Ledger = serde_json::from_str(&json_data).expect("Unable to parse ledger file");

    let src_id = SrcId::Mat(42);
    let filter_seq = vec![
        filter_seq!(MCRT, Interface, Refraction, SrcId::None),
        filter_seq!(MCRT, Material, Elastic, Mie, Any, src_id),
    ];
    println!("Filter seq: {:?}", filter_seq);
}
