pub mod zipping {
    use zstd_safe::{max_c_level, CompressionLevel, SafeResult, WriteBuf};

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

    struct ZstdCompress {}

    pub trait Compress {
        fn parse_code(code: usize) -> SafeResult;

        fn compress<C: WriteBuf + ?Sized>(
            &mut self,
            content: String,
            config: ZstdConfig,
        ) -> SafeResult;
    }

    impl Compress for ZstdCompress {
        /// Function to parse code and distinguish if it is an error
        fn parse_code(code: usize) -> SafeResult {
            if !unsafe { zstd_sys::ZSTD_isError(code) != 0 } {
                Ok(code)
            } else {
                Err(code)
            }
        }

        /// Function to compress the content
        fn compress<C: WriteBuf + ?Sized>(
            &mut self,
            content: String,
            config: ZstdConfig,
        ) -> SafeResult {
            let mut buffer: [u8; 256] = [0; 256];
            let code = zstd_safe::compress(&mut buffer, content.as_bytes(), config.levels);
            code
        }
    }
}
