extern crate percent_encoding;
extern crate clap;

use std::borrow::Borrow;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::iter;

use clap::{Arg, App, ArgMatches};
use percent_encoding as pe;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("urlencode")
        .version(VERSION)
        .author("Skyler Hawthorne <skylerhawthorne@gmail.com>")
        .about("URL-encodes or -decodes the input")
        .arg(Arg::with_name("decode").short("d").long("decode").help(
            "Decode the input, rather than encode.",
        ))
        .arg(
            Arg::with_name("strict-decode")
                .short("s")
                .long("strict-decode")
                .help(
                    "Decode the input non-lossily. If set, the program will fail if it \
                     encounters a sequence that does not produce valid UTF-8.",
                ),
        )
        .arg(
            Arg::with_name("encode-set")
                .short("e")
                .long("encode-set")
                .takes_value(true)
                .possible_values(&["default", "path", "query", "simple", "userinfo"])
                .default_value("default")
                .help(
                    "The encode set to use when encoding. See \
                      https://docs.rs/percent-encoding/1.0.0/percent_encoding/index.html \
                      for more details.",
                ),
        )
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

    let decode_mode = arg_matches.is_present("decode") || arg_matches.is_present("strict-decode");
    let lossy = !arg_matches.is_present("strict-decode");
    let mut buf = String::new();

    while stdin_handle.read_line(&mut buf)? > 0 {
        // use an anonymous scope so `line`'s borrow gets dropped
        // before clearing the buffer
        {
            let line = buf.trim_right();

            if decode_mode {
                decode(line.as_bytes(), &mut stdout_handle, lossy)?;
            } else {
                // Ugh, unfortunately, since EncodeSet : Cloned : Sized, it
                // cannot be boxed, so it's impossible to choose our encode set
                // only once.
                match arg_matches.value_of("encode-set").unwrap() {
                    "default" => encode(&line, pe::DEFAULT_ENCODE_SET, &mut stdout_handle)?,
                    "path" => encode(&line, pe::PATH_SEGMENT_ENCODE_SET, &mut stdout_handle)?,
                    "query" => encode(&line, pe::QUERY_ENCODE_SET, &mut stdout_handle)?,
                    "simple" => encode(&line, pe::SIMPLE_ENCODE_SET, &mut stdout_handle)?,
                    "userinfo" => encode(&line, pe::USERINFO_ENCODE_SET, &mut stdout_handle)?,
                    _ => panic!("Unknown encode set"),
                };
            }
        }

        buf.clear();
    }

    Ok(())
}

fn decode<W: io::Write>(
    line: &[u8],
    output: &mut W,
    lossy: bool,
) -> Result<(), Box<Error + Send + Sync>> {
    let decoder = pe::percent_decode(line);

    let decoded = if lossy {
        decoder.decode_utf8_lossy()
    } else {
        decoder.decode_utf8()?
    };

    let result = write_output(iter::once(decoded.borrow()), output);

    match result {
        Err(e) => Err(Box::new(e)),
        _ => Ok(()),
    }
}

fn encode<W: io::Write, E: pe::EncodeSet>(
    line: &str,
    encode_set: E,
    output: &mut W,
) -> io::Result<()> {
    let encoded = pe::utf8_percent_encode(line, encode_set);
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
