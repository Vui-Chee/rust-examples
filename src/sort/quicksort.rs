use rand::prelude::*;

use super::Sorter;

pub struct QuickSort;

impl QuickSort {
    fn partition<T>(
        &self,
        slice: &mut [T], // Full slice
        pivot_index: usize,
        start: usize,
        end: usize,
    ) -> Option<usize>
    where
        T: Ord,
    {
        if start > end || pivot_index < start || pivot_index > end {
            return None;
        }

        // Move pivot to front of slice.
        slice.swap(start, pivot_index);

        let mut boundary = start + 1; // start of right
        for i in start + 1..end + 1 {
            if slice[i] <= slice[start] {
                slice.swap(i, boundary);
                boundary += 1;
            }
        }

        // Move pivot back to its final place
        slice.swap(start, boundary - 1);

        Some(boundary - 1) // Pivot index after partition
    }

    pub fn quicksort_helper<T>(&self, slice: &mut [T], start: usize, end: usize)
    where
        T: Ord,
    {
        if start < end {
            let mut rng = rand::thread_rng();
            let pivot_index = rng.gen_range(start, end);
            if let Some(boundary) = self.partition(slice, pivot_index, start, end) {
                if boundary > 0 {
                    self.quicksort_helper(slice, start, boundary - 1);
                }
                self.quicksort_helper(slice, boundary + 1, end);
            }
        }
    }
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        if slice.len() > 0 {
            self.quicksort_helper(slice, 0, slice.len() - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::super::Sorter;
    use super::QuickSort;

    pub fn random(start: usize, limit: usize, default: usize) -> usize {
        let mut rng = rand::thread_rng();
        if limit > 1 {
            if let Some(result) = (limit - 1).checked_sub(start) {
                if result > 1 {
                    return rng.gen_range(start, limit);
                }
            }
        }
        return default;
    }

    #[quickcheck]
    fn partition(xs: Vec<isize>) -> bool {
        let mut arr = xs.clone();
        if xs.len() > 0 {
            let pivot_index = random(0, arr.len(), 0);
            let pivot = arr[pivot_index];

            // Generate random start and end indices.
            let start = random(0, arr.len(), 0);
            let end = random(arr.len() - 1, arr.len(), 0);

            if let Some(boundary) = QuickSort.partition(&mut arr, pivot_index, start, end) {
                // Check all elems are partitioned according to [ <= pivot | > pivot ]
                for i in start..boundary + 1 {
                    if arr[i] > pivot {
                        return false;
                    }
                }

                for i in boundary + 1..end {
                    if arr[i] <= pivot {
                        return false;
                    }
                }
            } else {
                // Parition() returns None when one of those conditions are met.
                return start > end || pivot_index < start || pivot_index > end;
            }
        }

        true
    }

    #[quickcheck]
    fn quicksort(xs: Vec<isize>) -> bool {
        let mut arr = xs.clone();
        QuickSort.sort(&mut arr);
        arr.iter().enumerate().all(|(i, x)| {
            if i > 0 {
                return arr[i - 1] <= *x;
            }
            true
        })
    }
}
