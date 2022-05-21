# `urlencode`

`urlencode` is a CLI utility for URL-encoding or -decoding strings.

## Usage

You can give it a positional argument for a single string, or you can pipe input
to it from stdin.

```bash
$ urlencode 'foo bar'
foo%20bar

$ echo -e "foo bar\nbaz quux" | urlencode
foo%20bar
baz%20quux
```

You can pass `-d` or `--decode` to decode the input.

```bash
$ urlencode -d 'foo%20bar'
foo bar

$ echo -e "foo%20bar\nbaz%20quux" | urlencode -d
foo bar
baz quux
```

Run `urlencode --help` to see all options.

### Encode sets

Since different parts of a URL have different encoding requirements, there are
many encode sets to choose from. See
[this documentation page](https://docs.rs/percent-encoding/1.0.0/percent_encoding/index.html)
for an explanation of each. They can be specified with the `-e` or `--encode-set`
option:

```bash
$ echo 'https://docs.rs/percent-encoding/1.0.0/percent_encoding/index.html' | urlencode -e path
https:%2F%2Fdocs.rs%2Fpercent-encoding%2F1.0.0%2Fpercent_encoding%2Findex.html

$ echo 'https://docs.rs/percent-encoding/1.0.0/percent_encoding/index.html' | urlencode -e userinfo
https%3A%2F%2Fdocs.rs%2Fpercent-encoding%2F1.0.0%2Fpercent_encoding%2Findex.html
----
```

### Cargo

You can install with Cargo with:

```bash
cargo install urlencode
```