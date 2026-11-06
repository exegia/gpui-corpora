#[allow(unused_imports)]
use super::*;

pub(crate) fn localize_literals(content: &str) -> Vec<(usize, String)> {
    let bytes = content.as_bytes();
    let mut found = Vec::new();
    let mut search_from = 0;
    while let Some(offset) = content[search_from..].find("localize") {
        let start = search_from + offset;
        let mut cursor = start + "localize".len();
        let preceded_by_ident = start > 0 && is_ident(bytes[start - 1]);
        if !preceded_by_ident {
            if content[cursor..].starts_with("_with_args") {
                cursor += "_with_args".len();
            }
            if !bytes.get(cursor).is_some_and(|byte| is_ident(*byte)) {
                while bytes
                    .get(cursor)
                    .is_some_and(|byte| byte.is_ascii_whitespace())
                {
                    cursor += 1;
                }
                if bytes.get(cursor) == Some(&b'(') {
                    cursor += 1;
                    while bytes
                        .get(cursor)
                        .is_some_and(|byte| byte.is_ascii_whitespace())
                    {
                        cursor += 1;
                    }
                    if bytes.get(cursor) == Some(&b'"') {
                        cursor += 1;
                        let mut literal: Vec<u8> = Vec::new();
                        while let Some(byte) = bytes.get(cursor).copied() {
                            cursor += 1;
                            match byte {
                                b'"' => break,
                                b'\\' => {
                                    if let Some(escaped) = bytes.get(cursor).copied() {
                                        literal.push(escaped);
                                        cursor += 1;
                                    }
                                }
                                _ => literal.push(byte),
                            }
                        }
                        found.push((start, String::from_utf8_lossy(&literal).into_owned()));
                    }
                }
            }
        }
        search_from = cursor.max(start + 1);
    }
    found
}
