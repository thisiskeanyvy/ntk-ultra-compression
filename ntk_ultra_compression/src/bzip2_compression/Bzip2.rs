//by litissia.ben-mohand

use super::BWT::*;
use super::MTF::*;
use super::Huffman::*;
use super::RLE::*;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::collections::BTreeMap;
use std::path::Path;
use std::collections::HashMap;

/*Compresses a file using Bzip2-like compression (BWT, MTF, RLE, Huffman Encoding)*/
pub fn compress(input_path: &str)
{
    let mut file = File::open(input_path).expect("couldn't open input file");

    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("error while readin the file");
    // Step 1: Apply Burrows-Wheeler Transform (BWT)
    let(bwt_data, bwt_index) = bwt_encode(&data);

    // Step 2: Move-to-Front Transform (MTF)
    let mtf_data = mtf_encode(&bwt_data);

    // Step 3: Run-Length Encoding (RLE)
    let rle_data = rle_encode(&mtf_data);

    // Step 4: encoding data using huffman codes 
    let freq_table = build_frequency_table(&rle_data);
    let huffman_tree = build_huffman_tree(&freq_table);
    let mut huffman_codes = BTreeMap::new();
    if let Some(node) = huffman_tree.as_ref()
    {    
        generate_huffman_codes(node, String::new(), &mut huffman_codes);
    }
    else
    {
        panic!("huffman tree empty");
    }

    let compressed_data = encode_with_huffman_codes(&rle_data, &huffman_codes);


    //write compressed data in output file
    let input_path = Path::new(input_path);
    let output_path = input_path.with_file_name(format!("compressed_{}.ntk", Path::new(input_path).file_stem().unwrap().to_str().unwrap()));
    let mut output_file = File::create(&output_path).expect("error while creating output file");

    let table_size = freq_table.len() as u32;
    output_file.write_all(&table_size.to_be_bytes()).expect("Erreur écriture taille table");

    for (&byte, &freq) in &freq_table {
        output_file.write_all(&[byte]).expect("Erreur écriture byte");
        output_file.write_all(&(freq as u64).to_be_bytes()).expect("Erreur écriture fréquence");
    }


    output_file.write_all(&(bwt_index as u64).to_be_bytes()).expect("error while writing in output file");
    output_file.write_all(&compressed_data).expect("error while writing in outpu file");
}


pub fn decompress(input_path: &str, extension: &str) 
{
    let mut file = File::open(input_path).expect("error while opening input file");
    let mut compressed_data = Vec::new();
    file.read_to_end(&mut compressed_data).expect("error while reading compressed file");
    let mut cursor = 0;

    //read frequency table
    let table_size = u32::from_be_bytes(compressed_data[cursor..cursor + 4].try_into().unwrap()) as usize;
    cursor = 4;
    let mut freq_table = BTreeMap::new();
    for _ in 0..table_size
    {
        let byte = compressed_data[cursor];
        let freq = u64::from_be_bytes(compressed_data[cursor+1..cursor+9].try_into().unwrap());
        freq_table.insert(byte,freq as usize);
        cursor += 9;
    }

    //read bwt index
    let bwt_index = u64::from_be_bytes(compressed_data[cursor..cursor+8].try_into().unwrap());
    
    let compressed_data = compressed_data[cursor+8..].to_vec();
    let huffman_tree = build_huffman_tree(&freq_table);
    let mut huffman_codes = BTreeMap::new();
    if let Some(node) = huffman_tree.as_ref()
    {
        generate_huffman_codes(node,String::new(), &mut huffman_codes);
    }
    else
    {
        panic!("huffman tree is empty");
    }
    let mut inverted = BTreeMap::new();
    for (byte, code) in huffman_codes {
        inverted.insert(code.clone(), byte);
    }
    for (byte, code) in &inverted {
}


    //do all the steps from compression but in reverse
    let rle_encoded_data = decode_huffman(compressed_data,&inverted);
    let mtf_encoded_data = rle_decode(&rle_encoded_data);
    let bwt_encoded_data = mtf_decode(&mtf_encoded_data);
    let original_data = bwt_decode(&bwt_encoded_data, bwt_index as usize);


    let input_path = Path::new(input_path);
    let output_path = input_path.with_file_name(format!("decompressed_{}.{}", Path::new(input_path).file_stem().unwrap().to_str().unwrap(), extension));
        //write decompressed data in the output file
    let mut output_file = File::create(&output_path).expect("Impossible de créer le fichier de sortie");
    output_file.write_all(&original_data).expect("error while writing data in output file");

}


