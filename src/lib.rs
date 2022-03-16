#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub trait Capitalize: AsRef<str> {
    /// Only affects Unicode characters equivalent in ASCII.
    /// It's implemented for all types that implement [`AsRef<str>`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("✨ Hello World".capitalize(), "✨ hello world");
    /// ```
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("ăn".capitalize(), "Ăn");
    /// ```
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("ñoque".capitalize(), "Ñoque");
    /// ```
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("こんにちは世界".capitalize(), "こんにちは世界");
    /// ```
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("".capitalize(), "");
    /// ```
    fn capitalize(&self) -> String;
}

impl<T: AsRef<str>> Capitalize for T {
    fn capitalize(&self) -> String {
        let mut chars = self.as_ref().chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first
                .to_uppercase()
                .chain(chars.flat_map(char::to_lowercase))
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Capitalize;

    #[test]
    fn string_reference() {
        let text = String::from("hello ✨ World");
        let text_ref = &text;
        assert_eq!(text_ref.capitalize(), "Hello ✨ world");
    }
}
