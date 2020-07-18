#[allow(dead_code)]
pub fn merge<T>(arr: &mut [T], s1: usize, s2: usize, end: usize)
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

/// Merge sort in place
#[allow(dead_code)]
pub fn merge_sort<T>(arr: &mut [T], start: usize, end: usize)
where
    T: Ord + Copy,
{
    if start >= end {
        return;
    }

    let mid = (start + end) / 2;
    merge_sort(arr, start, mid);
    merge_sort(arr, mid + 1, end);
    merge::<T>(arr, start, mid + 1, end);
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[quickcheck]
    fn test_merge_sort_with_random_arr(xs: Vec<isize>) -> bool {
        let mut arr = xs.to_owned();
        let size = arr.len();

        // Cannot run on empty array - like what for?
        if size <= 0 {
            return true;
        }

        merge_sort(&mut arr, 0, size - 1);

        // Check ordering.
        for i in 0..size - 1 {
            if arr[i] > arr[i + 1] {
                return false;
            }
        }

        true
    }
}
