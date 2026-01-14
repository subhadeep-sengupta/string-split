// !
#![warn(rust_2018_idioms)]
#[derive(Debug)]
pub struct StringSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> StringSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, D> Iterator for StringSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delim = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delim)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, self.len_utf8()))
    }
}

fn until_chars(s: &str, c: char) -> &str {
    StringSplit::new(s, c)
        .next()
        .expect("StringSplit always give some result")
}

#[test]
fn until_chars_test() {
    assert_eq!(until_chars("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StringSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}
#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StringSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
