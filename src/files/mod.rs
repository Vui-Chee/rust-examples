use std::fs;
use std::io::Result;

/// Return n biggest nodes within
/// src directory.
pub fn largest_files(src: &str, n: usize) -> Result<Vec<fs::DirEntry>> {
    // Do not proceed unless path leads to a directory.
    if !fs::metadata(src)?.is_dir() {
        return Ok(vec![]);
    }

    let mut entries: Vec<_> = fs::read_dir(src).unwrap().map(|r| r.unwrap()).collect();
    entries.sort_by(|file1, file2| {
        let size1 = fs::metadata(file1.path()).unwrap().len();
        let size2 = fs::metadata(file2.path()).unwrap().len();
        return size2.partial_cmp(&size1).unwrap();
    });
    entries.truncate(n);

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::largest_files;
    use std::env::current_dir;

    #[test]
    fn test_largest_files() {
        let test_dir = format!(
            "{}{}",
            current_dir().unwrap().to_str().unwrap(),
            "/src/files/test_files"
        );
        let res = largest_files(&test_dir, 0);
        assert_eq!(res.unwrap().len(), 0);
        let res = largest_files(&test_dir, 3);
        assert_eq!(res.unwrap().len(), 3);
        let res = largest_files(&test_dir, 5);
        assert_eq!(res.unwrap().len(), 3);
        let res = largest_files(&test_dir, 3);
        if let Ok(entries) = res {
            let entries_slice = entries.as_slice();
            assert_eq!(entries_slice[0].file_name(), "file1.txt");
            assert_eq!(entries_slice[1].file_name(), "file3.txt");
            assert_eq!(entries_slice[2].file_name(), "file2.txt");
        }
    }
}
