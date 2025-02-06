/*
by  Litissia Ben Mohand
 */

use image::GenericImageView;
use std::fs::File;
use std::io::{Read, Write};


pub fn encode(img_path: &str, file_path: &str, output_path: &str)
{
    //load the image
    let img = image::open(img_path).expect("can't load image");
    //convert the img to RGBA format for pixel manipulation
    let mut pixels = img.to_rgba8();
    //open the file that will be hidden
    let mut file = File::open(file_path).expect(&format!("can't open file : {}", file_path));
    let mut file_data = Vec::new();
    //read file and  add an end marker that we will use in decode fn 
    file.read_to_end(&mut file_data).expect("an error occured while trying to read data");
    file_data.extend_from_slice(b"NTK_END");//to know that we've reached the end 
   //convert file data into a vector of bits 
   let bits: Vec<u8> = file_data.iter()
    .flat_map(|&byte| (0..8).map(move |i| (byte >> (7 - i)) & 1))
    .collect(); //each byte is split into 8 bits
    let mut bit_index = 0;
    let (w,h) = img.dimensions(); //w = width, h = height
    if h*w < bits.len() as u32
    {
        println!("the image is not large enough to hide the file");
        return;
    }
    for (_,_, pixel) in pixels.enumerate_pixels_mut()
    {
        for i in 0..=2
        {
            if bit_index >= bits.len()
            {
                break;
            }
        // modify the least significant bit (LSB) of the each channel to store a bit from the file
        pixel[i] = (pixel[i] & 0xFE) | bits[bit_index];
        bit_index += 1;
        }
    }

    pixels.save(output_path).expect("couldn't save the image");
    println!("The file has been successfully hidden");
}



pub fn decode(img_path: &str, output_file_path: &str) {
    // load image 
    let img = image::open(img_path).expect(&format!("can't load image : {}",img_path));
    let pixels = img.to_rgba8();

    // extract the hidden bits from the image 
    let mut bits = Vec::new();

    for (_, _, pixel) in pixels.enumerate_pixels() {
        for i in 0..=2 
        {
            let bit = pixel[i] & 0x01; 
            bits.push(bit);
        }
    }

    //group bits into bytes
    let mut file_data = Vec::new();
    for chunk in bits.chunks(8) {
        let byte = chunk.iter().fold(0u8, |acc, &bit| (acc << 1) | bit);
        file_data.push(byte);
    }

    // search for the end marker and get rid of bytes that are not part of the file 
    let end_marker = b"NTK_END";
    if let Some(pos) = file_data.windows(end_marker.len()).position(|window| window == end_marker) {
        file_data.truncate(pos);
    } else {
        println!("end_marker wasn't found");
    }

    // create an output file and save data in it 
    let mut output_file = File::create(output_file_path)
        .expect(&format!("Failed to create output file: {}", output_file_path));
    output_file.write_all(&file_data).expect(&format!("Error writing to output file"));

    println!("The data has been successfully extracted and saved to: {}", output_file_path);
}

