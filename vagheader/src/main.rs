use std::{fs::File, io::Write, num::NonZeroU32, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[clap(version)]
struct Args {
    input: PathBuf,
    sample_rate: NonZeroU32,
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let file = std::fs::read(&args.input).unwrap();
    let file_len = file.len();

    let mut output = File::create(
        args.output
            .unwrap_or_else(|| args.input.with_extension("vag")),
    )
    .unwrap();

    output
        .write_all(
            &[
                [0x56, 0x41, 0x47, 0x70],
                [0x0, 0x0, 0x0, 0x20],
                [0; 4],
                (file_len as u32).to_be_bytes(),
                (args.sample_rate.get().to_be_bytes()),
                [0; 4],
                [0; 4],
                [0; 4],
                [0; 4],
                [0; 4],
                [0; 4],
                [0; 4],
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        )
        .unwrap();
    output.write_all(&file).unwrap();
}