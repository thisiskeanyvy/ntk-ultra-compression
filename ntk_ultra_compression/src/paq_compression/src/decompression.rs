use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::time::{Duration, Instant};

const MODEL_ORDER: usize = 3;
const PRECISION: u128 = 0xFFFFFFFFFFFFFFFF;
const HALF: u128 = 1 << 63;
const QUARTER: u128 = 1 << 62;
const THREE_QUARTER: u128 = QUARTER * 3;

pub struct Decompressor {
    model: ContextModel,
}

impl Decompressor {
    pub fn new() -> Self {
        Self {
            model: ContextModel::new(),
        }
    }

    pub fn decompress_file(&mut self, input_path: &str, output_path: &str) -> Result<DecompressionStats> {
        let timer = Instant::now();
        
        let compressed = std::fs::read(input_path)
            .with_context(|| format!("Échec de la lecture de {}", input_path))?;
        
        let decompressed = self.decompress(&compressed)?;
        
        std::fs::write(output_path, &decompressed)
            .with_context(|| format!("Échec de l'écriture dans {}", output_path))?;

        Ok(DecompressionStats {
            duration: timer.elapsed(),
            compressed_size: compressed.len(),
            decompressed_size: decompressed.len(),
        })
    }

    fn decompress(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = ArithmeticDecoder::new(data)?;
        let mut output = Vec::with_capacity(decoder.original_size());
        let mut context = Vec::new();

        for _ in 0..decoder.original_size() {
            let (counts, total) = self.model.predict(&context);
            let byte = decoder.decode(&counts, total)?;
            
            output.push(byte);
            self.model.update(&context, byte);
            
            context.push(byte);
            if context.len() > MODEL_ORDER {
                context.remove(0);
            }
        }

        Ok(output)
    }
}

struct ArithmeticDecoder {
    data: Vec<u8>,
    pos: usize, // position en bits dans le flux
    low: u128,
    high: u128,
    value: u128,
    original_size: usize,
}

impl ArithmeticDecoder {
    fn new(data: &[u8]) -> Result<Self> {
        if data.len() < 8 {
            return Err(anyhow!("Header manquant"));
        }

        let original_size = u64::from_be_bytes(data[..8].try_into()?) as usize;
        let data = data[8..].to_vec();

        let mut decoder = Self {
            data,
            pos: 0,
            low: 0,
            high: PRECISION,
            value: 0,
            original_size,
        };

        // Initialisation en lisant 64 bits du flux
        for _ in 0..64 {
            decoder.value = (decoder.value << 1) | decoder.next_bit() as u128;
        }

        Ok(decoder)
    }

    fn original_size(&self) -> usize {
        self.original_size
    }

    fn next_bit(&mut self) -> u8 {
        let byte_index = self.pos / 8;
        if byte_index < self.data.len() {
            let bit_index = 7 - (self.pos % 8);
            let bit = (self.data[byte_index] >> bit_index) & 1;
            self.pos += 1;
            bit
        } else {
            self.pos += 1;
            0
        }
    }

    fn decode(&mut self, counts: &[u32; 256], total: u32) -> Result<u8> {
        let total = total as u128;
        let range = self.high - self.low + 1;
        let offset = self.value - self.low;
        let target = ((offset + 1) * total - 1) / range;

        let mut cumulative = 0u128;
        let mut symbol = 0u8;
        let mut symbol_low = 0u128;
        let mut symbol_high = 0u128;
        for (i, &count) in counts.iter().enumerate() {
            let count = count as u128;
            if cumulative + count > target {
                symbol = i as u8;
                symbol_low = cumulative;
                symbol_high = cumulative + count;
                break;
            }
            cumulative += count;
        }

        self.high = self.low + (range * symbol_high) / total - 1;
        self.low = self.low + (range * symbol_low) / total;

        loop {
            if self.high < HALF {
                // Rien à ajuster
            } else if self.low >= HALF {
                self.value -= HALF;
                self.low -= HALF;
                self.high -= HALF;
            } else if self.low >= QUARTER && self.high < THREE_QUARTER {
                self.value -= QUARTER;
                self.low -= QUARTER;
                self.high -= QUARTER;
            } else {
                break;
            }
            self.low <<= 1;
            self.high = (self.high << 1) | 1;
            self.value = (self.value << 1) | self.next_bit() as u128;
        }

        Ok(symbol)
    }
}

struct ContextModel {
    counts: HashMap<Vec<u8>, [u32; 256]>,
}

impl ContextModel {
    fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    fn update(&mut self, context: &[u8], symbol: u8) {
        let entry = self.counts.entry(context.to_vec()).or_insert([1; 256]);
        entry[symbol as usize] += 1;
    }

    fn predict(&self, context: &[u8]) -> ([u32; 256], u32) {
        let counts = self.counts.get(context).unwrap_or(&[1; 256]);
        let total = counts.iter().sum::<u32>().max(1);
        (*counts, total)
    }
}

pub struct DecompressionStats {
    pub duration: Duration,
    pub compressed_size: usize,
    pub decompressed_size: usize,
}

impl DecompressionStats {
    pub fn print(&self) {
        println!("Décompression réussie en {:.3?}", self.duration);
        println!("Taille compressée   : {:>10} octets", self.compressed_size);
        println!("Taille décompressée : {:>10} octets", self.decompressed_size);
    }
}

