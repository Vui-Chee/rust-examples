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
