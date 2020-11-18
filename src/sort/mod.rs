pub mod mergesort;

pub trait Sorter {
    fn sort<T>(arr: &mut [T])
    where
        T: Ord + Copy;
}

pub fn sort<T, S>(arr: &mut [T])
where
    T: Ord + Copy,
    S: Sorter,
{
    S::sort(arr);
}
