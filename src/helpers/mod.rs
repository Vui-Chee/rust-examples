use std::fmt::Display;
use std::vec::IntoIter;

pub trait ExtendedIterator {
    type Item;

    fn mk_string(&mut self, start: &str, separator: &str, end: &str) -> String;
    fn grouped(&mut self, size: usize) -> IntoIter<IntoIter<Self::Item>>;
}

impl<O> ExtendedIterator for O
where
    O: Iterator,
    O::Item: Display + Clone,
{
    type Item = O::Item;

    fn mk_string(&mut self, start: &str, separator: &str, end: &str) -> String {
        format!(
            "{}{}{}",
            start,
            self.enumerate()
                .map(|(index, val)| {
                    if index > 0 {
                        format!("{}{}", separator, val.to_string())
                    } else {
                        format!("{}", val.to_string())
                    }
                })
                .collect::<String>(),
            end
        )
    }

    fn grouped(&mut self, size: usize) -> IntoIter<IntoIter<Self::Item>> {
        if size <= 0 {
            return vec![].into_iter();
        }

        let mut result = Vec::<IntoIter<Self::Item>>::new();
        let mut temp = Vec::<Self::Item>::new();
        for (index, value) in self.enumerate() {
            if index > 0 && index % size == 0 {
                result.push(temp.into_iter());
                temp = Vec::<Self::Item>::new();
            }
            temp.push(value);
        }
        result.push(temp.into_iter());
        result.into_iter()
    }
}

#[cfg(test)]
mod helper_test {
    use super::*;

    #[test]
    fn test_mk_string() {
        let arr: [u8; 3] = [1, 2, 3];
        let output_str = arr.iter().mk_string("<", "-", ">");
        assert_eq!(output_str, "<1-2-3>");
    }

    #[test]
    fn test_grouped() {
        let arr: [u8; 5] = [1, 2, 3, 4, 5];
        let groups = arr.iter().grouped(2);
        assert_eq!(groups.len(), 3);
        assert_eq!(groups.as_slice()[0].len(), 2);
        assert_eq!(groups.as_slice()[1].len(), 2);
        assert_eq!(groups.as_slice()[2].len(), 1);
    }
}
