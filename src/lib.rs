#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub trait Capitalize: AsRef<str> {
    /// First character to upper case and the rest to lower case.
    ///
    /// Only affects Unicode characters equivalent in ASCII.
    /// It's implemented for all types that implement [`AsRef<str>`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use capitalize::Capitalize;
    ///
    /// assert_eq!("hello 🥰 WORLD".capitalize(), "Hello 🥰 world");
    /// assert_eq!("🦄 Hello World".capitalize(), "🦄 hello world");
    /// assert_eq!("".capitalize(), "");
    /// assert_eq!("ăn".capitalize(), "Ăn");
    /// assert_eq!("ñoque".capitalize(), "Ñoque");
    /// assert_eq!("こんにちは世界".capitalize(), "こんにちは世界");
    /// assert_eq!("안녕하세요 세상".capitalize(), "안녕하세요 세상");
    /// assert_eq!("你好世界".capitalize(), "你好世界");
    /// assert_eq!("สวัสดีชาวโลก".capitalize(), "สวัสดีชาวโลก");
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
