use std::borrow::Borrow;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::iter;

use clap::{Arg, ArgMatches, Command};
use pe::AsciiSet;
use percent_encoding as pe;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod encode_sets;

fn main() {
    let matches = Command::new("urlencode")
        .version(VERSION)
        .author(AUTHORS)
        .about(
            "URL-encodes or -decodes the input. If INPUT is given, it encodes or \
             decodes INPUT, otherwise it takes its input fromt stdin.",
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .help("Decode the input, rather than encode."),
        )
        .arg(
            Arg::new("strict-decode")
                .short('s')
                .long("strict-decode")
                .help(
                    "Decode the input non-lossily. If set, the program will fail if it \
                     encounters a sequence that does not produce valid UTF-8.",
                ),
        )
        .arg(
            Arg::new("encode-set")
                .short('e')
                .long("encode-set")
                .takes_value(true)
                .possible_values(&[
                    "control",
                    "fragment",
                    "query",
                    "squery",
                    "path",
                    "userinfo",
                    "component",
                    "form",
                ])
                .default_value("component")
                .help("The encode set to use when encoding.")
                .long_help(
                    "The encode set to use when encoding. See \
                     https://url.spec.whatwg.org/ \
                     for more details.",
                ),
        )
        .arg(Arg::new("INPUT").help("The string to encode.").index(1))
        .get_matches();

    if let Err(e) = run(&matches) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run(arg_matches: &ArgMatches) -> Result<(), Box<dyn Error + Send + Sync>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    let mut stdin_handle = stdin.lock();
    let encode_set = get_encode_set(arg_matches);

    if arg_matches.is_present("INPUT") {
        let input = arg_matches.value_of("INPUT").unwrap();
        return transform_line(input, &mut stdout_handle, encode_set, arg_matches);
    }

    let mut buf = String::new();

    while stdin_handle.read_line(&mut buf)? > 0 {
        transform_line(buf.trim_end(), &mut stdout_handle, encode_set, arg_matches)?;
        buf.clear();
    }

    Ok(())
}

fn get_encode_set(args: &ArgMatches) -> &'static AsciiSet {
    match args.value_of("encode-set").unwrap() {
        "control" => encode_sets::CONTROLS,
        "fragment" => encode_sets::FRAGMENT,
        "query" => encode_sets::QUERY,
        "squery" => encode_sets::SPECIAL_QUERY,
        "path" => encode_sets::PATH,
        "userinfo" => encode_sets::USERINFO,
        "component" => encode_sets::COMPONENT,
        "form" => encode_sets::FORM,
        _ => panic!("Unknown encode set"),
    }
}

fn transform_line<W: io::Write>(
    line: &str,
    output: &mut W,
    encode_set: &'static AsciiSet,
    arg_matches: &ArgMatches,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let decode_mode = arg_matches.is_present("decode") || arg_matches.is_present("strict-decode");
    let lossy = !arg_matches.is_present("strict-decode");

    if decode_mode {
        decode(line.as_bytes(), output, lossy)
    } else {
        encode(line, encode_set, output)?;
        Ok(())
    }
}

fn decode<W: io::Write>(
    line: &[u8],
    output: &mut W,
    lossy: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
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

fn encode<W: io::Write>(
    line: &str,
    encode_set: &'static AsciiSet,
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
        output.write_all(string.as_bytes())?;
    }

    output.write_all("\n".as_bytes())?;

    Ok(())
}
