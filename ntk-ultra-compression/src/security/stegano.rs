/*
by  Litissia Ben Mohand
 */

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::fs::File;
use std::io::{Read, Write};


pub fn encode(img_path: &str, file_path: &str, output_path: &str)
{
    //load the image
    let mut img = image::open(img_path).expect("can't load image");
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
        if bit_index >= bits.len()
        {
            break;
        }
        // modify the least significant bit (LSB) of the red channel to store a bit from the file
        pixel[0] = (pixel[0] & 0xFE) | bits[bit_index];
        bit_index += 1;
    }

    pixels.save(output_path).expect("couldn't save the image");
    println!("The file has been successfully hidden");
}



pub fn decode(img_path: &str, output_file_path: &str) -> Result<(), String> {
    // load image 
    let img = image::open(img_path).map_err(|e| format!("Impossible de charger l'image : {}", e))?;
    let pixels = img.to_rgba8(); // Convertit l'image en RGBA (format modifiable)

    // extract the hidden bits from the image 
    let mut bits = Vec::new();

    for (_, _, pixel) in pixels.enumerate_pixels() {
        // On récupère le bit caché dans le premier composant du pixel (le canal alpha)
        let bit = pixel[0] & 0x01; // Masque les autres bits et récupère le bit caché
        bits.push(bit);
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
        return Err("end_marker wasn't found".to_string());
    }

    // create an output file and save data in it 
    let mut output_file = File::create(output_file_path)
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    output_file.write_all(&file_data).map_err(|e| format!("Error writing to output file: {}", e))?;

    println!("The data has been successfully extracted and saved to: {}", output_file_path);
    Ok(())
}

