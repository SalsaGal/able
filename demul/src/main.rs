use std::{fs::File, io::Write, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// The `mul` file to read from.
    input: PathBuf,
    /// The output directory
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mul_file = std::fs::read(&args.input).unwrap();

    let sample_rate = u32::from_le_bytes([mul_file[0], mul_file[1], mul_file[2], mul_file[3]]);
    let channels = u32::from_le_bytes([mul_file[12], mul_file[13], mul_file[14], mul_file[15]]);

    // TODO Use slices instead of `Vec<Vec<>>`s
    let mut body = mul_file[0x800..].iter().copied().enumerate();
    let mut audio_slices = (0..channels).map(|_| vec![]).collect::<Vec<_>>();
    let mut data_slices = vec![];
    let mut audio_chunks = 0;
    let mut padding_chunks = 0;
    while let Some(current_chunk) = Chunk::parse(&mut body) {
        match current_chunk {
            Chunk::Audio { size } => {
                audio_chunks += 1;
                let split_size = size / channels;
                let bytes = get_bytes(&mut body, size as usize);
                for (i, slice) in bytes.chunks(split_size as usize).enumerate() {
                    audio_slices[i].push(slice.to_vec());
                }
            }
            Chunk::Data { size } => data_slices.push(get_bytes(&mut body, size as usize)),
            Chunk::Padding { size } => {
                padding_chunks += 1;
                get_bytes(&mut body, size as usize);
            }
        }
    }

    let project_name = args.input.file_stem().unwrap().to_string_lossy();
    let output_dir = args.output.unwrap_or_else(|| args.input.with_extension(""));
    std::fs::create_dir(&output_dir).unwrap();
    if !audio_slices.is_empty() {
        for (i, slices) in audio_slices.iter().enumerate() {
            let mut out = File::create(format!(
                "{}/{}_audio_ch{i}.bin",
                output_dir.to_string_lossy(),
                project_name
            ))
            .unwrap();
            out.write_all(
                &slices
                    .iter()
                    .flatten()
                    .copied()
                    .map(|(_, x)| x)
                    .collect::<Vec<_>>(),
            )
            .unwrap();
        }
    }
    if !data_slices.is_empty() {
        let mut out = File::create(format!(
            "{}/{}_data.bin",
            output_dir.to_string_lossy(),
            project_name
        ))
        .unwrap();
        out.write_all(
            &data_slices
                .iter()
                .flatten()
                .copied()
                .map(|(_, x)| x)
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }

    let mut rate_file = File::create(format!(
        "{}/{project_name}_rate.txt",
        output_dir.to_string_lossy()
    ))
    .unwrap();
    for i in 0..channels {
        write!(
            &mut rate_file,
            "{project_name}_audio_ch{i}.bin 1 {sample_rate} 0 16\r\n",
        )
        .unwrap();
    }

    println!("MUL file");
    println!("Audio channels: {channels}");
    println!("Audio sample rate: {sample_rate}");
    println!(
        "Total chunks: {}",
        data_slices.len() + audio_chunks + padding_chunks
    );
    println!("Data chunks: {}", data_slices.len());
    println!("Audio chunks: {audio_chunks}");
    println!("Padding chunks: {padding_chunks}");
}

enum Chunk {
    Audio { size: u32 },
    Data { size: u32 },
    Padding { size: u32 },
}

impl Chunk {
    fn parse(bytes: &mut impl Iterator<Item = (usize, u8)>) -> Option<Self> {
        let variant = parse_u32(bytes)?;
        match variant {
            0 => {
                let audio = Self::Audio {
                    size: parse_u32(bytes)? - 16,
                };
                get_bytes(bytes, 8 + 16);
                Some(audio)
            }
            1 => {
                let data = Self::Data {
                    size: parse_u32(bytes)?,
                };
                get_bytes(bytes, 8);
                Some(data)
            }
            2 => {
                let data = Self::Padding {
                    size: parse_u32(bytes)?,
                };
                get_bytes(bytes, 8);
                Some(data)
            }
            _ => panic!("Unsupported variant: {variant}"),
        }
    }
}

fn parse_u32(bytes: &mut impl Iterator<Item = (usize, u8)>) -> Option<u32> {
    Some(u32::from_le_bytes([
        bytes.next()?.1,
        bytes.next()?.1,
        bytes.next()?.1,
        bytes.next()?.1,
    ]))
}

fn get_bytes(bytes: &mut impl Iterator<Item = (usize, u8)>, count: usize) -> Vec<(usize, u8)> {
    (0..count).map(|_| bytes.next().unwrap()).collect()
}
