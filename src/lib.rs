#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// It's implemented for all types that implement [`AsRef<str>`].
pub trait Capitalize: AsRef<str> {
    /// First character to upper case and the rest to lower case.
    /// Behavior is like [Python's `str.capitalize`].
    ///
    /// Only affects Unicode characters equivalent in ASCII.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("✨ Hello World".capitalize(), "✨ hello world");
    /// assert_eq!("ñandu".capitalize(), "Ñandu");
    /// assert_eq!("こんにちは世界".capitalize(), "こんにちは世界");
    /// ```
    ///
    /// [Python's `str.capitalize`]: https://docs.python.org/3/library/stdtypes.html#str.capitalize
    fn capitalize(&self) -> String;

    /// First character to upper case and the rest will remain the same.
    ///
    /// Only affects Unicode characters equivalent in ASCII.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("hello World".capitalize_first_only(), "Hello World");
    /// assert_eq!("✨ hello World".capitalize_first_only(), "✨ hello World");
    /// ```
    fn capitalize_first_only(&self) -> String;

    /// The last character to upper case and the rest will remain the same.
    ///
    /// Only affects Unicode characters equivalent in ASCII.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("✨ Hello World".capitalize_last_only(), "✨ Hello WorlD");
    /// assert_eq!("Hello World ✨".capitalize_last_only(), "Hello World ✨");
    /// assert_eq!("hello world".capitalize_last_only(), "hello worlD");
    /// ```
    fn capitalize_last_only(&self) -> String;
}

impl<T: AsRef<str>> Capitalize for T {
    fn capitalize(&self) -> String {
        let mut chars = self.as_ref().chars();
        let Some(first) = chars.next() else {
            return String::with_capacity(0);
        };
        first
            .to_uppercase()
            .chain(chars.flat_map(char::to_lowercase))
            .collect()
    }

    fn capitalize_first_only(&self) -> String {
        let mut chars = self.as_ref().chars();
        let Some(first) = chars.next() else {
            return String::with_capacity(0);
        };
        first.to_uppercase().chain(chars).collect()
    }

    fn capitalize_last_only(&self) -> String {
        let mut chars = self.as_ref().chars().rev();
        let Some(last) = chars.next() else {
            return String::with_capacity(0);
        };
        last.to_uppercase().chain(chars).rev().collect()
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

    #[test]
    fn capitalize_first_only_reference() {
        let text = String::from("heLLo ✨ World");
        let text_ref = &text;
        assert_eq!(text_ref.capitalize_first_only(), "HeLLo ✨ World");
    }

    #[test]
    fn capitalize_final_only_reference() {
        let text = String::from("heLLo ✨ World");
        let text_ref = &text;
        assert_eq!(text_ref.capitalize_last_only(), "heLLo ✨ WorlD");
    }
}
