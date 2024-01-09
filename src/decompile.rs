

use anyhow::{bail, Context, Result};


use crate::{get_tok_map, TibToken};

fn next_chunk<I: Iterator<Item = u8>>(iter: &mut I, sz: usize) -> Result<Box<[u8]>> {
    let mut buf = Vec::<u8>::new();

    for _i in 0..sz {
        let next = iter.next().with_context(|| "Unexpected end of buffer")?;
        buf.push(next);
    }

    Ok(buf.into_boxed_slice())
}

pub fn decompile(raw: Box<[u8]>) -> Result<Vec<TibToken>> {
    let tok_map = get_tok_map();
    let mut iter = raw.iter().copied();
    let header = next_chunk(&mut iter, 8)?;

    if String::from_utf8(header.into_vec())?.as_str() != "**TI83F*" {
        bail!("Invalid header");
    }

    next_chunk(&mut iter, 3)?; // ext sig
    next_chunk(&mut iter, 42)?; // comment
    next_chunk(&mut iter, 2)?; // data section len

    next_chunk(&mut iter, 2)?; // data section header

    let sz: u16 = {
        let [a, b] = *next_chunk(&mut iter, 2)? else {
            bail!("Expected 2 bytes");
        };

        (((b as u16) << 8) | a as u16).try_into()?
    };

    next_chunk(&mut iter, 1)?; // var section type
    next_chunk(&mut iter, 8)?; // var section name
    next_chunk(&mut iter, 2)?; // version and flag
    next_chunk(&mut iter, 2)?; // duplicate of `sz`

    let data = next_chunk(&mut iter, sz.into())?;
    let mut toks = Vec::<TibToken>::new();
    let mut i = 2; // skip first 2 bytes of prgm section (header info)

    while i < sz {
        // attempt token lookup for 2-byte tokens

        if i + 1 < sz {
            let tok = tok_map.values().filter(|tok| tok.size == 2).find(|tok| {
                tok.hex_str
                    == format!(
                        "0x{:02X}{:02X}",
                        data[usize::from(i + 1)],
                        data[usize::from(i)]
                    )
            });

            if let Some(tok) = tok {
                toks.push(tok.clone());
                i += 2; // consume the next byte as well
                continue;
            }
        }

        // attempt token lookup for single-byte tokens

        let tok = tok_map
            .values()
            .filter(|tok| tok.size == 1)
            .find(|tok| tok.hex_str == format!("0x{:02X}", data[usize::from(i)]));

        if let Some(tok) = tok {
            toks.push(tok.clone());
            i += 1;
        } else {
            bail!(
                "Could not find token for byte: 0x{:02X}",
                data[usize::from(i)]
            );
        }
    }

    Ok(toks)
}
