use std::fs::read_to_string;
use std::path::PathBuf;
use std::io::Write;

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
            let readdirs = dir_path.expect(
                "Returns an error if Path does not exist or does not have permission to access",
            );
            for entry in readdirs {
                let dir_file = entry.expect("Not found files in directory");
                dir_file.path();
            }
        }
    }

    /// Returns the number of elements in an array
    pub fn take_file_vec_len(&self) -> usize {
        self.files.len()
    }

    /// Returns the PathBuf in the array
    pub fn take_file_from_vec(&self) {
        for file_iter in self.files.iter() {
            println!("{:?}", file_iter);
        }
    }

    /// Returns the Directory and Path
    pub fn dict_and_path_vec(&self) {
        if let Some(dir) = &self.directory {
            let mut dir_path = &dir;
            let file_path = self.files;

            for i in 0..=self.take_file_vec_len() {
                dir_path.push(file_path[i].as_path());
            }

        }
    }
}

pub struct Fileread {}

impl Fileread {
    /// Reads the file and returns the contents
    pub fn read_file(path: PathBuf) -> String {
        read_to_string(path).expect("Error reading file")
    }
}

pub struct FileWrite {}

impl FileWrite {
    pub fn write_file<T: AsRef<u8>>(&mut self,content:T) -> () {
        Write::write_all(content.as_ref());
    }
}
