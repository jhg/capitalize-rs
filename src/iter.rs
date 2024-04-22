/// It's implemented for all types that implement [`Iterator<Item=char>`].
pub trait CapitalizeIterator: Iterator<Item = char> {
    fn capitalize(self) -> CapitalizeIter<impl Iterator<Item = char>>;
}

impl<T: Iterator<Item = char>> CapitalizeIterator for T {
    #[inline]
    fn capitalize(mut self) -> CapitalizeIter<impl Iterator<Item = char>> {
        CapitalizeIter {
            // This is required because upper hint is lost
            // after flat_map, to_uppercase or chain calls.
            size_hint: self.size_hint(),
            chars: self
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(self.flat_map(char::to_lowercase)),
        }
    }
}

/// This struct is created with [`CapitalizeIterator::capitalize()`] method.
pub struct CapitalizeIter<T: Iterator<Item = char>> {
    size_hint: (usize, Option<usize>),
    chars: T,
}

impl<T: Iterator<Item = char>> Iterator for CapitalizeIter<T> {
    type Item = char;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint
    }

    #[inline(always)]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.chars.count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn capitalize_size_hint() {
        let text = "0123456789";
        let chars = text.chars();

        let (lower_hint, Some(len)) = chars.size_hint() else {
            panic!("Incorrect chars upper size hint");
        };
        assert_eq!(len, text.len());

        let capitalize = chars.capitalize();

        assert_eq!(capitalize.size_hint(), (lower_hint, Some(text.len())));
    }
}
