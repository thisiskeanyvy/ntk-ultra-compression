use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/*
 * Builds a frequency table from the input data.
 *This function counts the occurrences of each byte in 
 the input and returns a map.
*/
pub fn build_frequency_table(data: &[u8]) -> BTreeMap<u8, usize> {
    let mut freq = BTreeMap::new();

    for byte in data.iter()
    {
        *freq.entry(*byte).or_insert(0) += 1;
    }
    freq
}


#[derive(Eq, PartialEq)]
pub struct Node {
    frequency: usize,
    character: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency) // Min-Heap (inverse l'ordre)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_huffman_tree(frequencies: &BTreeMap<u8, usize>) -> Option<Box<Node>> 
{

    let mut heap = BinaryHeap::new();
    // Create a leaf node for each character and push it into the heap
    for (&ch,&freq) in frequencies.iter()
    {
        heap.push(Box::new( Node {
            frequency : freq,
            character : Some(ch),
            left : None,
            right : None,
        }));
    }

    //build the tree by merging the two lowest frequency nodes until one remains
    //it will be the root of the tree
    while heap.len() > 1
    {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let parent = Box::new(Node {
            frequency : left.frequency + right.frequency,
            character : None,
            left : Some(left),
            right : Some(right),
        });
        heap.push(parent);
    }

    heap.pop()
}


pub fn generate_huffman_codes(
    node: &Node,
    prefix: String,
    codes: &mut std::collections::BTreeMap<u8, String>) 
{
    if let Some(ch) = node.character
    {
        codes.insert(ch, prefix);
    }
    else
    {
        if let Some(ref left) = node.left
        {
            generate_huffman_codes(left, format!("{}0", prefix), codes);
        }
        if let Some(ref right) = node.right
        {
            generate_huffman_codes(right, format!("{}1",prefix), codes);
        }
    }

}

pub fn encode_with_huffman_codes(data: &[u8], huffman_codes: &BTreeMap<u8, String>) -> Vec<u8>
{
    let mut encoded_bits : String  = String::new();

    for byte in data.iter()
    {
        if let Some(code) = huffman_codes.get(byte)
        {
            encoded_bits.push_str(code);
        }
        else
        {
            panic!("Byte {:?} not found in huffman codes table", byte);
        }
    }
    let mut compressed_data : Vec<u8> = Vec::new();
    let mut current_byte : u8 = 0;
    let mut bit_count : u8 = 0;

    for bit in encoded_bits.chars()
    {
        current_byte <<= 1;
        if bit == '1'
        {
            current_byte |= 1;
        }
        bit_count += 1;
        if bit_count == 8
        {
            compressed_data.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }

    }

    let padding_bits = 
        if bit_count > 0
        {
            current_byte <<= 8 - bit_count;
            compressed_data.push(current_byte);
            8 - bit_count
        } 
        else
        {
            0
        };

    compressed_data.push(padding_bits);
    compressed_data

}


pub fn decode_huffman(mut encoded_data: Vec<u8>, huffman_table: &BTreeMap<String, u8>) -> Vec<u8>
{
    let mut bit_string : String = String::new();
    let bits_to_ignore = encoded_data.pop().unwrap() as usize;
    
    for byte in encoded_data
    {
        bit_string.push_str(&format!("{:08b}",byte));
    }

    if bits_to_ignore > 0
    {
        bit_string.truncate(bit_string.len()-bits_to_ignore);
    }
    let mut decoded_data : Vec<u8> = Vec::new();
    let mut current_code : String = String::new();

    for bit in bit_string.chars()
    {
        current_code.push(bit);
        if let Some(ch) = huffman_table.get(&current_code)
        {
            decoded_data.push(*ch);
            current_code = String::new();
        }
    }
    decoded_data
}
