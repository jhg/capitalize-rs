#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(iter_intersperse))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

// NOTE: Why mainly use iterator methods? In the end we will copy each char,
// doing it in that way we avoid use indexing (a[b]). To avoid compiler check
// bounds and possible panic. It must be compared with benchmarks to ensure
// that speed is not affected or optimize that.

mod iter;
use iter::CapitalizeIter;

/// It's implemented for all types that implement [`AsRef<str>`].
pub trait Capitalize: AsRef<str> {
    /// First character to title case and the rest to lower case.
    /// This means that characters like digraphs will only have
    /// their first letter capitalized, instead of the full character.
    ///
    /// Behavior is like [Python's `str.capitalize`]. Also, it uses
    /// [`char::to_uppercase`] under the hood, then read its doc.
    /// That relies on Unicode to change to upper case.
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

    /// Split by space into words, then for each word change the first
    /// character to title case and the rest to lower case.
    /// This means that characters like digraphs will only have
    /// their first letter capitalized, instead of the full character.
    ///
    /// It uses [`char.to_uppercase()`] under the hood, then read its doc.
    /// That relies on Unicode to change to upper case.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use capitalize::Capitalize;
    /// assert_eq!("✨ hello world".capitalize_words(), "✨ Hello World");
    /// assert_eq!("ñandu".capitalize_words(), "Ñandu");
    /// assert_eq!("こんにちは世界".capitalize_words(), "こんにちは世界");
    /// ```
    #[cfg(feature = "nightly")]
    #[doc(cfg(feature = "nightly"))]
    fn capitalize_words(&self) -> String;

    /// First character to upper case and the rest will remain the same.
    ///
    /// It uses [`char.to_uppercase()`] under the hood, then read its doc.
    /// That relies on Unicode to change to upper case.
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
    /// It uses [`char.to_uppercase()`] under the hood, then read its doc.
    /// That relies on Unicode to change to upper case.
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
        let string = self.as_ref();

        let mut buf = String::with_capacity(string.len());
        buf.extend(string.chars().capitalize());

        return buf;
    }

    #[cfg(feature = "nightly")]
    fn capitalize_words(&self) -> String {
        if self.as_ref().is_empty() {
            return String::with_capacity(0);
        }
        self.as_ref()
            .split(" ")
            .intersperse(" ")
            .map(|item| {
                let mut chars = item.chars();
                chars
                    .next()
                    .into_iter()
                    .flat_map(char::to_uppercase)
                    .chain(chars.flat_map(char::to_lowercase))
            })
            .flatten()
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
