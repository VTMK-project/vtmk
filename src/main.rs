use fileadd::encode;
fn main() -> Result<(), std::io::Error> {
    let ank = encode::Zstds {
        levels: 3,
        source: "D:\\vtmk\\test\\test.txt",
    };

    ank.zstd_use()
}
