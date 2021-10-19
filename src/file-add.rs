pub mod encode {
    use std::fs;
    use std::io;
    use std::path::PathBuf;

    use zstd::stream::encode_all;

    pub const SUFFIX: &'static str = ".zst";

    pub struct Zstds {
        pub levels: i32,
        pub source: &'static str,
    }

    impl Zstds {
        pub fn zstd_use(&self) -> io::Result<()> {
            let mut file = fs::File::open(self.source)?;
            let mut encoder = {
                let target = fs::File::create(self.source.to_string() + SUFFIX)?;
                zstd::Encoder::new(target, self.levels)?
            };

            io::copy(&mut file, &mut encoder)?;
            encoder.finish()?;

            Ok(())
        }
    }
}
