use std::str::CharIndices;
use std::io::{Result, Write};

struct WordsIter<'a> {
    data: &'a str,
    iter: CharIndices<'a>,
}

impl<'a> WordsIter<'a> {
    fn new(data: &'a str) -> WordsIter<'a> {
        WordsIter {
            data,
            iter: data.char_indices(),
        }
    }
}

// to return word by word, ignore \t, \n, \r and extra spaces
impl<'a> Iterator for WordsIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let word_start;
        loop {
            let (idx, ch) = match self.iter.next() {
                None => return None,
                Some(idx_ch) => idx_ch,
            };
            match ch {
                ' ' | '\t' | '\r' | '\n' => continue,
                _ => {
                    word_start = idx;
                    break;
                }
            }
        }
        loop {
            let (idx, ch) = match self.iter.next() {
                None => break,
                Some((idx, ch)) => ((idx, ch)),
            };
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    return Some(&self.data[word_start..idx]);
                }
                _ => continue,
            }
        }
        return Some(&self.data[word_start..self.data.len()]);
    }
}

// ?AK? review how other result types are converted to Result type, '?'
pub fn wrap_text(buf: &mut Write, data: &str, width: usize, indent: usize)
    -> Result<()>
{
    let mut witer = WordsIter::new(data);
    let mut off = indent;
    match witer.next() {
        None => {
            return Ok(());
        }
        Some(word) => {
            buf.write(word.as_bytes())?;
            off += word.len();
        }
    }
    for word in witer {
        if off + word.len() + 1 > width {
            buf.write(b"\n")?;
            for _ in 0..indent {
                buf.write(b" ")?;
            }
            off = indent;
        } else {
            buf.write(b" ")?;
            off += 1;
        }
        buf.write(word.as_bytes())?;
        off += word.len();
    }
    return Ok(());
}
