#![warn(missing_docs, rust_2018_idioms, missing_debug_implementations)]

pub struct StringSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl StringSplit<'_> {
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl Iterator for StringSplit<'_> {
    type Item = &str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO: bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StringSplit::new(haystack, " ");

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}
