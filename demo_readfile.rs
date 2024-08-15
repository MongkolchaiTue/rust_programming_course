use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("cat.txt")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..])?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

/*
https://doc.rust-lang.org/std/io/trait.Read.html

--output--
--empty file
rust_programming_course % ./demo_readfile
The bytes: []

--demo read file.--text in file
rust_programming_course % ./demo_readfile
The bytes: [100, 101, 109, 111, 32, 114, 101, 97, 100, 32]

--file name not found
rust_programming_course % ./demo_readfile
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
*/
/*
-rwxr-xr-x  1   staff  409096 Aug 15 20:46 demo_readfile
-rw-r--r--  1   staff     646 Aug 15 20:54 demo_readfile.rs
*/