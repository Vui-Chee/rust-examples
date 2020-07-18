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

    #[quickcheck]
    fn test_grouped(arr: Vec<isize>, group_size: usize) -> bool {
        if group_size <= 0 || arr.len() <= 0 {
            return true;
        }

        let groups = arr.iter().grouped(group_size);

        // Calc number of proper groups to iterate over

        for i in 0..(groups.len() / group_size) {
            if groups.as_slice()[i].len() != group_size {
                return false;
            }
        }

        groups.len() == ((arr.len() + group_size - 1) / group_size)
    }
}
