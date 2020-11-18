use super::Sorter;

struct MergeSort;

impl MergeSort {
    fn merge<T>(arr: &mut [T], s1: usize, s2: usize, end: usize)
    where
        T: Ord + Copy,
    {
        let mut temp = Vec::<T>::new();

        // Copy first sorted array into temp vector.
        for index in s1..s2 {
            temp.push(arr[index]);
        }

        let mut i = 0;
        let mut j = s2;
        let mut k = s1;
        while i < temp.len() && j <= end {
            if temp[i] < arr[j] {
                arr[k] = temp[i];
                i += 1;
            } else {
                arr[k] = arr[j];
                j += 1;
            }
            k += 1;
        }

        // Copy remaining elements from temp (if any)
        for index in i..temp.len() {
            arr[k] = temp[index];
            k += 1;
        }
    }

    pub fn merge_sort_helper<T>(arr: &mut [T], start: usize, end: usize)
    where
        T: Ord + Copy,
    {
        if start >= end {
            return;
        }

        let mid = (start + end) / 2;
        MergeSort::merge_sort_helper(arr, start, mid);
        MergeSort::merge_sort_helper(arr, mid + 1, end);
        MergeSort::merge::<T>(arr, start, mid + 1, end);
    }
}

impl Sorter for MergeSort {
    fn sort<T>(arr: &mut [T])
    where
        T: Ord + Copy,
    {
        if arr.len() > 0 {
            MergeSort::merge_sort_helper(arr, 0, arr.len() - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::sort;
    use super::MergeSort;

    #[quickcheck]
    fn test_mergesort(xs: Vec<isize>) -> bool {
        let mut arr = xs.to_owned();
        let size = arr.len();

        sort::<_, MergeSort>(&mut arr);

        if size > 0 {
            // Check ordering.
            for i in 0..size - 1 {
                if arr[i] > arr[i + 1] {
                    return false;
                }
            }
        }

        true
    }
}
