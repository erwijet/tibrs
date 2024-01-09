# tibrs

A library and CLI for compimling and decompiling TI-BASIC to and from an `8xp` outside of TI-CONNECT.

## Installation
```sh
# add this crate to your project
$ cargo add tibrs

# install `tibc` binary
$ cargo install tibrs --bins
$ tibc --help # confirm installation
```

## Crate Usage

```rust

fn main() -> {
    let source = fs::read_to_string("~/path/to/source.tib").unwrap();
    let tokens: Vec<tibrs::TibToken> = tibrs::parse_str(source).unwrap();
    let buf: Vec<u8> = compile(tokens, "DEMONAME".as_bytes()).unwrap();

    fs::write("~/path/to/DEMONAME.8xp", buf).unwrap();
}

```

## CLI Usage

Note: when compiling `8xp` files, `tibc` will infer the embedded `PRGM` var name from the first 8 letters of the outfile name, capitilized.

```sh
$ cargo install tibrs --bin tibc

# compile
$ tibc --compile --outfile DEMONAME.8xp source.tib

# decompile
$ tibc --decompile --outfile decomp.tib DEMONAME.8xp
```

## References

This binary implementation was based off the highly useful [Merthsoft TI Linkguide](https://merthsoft.com/linkguide/ti83+/fformat.html).