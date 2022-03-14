#![doc = include_str!("../README.md")]

pub trait Capitalize: AsRef<str> {
    /// Change first character to upper case and the rest to lower case.
    fn capitalize(&self) -> String;
}

impl<T: AsRef<str>> Capitalize for T {
    fn capitalize(&self) -> String {
        let mut chars = self.as_ref().chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first
                .to_uppercase()
                .chain(chars.map(|c| c.to_ascii_lowercase()))
                .collect(),
        }
    }
}
