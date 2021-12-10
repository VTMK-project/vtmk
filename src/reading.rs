use std::path::PathBuf;

/// Information about the file to be compressed
pub struct FileInfo {
    pub files: Vec<PathBuf>,
    pub directory: Option<PathBuf>,
}

impl FileInfo {
    ///Returns the files in the specified directory
    pub fn find_file_dir(&self) {
        if let Some(file) = &self.directory {
            let dir_path = file.as_path().read_dir();
        }
    }
}
