//by litissia.ben-mohand

use super::BWT::*;
use super::MTF::*;
use super::Huffman::*;
use super::RLE::*;
use std::fs::File;
use std::io::BufRead;
use std::io::{BufReader, BufWriter, Read, Write}; // Ajoute BufReader et BufWriter
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};


pub fn compress(input_path: &str) {
    let input_file = File::open(input_path).expect("Couldn't open input file");
    let mut reader = BufReader::new(input_file);

    let input_path = Path::new(input_path);
    let extension = input_path.extension().unwrap_or_default().to_str().unwrap();
    
    let output_path = input_path.with_file_name(format!("compressed_{}.ntk", input_path.file_stem().unwrap().to_str().unwrap()));
    let output_file = File::create(&output_path).expect("Couldn't create output file");
    let mut writer = BufWriter::new(output_file);

    // Write extension length and extension
    let ext_len = extension.len() as u8;
    writer.write_all(&[ext_len]).expect("Failed to write extension length");
    writer.write_all(extension.as_bytes()).expect("Failed to write extension");

    let mut buffer = [0; 16384];
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let chunk = &buffer[..bytes_read];

        // Step 1: Apply Burrows-Wheeler Transform (BWT)
        let (bwt_data, bwt_index) = bwt_encode(chunk);

        // Step 2: Move-to-Front Transform (MTF)
        let mtf_data = mtf_encode(&bwt_data);

        // Step 3: Run-Length Encoding (RLE)
        let rle_data = rle_encode(&mtf_data);

        // Step 4: Build Huffman frequency table
        let freq_table = build_frequency_table(&rle_data);

        // Step 5: Construct Huffman tree and generate codes
        let huffman_tree = build_huffman_tree(&freq_table);
        let mut huffman_codes = BTreeMap::new();
        if let Some(node) = huffman_tree.as_ref() {    
            generate_huffman_codes(node, String::new(), &mut huffman_codes);
        } else {
            panic!("Huffman tree is empty");
        }

        // Step 6: Encode data using Huffman codes
        let compressed_data = encode_with_huffman_codes(&rle_data, &huffman_codes);

        // Write metadata
        writer.write_all(&bwt_index.to_le_bytes()).expect("Failed to write BWT index");

        let freq_table_size = freq_table.len();
        writer.write_all(&freq_table_size.to_le_bytes()).expect("Failed to write freq table size");

        for (&symbol, &freq) in &freq_table {
            writer.write_all(&[symbol]).expect("Failed to write symbol");
            writer.write_all(&freq.to_le_bytes()).expect("Failed to write frequency");
        }

        let compressed_size = compressed_data.len();
        writer.write_all(&compressed_size.to_le_bytes()).expect("Failed to write compressed data size");
        writer.write_all(&compressed_data).expect("Failed to write compressed data");
    }

    writer.flush().expect("Error flushing output file");
}


pub fn decompress(input_path: &str) {
    let input_file = File::open(input_path).expect("Couldn't open compressed file");
    let mut reader = BufReader::new(input_file);

    // Read extension length and extension
    let mut ext_len = [0u8; 1];
    reader.read_exact(&mut ext_len).expect("Failed to read extension length");
    let ext_len = ext_len[0] as usize;

    let mut ext_buffer = vec![0u8; ext_len];
    reader.read_exact(&mut ext_buffer).expect("Failed to read extension");
    let extension = String::from_utf8(ext_buffer).expect("Invalid UTF-8 extension");

    // Construct output file name
    let path = Path::new(&input_path);
    let file_name = path.file_stem()
        .and_then(|stem| stem.to_str()) 
        .expect("Invalid filename");  
    let output_path = Path::new(input_path)
        .with_file_name(format!("decompressed_{}.{}",file_name, extension));
    let output_file = File::create(&output_path).expect("Couldn't create output file");
    let mut writer = BufWriter::new(output_file);

    while let Ok(_) = reader.fill_buf() {
        // Read BWT index
        let mut index_bytes = [0u8; std::mem::size_of::<usize>()];
        if reader.read_exact(&mut index_bytes).is_err() {
            break;
        }
        let bwt_index = usize::from_le_bytes(index_bytes);

        // Read frequency table size
        let mut size_bytes = [0u8; std::mem::size_of::<usize>()];
        reader.read_exact(&mut size_bytes).expect("Failed to read freq table size");
        let freq_table_size = usize::from_le_bytes(size_bytes);

        // Read frequency table
        let mut freq_table = BTreeMap::new();
        for _ in 0..freq_table_size {
            let mut symbol = [0u8; 1];
            reader.read_exact(&mut symbol).expect("Failed to read symbol");

            let mut freq_bytes = [0u8; std::mem::size_of::<usize>()];
            reader.read_exact(&mut freq_bytes).expect("Failed to read frequency");
            let freq = usize::from_le_bytes(freq_bytes);

            freq_table.insert(symbol[0], freq);
        }


        // Read compressed data size
        reader.read_exact(&mut size_bytes).expect("Failed to read compressed data size");
        let compressed_size = usize::from_le_bytes(size_bytes);

        // Read compressed data
        let mut compressed_data = vec![0u8; compressed_size];
        reader.read_exact(&mut compressed_data).expect("Failed to read compressed data");

        // Decode Huffman
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
        for (byte, code) in huffman_codes
        {
            inverted.insert(code.clone(), byte);
        }


        let decompressed_rle = decode_huffman(compressed_data, &inverted);
        // Decode RLE
        let decompressed_mtf = rle_decode(&decompressed_rle);

        // Decode MTF
        let decompressed_bwt = mtf_decode(&decompressed_mtf);

        // Reverse BWT
        let original_data = bwt_decode(&decompressed_bwt, bwt_index);

        writer.write_all(&original_data).expect("Failed to write decompressed data");
    }

    writer.flush().expect("Error flushing output file");
}
