use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Files {
    pub files: HashSet<PathBuf>,
    pub config: PathBuf,
}

/// Returns a list of all files that should be linted by the linter
///
/// # Arguments
///
/// * `paths` - A vector containing all paths to use as a root
pub fn get_all_files_to_lint(paths: Vec<PathBuf>) -> Files {
    let mut to_lint = HashSet::new();
    let mut cfg_file: PathBuf = PathBuf::new();

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
                    } else if cfg_file == PathBuf::new() {
                        // Obtain config file
                        let cf = get_config_file(None, path_buf);

                        if let Some(cfg) = cf {
                            cfg_file = cfg
                        }
                    }
                }
                Err(_) => todo!(),
            }
        }
    }

    Files {
        files: to_lint,
        config: cfg_file,
    }
}

pub fn get_config_file(config: Option<String>, root: PathBuf) -> Option<PathBuf> {
    match config {
        Some(str) => {
            let path = PathBuf::from(str);

            if path.exists() {
                Some(path)
            } else {
                // Panic in a more useful way
                panic!("Specified config file does not exist.")
            }
        }
        None => {
            let files = [
                ".eslintrc.js",
                ".eslintrc.cjs",
                ".eslintrc.yaml",
                ".eslintrc.yml",
                ".eslintrc.json",
                "package.json",
            ];
            for file in files {
                let path = root.join(PathBuf::from(file));
                if path.exists() {
                    // TODO: if checking for package.json, check for the existence of the appropriate eslint property

                    return Some(path);
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_paths_provided() {
        assert_eq!(
            get_all_files_to_lint(vec![]).files,
            vec![].into_iter().collect()
        );
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

        assert_eq!(get_all_files_to_lint(input).files, expected_output);
    }

    #[test]
    fn one_file() {
        let input = vec![PathBuf::from("./test/src/index.js")];

        let expected_output: HashSet<PathBuf> = vec![PathBuf::from("./test/src/index.js")]
            .into_iter()
            .collect();

        assert_eq!(get_all_files_to_lint(input).files, expected_output);
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

        assert_eq!(get_all_files_to_lint(input).files, expected_output);
    }
}
