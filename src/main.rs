use std::path::Path;
use std::io::{stdin, Read};
use std::fs::File;
use std::str;

use log;
use base64;
use tree_magic_mini;
use urlencoding::encode as urlencode;
use color_eyre::eyre::{Result};
use clap::{crate_version, crate_authors, crate_description, Clap};
use flexi_logger::Logger;


#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
struct Opts {
    #[clap(about = "Input file, pass \"-\" for standard input.")]
    infile: String,
    #[clap(short, long, about = "Prefer percent encoding for text file.")]
    text: bool,
    #[clap(short, long, parse(from_occurrences), about = "Verbosity level (repeat to increase).")]
    verbose: i8,
}


fn read_input(infile: &str) -> Result<Vec<u8>> {
    match infile {
        "-" => {
            let mut buf = Vec::<u8>::new();
            let mut stdin = stdin();
            stdin.read_to_end(&mut buf)?;
            Ok(buf)
        },
        _ => {
            let path = Path::new(infile);
            let mut buf = Vec::<u8>::new();
            File::open(path)?.read_to_end(&mut buf)?;
            Ok(buf)
        }
    }
}


fn encode(content: Vec<u8>, mtype: &str, prefer_percent: bool) -> Result<String> {
    let mut body = String::new();
    if prefer_percent && mtype.starts_with("text/") {
        body = urlencode(str::from_utf8(&content)?)
    } else {
        base64::encode_config_buf(&content, base64::URL_SAFE, &mut body)
    }
    Ok(body)
}


fn main() -> Result<()> {
    let opts = Opts::parse();
    let level = match opts.verbose {
        0 => "warning",
        1 => "info",
        _ => "debug"
    };
    Logger::try_with_str(level)?.start()?;
    log::debug!("Process file {}", opts.infile);
    let infile = opts.infile.trim();
    let content = read_input(infile)?;
    let mtype = tree_magic_mini::from_u8(&content);
    // Note: "content" is moved to encode() to be dropped early
    let body = encode(content, mtype, opts.text)?;
    log::info!("MIME type: {}", mtype);
    let encoding = if opts.text && mtype.starts_with("text/") { "" } else { "base64" };
    print!("data:{};{},{}", mtype, encoding, body);
    Ok(())
}
