use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()>
{
    let buffer = d_in("speech.wav").expect("failed reading file");
    d_out(buffer, "output.wav").expect("failed writing file");

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
}y

fn d_out(input_data: Vec<f32>, output_file: &'static str) -> io::Result<()>
{
    let mut f = File::create(output_file)?;

    for sample in input_data
    {
        let fused_byte: [u8; 4] = unsafe { std::mem::transmute::<f32, [u8; 4]>(sample) };
        f.write(&fused_byte)?;
    }

    Ok(())
}
