use std::io::{stdin, Read};
use std::path::Path;
use std::str::Utf8Error;

use clap::Parser;
use flexi_logger::Logger;
use urlencoding::encode as urlencode;
use clap_verbosity_flag::Verbosity;
use base64::{Engine as _, engine::general_purpose};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Input file, pass "-" for standard input.
    infile: String,
    /// Prefer percent encoding for text file.
    #[arg(short, long)]
    text: bool,
    #[command(flatten)]
    verbose: Verbosity,
}

fn read_input(infile: &str) -> std::io::Result<Vec<u8>> {
    match infile {
        "-" => {
            let mut buf = Vec::<u8>::new();
            let mut stdin = stdin();
            stdin.read_to_end(&mut buf)?;
            Ok(buf)
        }
        _ => {
            let path = Path::new(infile);
            std::fs::read(path)
        }
    }
}

fn encode(content: Vec<u8>, mtype: &str, prefer_percent: bool) -> Result<String, Utf8Error> {
    if prefer_percent && mtype.starts_with("text/") {
        Ok(urlencode(std::str::from_utf8(&content)?).to_string())
    } else {
        Ok(general_purpose::URL_SAFE.encode(&content))
    }
}

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();
    let l = opts.verbose.log_level_filter();
    Logger::try_with_str(l.as_str())?.start()?;
    color_eyre::install()?;
    log::debug!("Process file {}", opts.infile);
    let infile = opts.infile.trim();
    let content = read_input(infile)?;
    let mtype = tree_magic_mini::from_u8(&content);
    // Note: "content" is moved to encode() to be dropped early
    let body = encode(content, mtype, opts.text)?;
    log::info!("MIME type: {}", mtype);
    let encoding = if opts.text && mtype.starts_with("text/") {
        ""
    } else {
        "base64"
    };
    print!("data:{};{},{}", mtype, encoding, body);
    Ok(())
}
