pub mod zipping {
    use zstd_safe::{
        compress_using_cdict, create_cdict, max_c_level, CCtx, CDict, CompressionLevel, SafeResult,
        WriteBuf,
    };
    use std::fs::File;
    use std::path::PathBuf;

    use crate::reading::{FileInfo,Fileread};

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

    pub struct ZstdCompress {}

    pub trait Compress {
        fn parse_code(code: usize) -> SafeResult;

        fn compress<C: WriteBuf + ?Sized>(content: String, config: ZstdConfig) -> SafeResult;

        fn compress_using_dict(
            cctx: &mut CCtx<'_>,
            content: String,
            dict_file: &CDict<'_>,
        ) -> SafeResult;

        fn create_dict(dict_buffer: &[u8], config: ZstdConfig) -> CDict<'static>;

        fn new(config: ZstdConfig, files: PathBuf, compress_file_name: String) -> anyhow::Result<File>;
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
        fn compress<C: WriteBuf + ?Sized>(content: String, config: ZstdConfig) -> SafeResult {
            let mut buffer: [u8; 255] = [0; 255];
            let code = zstd_safe::compress(&mut buffer, content.as_bytes(), config.levels);
            code
        }

        /// Function to compress the content using a dictionary
        fn compress_using_dict(
            cctx: &mut CCtx<'_>,
            content: String,
            dict_file: &CDict<'_>,
        ) -> SafeResult {
            let mut buffer: [u8; 255] = [0; 255];
            compress_using_cdict(cctx, &mut buffer, content.as_bytes(), dict_file)
        }

        /// Function to create a dictionary
        fn create_dict(dict_buffer: &[u8], config: ZstdConfig) -> CDict<'static> {
            create_cdict(dict_buffer, config.levels)
        }


        fn new(config: ZstdConfig,files: PathBuf,compress_file_name: String) -> anyhow::Result<File> {
            if config.make_dictionary_file == true {
                let file_content = Fileread::read_file(files);
                let dictionary = Self::create_dict(file_content.as_bytes(),config);
                let cctx = zstd_safe::CCtx::create();
                let compress = Self::compress_using_dict(&mut cctx,file_content,&dictionary);
                let file_full_name = compress_file_name+&EXTENSION.to_string();
                let mut writefile = match compress {
                    Ok(content) => match File::create(file_full_name.as_ref()) {
                        Ok(file) => todo!(),
                        Err(why) => todo!()
                    },
                    Err(err_code) => todo!()
                };
           } else {
               todo!() 
           } 
        }
    }
}
