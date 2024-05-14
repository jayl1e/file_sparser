use byte_unit;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::{
    fmt, fs,
    io::{Read, Seek, Write},
    os::unix::fs::MetadataExt,
};
use structopt::{self, StructOpt};

fn parse_bytes(src: &str) -> Result<usize, byte_unit::ParseError> {
    byte_unit::Byte::parse_str(src, false).map(|x| x.as_u64() as usize)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "file sparser", about = "dig hole for sparse file")]
struct Opts {
    #[structopt(short, long)]
    src: String,

    #[structopt(short, long)]
    dst: String,

    #[structopt(long, default_value = "16777216", parse(try_from_str = parse_bytes))]
    block: usize,
}

fn main() {
    let args = Opts::from_args();
    let mut sfile = fs::File::open(&args.src).unwrap();
    let mut dfile = fs::File::create(&args.dst).unwrap();
    let mut buffer = vec![0u8; args.block];
    let mut copied = 0;
    let total = sfile.metadata().unwrap().size();
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));
    loop {
        match sfile.read(&mut buffer) {
            Ok(0) => {
                drop(dfile);
                drop(sfile);
                pb.finish_with_message("success");
                println!("success");
                break;
            }
            Ok(readed) => {
                if buffer.iter().all(|&x| x == 0) {
                    dfile
                        .seek(std::io::SeekFrom::Current(readed as i64))
                        .unwrap();
                } else {
                    dfile.write(&buffer[0..readed]).unwrap();
                }
                copied += readed;
                pb.set_position(copied as u64);
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
}
