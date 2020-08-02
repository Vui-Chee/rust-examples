use std::fs;
use std::io::Result;
use std::path::Path;

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

pub fn sync(src: &str, dest: &str) {
    let entries: Vec<_> = fs::read_dir(src).unwrap().map(|r| r.unwrap()).collect();
    for entry in entries {
        let src_sub_path = entry.path();
        let dest_sub_path = Path::new(dest).join(entry.file_name());
        let src_is_dir = src_sub_path.is_dir();
        let dest_is_dir = dest_sub_path.is_dir();

        if (src_is_dir && !dest_is_dir) || (!src_is_dir && dest_is_dir) {
            fs::copy(src_sub_path, dest_sub_path).unwrap();
        } else if !src_is_dir && !dest_is_dir {
            // Create copied paths for read(...)
            let src_path = src_sub_path.clone();
            let dest_path = dest_sub_path.clone();
            if !dest_path.exists() || (fs::read(src_path).unwrap() != fs::read(dest_path).unwrap())
            {
                fs::copy(src_sub_path, dest_sub_path).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{largest_files, sync};
    use std::env::current_dir;
    use std::fs;
    use std::path::Path;
    use std::str::from_utf8;

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

    #[test]
    fn test_sync() {
        assert!(match current_dir().unwrap().to_str() {
            Some(current_dir) => {
                // Generate test directory
                let dest_path = format!("{}/{}", current_dir, "out");
                let dest = Path::new(&dest_path);
                if !dest.exists() {
                    fs::create_dir(dest).unwrap();
                }

                // First level file
                if !dest.join("folder1").exists() {
                    fs::create_dir(dest.join("folder1")).unwrap();
                }
                fs::write(dest.join("folder1/hello.txt"), "HELLO").unwrap();

                // Nested file (second level)
                if !dest.join("folder1/nested").exists() {
                    fs::create_dir(dest.join("folder1/nested")).unwrap();
                }
                fs::write(dest.join("folder1/nested/world.txt"), "WORLD").unwrap();

                // First sync
                sync(&dest_path, &dest_path);

                assert_eq!(
                    from_utf8(
                        fs::read(format!("{}/{}", dest_path, "folder1/hello.txt"))
                            .unwrap()
                            .as_slice()
                    )
                    .unwrap(),
                    "HELLO"
                );
                assert_eq!(
                    from_utf8(
                        fs::read(format!("{}/{}", dest_path, "folder1/nested/world.txt"))
                            .unwrap()
                            .as_slice()
                    )
                    .unwrap(),
                    "WORLD"
                );

                // Everything succeeded
                true
            }
            None => false,
        });
    }
}
