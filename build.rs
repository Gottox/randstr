use std::env;
use std::fs;
use std::path::Path;

fn main() {
    gen_alphabet("lower.txt", |x| x.is_ascii_lowercase());
    gen_alphabet("upper.txt", |x| x.is_ascii_uppercase());
    gen_alphabet("letter.txt", |x| x.is_ascii_alphabetic());
    gen_alphabet("digit.txt", |x| x.is_ascii_digit());
    gen_alphabet("whitespace.txt", |x| x.is_ascii_whitespace());
    gen_alphabet("symbol.txt", |x| {
        !(x.is_ascii_control()
            || x.is_ascii_alphanumeric()
            || x.is_ascii_whitespace())
    });
    println!("cargo:rerun-if-changed=build.rs");
}

fn gen_alphabet<F>(filename: &str, f: F)
where
    F: FnMut(&char) -> bool,
{
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(filename);
    let alpha: String = (0..127).filter_map(char::from_u32).filter(f).collect();
    fs::write(dest_path, alpha).unwrap();
}
