pub mod mergesort;
pub mod quicksort;

pub trait Sorter {
    fn sort<T>(&self, arr: &mut [T])
    where
        T: Ord + Copy;
}
