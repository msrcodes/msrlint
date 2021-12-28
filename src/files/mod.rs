use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Returns a list of all files that should be linted by the linter
///
/// # Arguments
///
/// * `paths` - A vector containing all paths to use as a root
pub fn get_all_files_to_lint(paths: Vec<PathBuf>) -> HashSet<PathBuf> {
    let mut to_lint = HashSet::new();

    for path in &paths {
        let walker = WalkDir::new(path).into_iter();

        for entry in walker {
            match entry {
                Ok(dir_entry) => {
                    let path_buf = dir_entry.path().to_path_buf();

                    // We can't lint directories, so only lint files
                    if !path_buf.is_dir() {
                        // If extension is valid, use it
                        if let Some(extension) = path_buf.extension() {
                            if extension == "js" || extension == "jsx" {
                                to_lint.insert(path_buf);
                            }
                        }
                    }
                }
                Err(_) => todo!(),
            }
        }
    }

    to_lint
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_paths_provided() {
        assert_eq!(get_all_files_to_lint(vec![]), vec![].into_iter().collect());
    }

    #[test]
    fn standard_run() {
        let input = vec![PathBuf::from("./test")];

        let expected_output: HashSet<PathBuf> = vec![
            PathBuf::from("./test/src/index.js"),
            PathBuf::from("./test/src/myTestDir/anotherTestDir/hidden.js"),
            PathBuf::from("./test/src/myTestDir/another.js"),
        ]
        .into_iter()
        .collect();

        assert_eq!(get_all_files_to_lint(input), expected_output);
    }

    #[test]
    fn one_file() {
        let input = vec![PathBuf::from("./test/src/index.js")];

        let expected_output: HashSet<PathBuf> = vec![PathBuf::from("./test/src/index.js")]
            .into_iter()
            .collect();

        assert_eq!(get_all_files_to_lint(input), expected_output);
    }

    #[test]
    fn multiple_input() {
        let input = vec![
            PathBuf::from("./test/src/index.js"),
            PathBuf::from("./test/src/myTestDir"),
        ];

        let expected_output: HashSet<PathBuf> = vec![
            PathBuf::from("./test/src/index.js"),
            PathBuf::from("./test/src/myTestDir/anotherTestDir/hidden.js"),
            PathBuf::from("./test/src/myTestDir/another.js"),
        ]
        .into_iter()
        .collect();

        assert_eq!(get_all_files_to_lint(input), expected_output);
    }
}
