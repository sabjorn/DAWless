use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::stdout;

fn main() -> io::Result<()>
{
    let buffer = d_in("speech.wav").expect("failed reading file");
    for sample in buffer
    {
        let fused_byte: [u8; 4] = unsafe { std::mem::transmute::<f32, [u8; 4]>(sample) };
        io::stdout().write(&fused_byte).expect("failed to write to stdout");
    }
    io::stdout().write(b"");
    Ok(())
}

fn d_in(filename: &'static str) -> io::Result<Vec<f32>>
{
    let mut f = File::open(filename)?;

    let mut raw_data: Vec<u8> = Vec::new();
    f.read_to_end(&mut raw_data)?;

    let header_length = 44;
    let buffer_range = raw_data.len() - header_length;
    let mut internal_buffer: Vec<f32> = Vec::with_capacity((raw_data.len() - header_length) / 2);

    for x in (1..buffer_range).step_by(2)
    {
        let first_byte: i16 = raw_data[x] as i16;
        let second_byte: i16 = raw_data[x + 1] as i16;
        let fused_byte: i16 = first_byte << 8 | second_byte;
        internal_buffer.push( fused_byte as f32 / (32768 as f32));
    }

    Ok(internal_buffer)
}
