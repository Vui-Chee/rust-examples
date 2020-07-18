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

    #[quickcheck]
    fn test_mk_string(arr: Vec<isize>) -> bool {
        let mut expected = String::with_capacity(arr.len() * 2 + 1);

        expected.push('<');
        for (index, &val) in arr.iter().enumerate() {
            if index > 0 {
                expected.push('-');
            }
            expected.push_str(&val.to_string());
        }
        expected.push('>');

        arr.iter().mk_string("<", "-", ">") == expected
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
