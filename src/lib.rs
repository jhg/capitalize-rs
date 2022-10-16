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
    /// Only affects Unicode characters equivalent in ASCII.
    /// It's implemented for all types that implement [`AsRef<str>`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("hello World".capitalize_first_only(), "Hello World");
    /// ```
    fn capitalize_first_only(&self) -> String;
    /// Only affects Unicode characters equivalent in ASCII.
    /// It's implemented for all types that implement [`AsRef<str>`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("✨ Hello World".capitalize_last_only(), "✨ Hello WorlD");
    /// ```
    fn capitalize_last_only(&self) -> String;
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

    fn capitalize_first_only(&self) -> String {
        let mut chars = self.as_ref().chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().chain(chars).collect(),
        }
    }

    fn capitalize_last_only(&self) -> String {
        if self.as_ref().is_empty() {
            return String::new();
        }
        let mut str_final = String::new();
        self.as_ref().char_indices().for_each(|(idx, c)| {
            if idx == self.as_ref().len() - 1 {
                let c_cap: String = c.to_uppercase().collect();
                str_final.push_str(&c_cap);
            } else {
                str_final.push(c);
            }
        });
        str_final
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
