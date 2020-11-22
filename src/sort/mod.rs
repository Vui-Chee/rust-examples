pub mod mergesort;

pub trait Sorter {
    fn sort<T>(&self, arr: &mut [T])
    where
        T: Ord + Copy;
}
