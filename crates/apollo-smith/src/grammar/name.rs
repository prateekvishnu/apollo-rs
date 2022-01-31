use arbitrary::Unstructured;

use crate::{Node, SchemaBuilder};

const CHARSET_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const CHARSET_NUMBERS: &[u8] = b"0123456789";

/// See: https://spec.graphql.org/October2021/#Name
///
/// *Name*:
///     [_A-Za-z][_0-9A-Za-z]
pub(crate) fn name(u: &mut Unstructured, builder: &mut SchemaBuilder) -> Node {
    let mut name_str = limited_string(50, u);
    if builder.type_names.contains(&name_str) {
        name_str.push_str(&format!("{}", builder.type_names.len()));
    }
    Node::Name(name_str)
}

// Mirror what happens in `Arbitrary for String`, but do so with a clamped size.
pub(crate) fn limited_string(max_size: usize, u: &mut Unstructured) -> String {
    loop {
        let size = u.arbitrary_len::<u8>().unwrap();
        let size = if size != 0 {
            std::cmp::min(size, max_size)
        } else {
            max_size
        };

        let gen_str = String::from_utf8(
            (0..size)
                .map(|curr_idx| {
                    // TODO fix this by using choose
                    let idx = u.arbitrary::<usize>().unwrap();
                    // Cannot start with a number
                    if curr_idx == 0 {
                        CHARSET_LETTERS[idx % CHARSET_LETTERS.len()]
                    } else {
                        let idx = idx % (CHARSET_LETTERS.len() + CHARSET_NUMBERS.len());
                        if idx < CHARSET_LETTERS.len() {
                            CHARSET_LETTERS[idx]
                        } else {
                            CHARSET_NUMBERS[idx - CHARSET_LETTERS.len()]
                        }
                    }
                })
                .collect::<Vec<u8>>(),
        )
        .unwrap();

        if !gen_str.is_empty() {
            break gen_str;
        }
    }
}

/// See: https://spec.graphql.org/October2021/#Alias
///
/// *Alias*:
///     Name **:**
pub(crate) fn alias(u: &mut Unstructured, builder: &mut SchemaBuilder) -> Node {
    let name = name(u, builder);
    Node::Alias(name.try_into_name().unwrap())
}

fn is_start_char(c: char) -> bool {
    matches!(c, '_' | 'A'..='Z' | 'a'..='z')
}

fn is_remainder_char(c: char) -> bool {
    matches!(c, '_' | 'A'..='Z' | 'a'..='z' | '0'..='9')
}
