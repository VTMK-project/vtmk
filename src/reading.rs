use std::path::PathBuf;

/// Information about the file to be compressed
pub struct FileInfo {
    pub files: Vec<PathBuf>,
    pub directory: Option<PathBuf>,
}

impl FileInfo {
    ///Returns the files in the specified directory
    pub fn find_file_dir(&self) {
        if let Some(dir) = &self.directory {
            let dir_path = dir.as_path().read_dir();
            match dir_path {
                Ok(dir) => {
                    for entry in dir {
                        let dir_file = entry.expect("Not found files in directory");
                        dir_file.path();
                    }
                }
                // Returns an error if Path does not exist or does not have permission to access
                Err(e) => println!("{}", e),
            }
        }
    }

    pub fn take_file_from_vec(&self) {}
}
