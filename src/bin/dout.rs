use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::stdin;

fn main() -> std::io::Result<()> {
    let mut output: Vec<u8> = Vec::new();
    loop {
        let mut input: [u8; 1] = [0];
        match io::stdin().read(&mut input) {
            Ok(len) => if len == 0 {
                break;
            } else {
                output.push(input[0]);
            }
            Err(error) => {
                eprintln!("error: {}", error);
            }
        }
    }

    let mut file = File::create("foo.wav")?;
    file.write_all(&output)?;
    Ok(())
}
