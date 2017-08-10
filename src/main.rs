extern crate percent_encoding;
extern crate clap;

use std::borrow::Borrow;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::iter;

use clap::{Arg, App, ArgMatches};
use percent_encoding::{utf8_percent_encode, percent_decode, PATH_SEGMENT_ENCODE_SET};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("urlencode")
        .version(VERSION)
        .author("Skyler Hawthorne <skylerhawthorne@gmail.com>")
        .about("URL-encodes or -decodes the input")
        .arg(Arg::with_name("decode").short("d").long("decode").help(
            "Decode the input, rather than encode",
        ))
        .get_matches();

    if let Err(e) = run(&matches) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run(arg_matches: &ArgMatches) -> Result<(), Box<Error + Send + Sync>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    let mut stdin_handle = stdin.lock();

    let decode_mode = arg_matches.is_present("decode");
    let mut buf = String::new();

    while stdin_handle.read_line(&mut buf)? > 0 {
        // use an anonymous scope so `line`'s borrow gets dropped
        // before clearing the buffer
        {
            let line = &buf[..buf.len() - 1];

            if decode_mode {
                decode(line.as_bytes(), &mut stdout_handle)?;
            } else {
                encode(&line, &mut stdout_handle)?;
            }
        }

        buf.clear();
    }

    Ok(())
}

fn decode<W: io::Write>(line: &[u8], output: &mut W) -> Result<(), Box<Error + Send + Sync>> {
    let decoded = percent_decode(line).decode_utf8()?;
    let result = write_output(iter::once(decoded.borrow()), output);

    match result {
        Err(e) => Err(Box::new(e)),
        _ => Ok(()),
    }
}

fn encode<W: io::Write>(line: &str, output: &mut W) -> io::Result<()> {
    let encoded = utf8_percent_encode(line, PATH_SEGMENT_ENCODE_SET);
    write_output(encoded, output)
}

fn write_output<'a, B, W>(strings: B, output: &mut W) -> io::Result<()>
where
    B: IntoIterator<Item = &'a str>,
    W: io::Write,
{
    for string in strings {
        output.write(string.as_bytes())?;
    }

    output.write("\n".as_bytes())?;

    Ok(())
}
