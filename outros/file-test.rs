use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let path = "foo.txt";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
		.append(true)
        .open(path)?;
    file.write_all(b"Hello, world!\n")?;
    file.sync_all()?;
    Ok(())
}