use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

const MODEL_ORDER: usize = 3;
// Précision sur 64 bits
const PRECISION: u128 = 0xFFFFFFFFFFFFFFFF;
// Définition des bornes pour l'algorithme classique d'arithmétique
const HALF: u128 = 1 << 63;            // 0x8000_0000_00000000
const QUARTER: u128 = 1 << 62;         // 0x4000_0000_00000000
const THREE_QUARTER: u128 = QUARTER * 3; // 0xC000_0000_00000000

pub struct Compressor {
    model: ContextModel,
}

impl Compressor {
    pub fn new() -> Self {
        Self {
            model: ContextModel::new(),
        }
    }

    pub fn compress_file(&mut self, input_path: &str, output_path: &str) -> Result<CompressionStats> {
        let timer = Instant::now();
        
        let data = std::fs::read(input_path)
            .with_context(|| format!("Échec de la lecture de {}", input_path))?;
        let original_size = data.len();

        let mut encoder = ArithmeticEncoder::new();
        encoder.write_header(original_size as u64);
        
        for (i, &byte) in data.iter().enumerate() {
            let context = self.get_context(&data, i);
            let (counts, total) = self.model.predict(&context);
            encoder.encode(byte, &counts, total);
            self.model.update(&context, byte);
        }
        
        let compressed = encoder.finalize();

        File::create(output_path)
            .and_then(|mut f| f.write_all(&compressed))
            .with_context(|| format!("Échec de l'écriture dans {}", output_path))?;

        Ok(CompressionStats {
            duration: timer.elapsed(),
            original_size,
            compressed_size: compressed.len(),
        })
    }

    fn get_context(&self, data: &[u8], pos: usize) -> Vec<u8> {
        let start = pos.saturating_sub(MODEL_ORDER);
        data[start..pos].to_vec()
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

struct ArithmeticEncoder {
    low: u128,
    high: u128,
    underflow: u32,
    output: Vec<u8>,
    // Pour émettre bit par bit puis les regrouper en octets
    buffer: u8,
    bits_in_buffer: u8,
}

impl ArithmeticEncoder {
    fn new() -> Self {
        Self {
            low: 0,
            high: PRECISION,
            underflow: 0,
            output: Vec::new(),
            buffer: 0,
            bits_in_buffer: 0,
        }
    }

    fn write_header(&mut self, size: u64) {
        // On stocke la taille originale en entête (8 octets)
        self.output.extend_from_slice(&size.to_be_bytes());
    }

    fn output_bit(&mut self, bit: u8) {
        self.buffer = (self.buffer << 1) | (bit & 1);
        self.bits_in_buffer += 1;
        if self.bits_in_buffer == 8 {
            self.output.push(self.buffer);
            self.buffer = 0;
            self.bits_in_buffer = 0;
        }
    }

    fn flush_buffer(&mut self) {
        if self.bits_in_buffer > 0 {
            self.buffer <<= 8 - self.bits_in_buffer;
            self.output.push(self.buffer);
            self.buffer = 0;
            self.bits_in_buffer = 0;
        }
    }

    fn encode(&mut self, symbol: u8, counts: &[u32; 256], total: u32) {
        let range = self.high - self.low + 1;
        let total = total as u128;
        let symbol_index = symbol as usize;
        let mut cumulative = 0u128;
        let mut symbol_low = 0u128;
        let mut symbol_high = 0u128;

        // Calcul de la plage cumulée pour le symbole
        for (i, &count) in counts.iter().enumerate() {
            let count = count as u128;
            if i == symbol_index {
                symbol_low = cumulative;
                symbol_high = cumulative + count;
                break;
            }
            cumulative += count;
        }

        self.high = self.low + (range * symbol_high) / total - 1;
        self.low = self.low + (range * symbol_low) / total;

        // Renormalisation avec gestion de l'underflow
        loop {
            if self.high < HALF {
                // Cas où les deux bornes sont dans la moitié inférieure
                self.output_bit(0);
                for _ in 0..self.underflow {
                    self.output_bit(1);
                }
                self.underflow = 0;
                self.low <<= 1;
                self.high = (self.high << 1) | 1;
            } else if self.low >= HALF {
                // Cas où les deux bornes sont dans la moitié supérieure
                self.output_bit(1);
                for _ in 0..self.underflow {
                    self.output_bit(0);
                }
                self.underflow = 0;
                self.low = (self.low - HALF) << 1;
                self.high = ((self.high - HALF) << 1) | 1;
            } else if self.low >= QUARTER && self.high < THREE_QUARTER {
                // Cas d'underflow
                self.underflow += 1;
                self.low = (self.low - QUARTER) << 1;
                self.high = ((self.high - QUARTER) << 1) | 1;
            } else {
                break;
            }
        }
    }

    fn finalize(mut self) -> Vec<u8> {
        // Terminaison de l'encodage
        self.underflow += 1;
        if self.low < QUARTER {
            self.output_bit(0);
            for _ in 0..self.underflow {
                self.output_bit(1);
            }
        } else {
            self.output_bit(1);
            for _ in 0..self.underflow {
                self.output_bit(0);
            }
        }
        self.flush_buffer();
        self.output
    }
}

pub struct CompressionStats {
    pub duration: Duration,
    pub original_size: usize,
    pub compressed_size: usize,
}

impl CompressionStats {
    pub fn print(&self) {
        println!("Compression réussie en {:.3?}", self.duration);
        println!("Taille originale  : {:>10} octets", self.original_size);
        println!("Taille compressée : {:>10} octets", self.compressed_size);
        println!("Taux de compression : {:.2}%", 
            (self.compressed_size as f64 / self.original_size as f64) * 100.0);
    }
}

