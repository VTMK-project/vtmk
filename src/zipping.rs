pub mod zipping {
    use zstd_safe::{compress, max_c_level, CompressionLevel, SafeResult};

    /// File extension after compression
    pub const EXTENSION: &'static str = ".vlog";

    /// Zstd settings, such as compression level.
    pub struct ZstdConfig {
        /// Supported from 1 to 22.
        pub levels: i32,
        /// Creates a Zstd dictionary file
        /// Dictionary files increase compression efficiency when compressing multiple small files.
        pub make_dictionary_file: bool,
    }

    impl ZstdConfig {
        /// Function to determine if the compression is less than or equal to the maximum level
        pub fn as_comperssionlevel(&self) -> Option<CompressionLevel> {
            if self.levels <= max_c_level() + 1 {
                Some(self.levels)
            } else {
                None
            }
        }
    }

    pub trait Compress {
        fn new(&self);
        fn compress(&self, config: ZstdConfig) -> SafeResult;
    }

    impl dyn Compress {
        fn new(&self) {}
        fn compress(&self, config: ZstdConfig) -> SafeResult {
            todo!()
        }
    }
}
