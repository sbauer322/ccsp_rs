/// Corresponds to CCSP in Python, Section 1.2, titled "Trivial Compression"

#[derive(Debug)]
struct CompressedGene {
    bit_string: i32,
}

impl CompressedGene {
    fn compress(gene: &str) -> CompressedGene {
        let mut bit_string: i32 = 1;
        for nucleotide in gene.to_uppercase().chars() {
            bit_string <<= 2;
            match nucleotide {
                'A' => bit_string |= 0b00,
                'C' => bit_string |= 0b01,
                'G' => bit_string |= 0b10,
                'T' => bit_string |= 0b11,
                _ => unimplemented!(),
            };
        }
        CompressedGene { bit_string }
    }

    fn decompress(&self) -> String {
        let mut gene = String::new();
        let bit_length = count_bits(self.bit_string) - 1;
        for i in (0..bit_length).step_by(2) {
            let bits = self.bit_string >> i & 0b11;
            match bits {
                0b00 => gene.push('A'),
                0b01 => gene.push('C'),
                0b10 => gene.push('G'),
                0b11 => gene.push('T'),
                _ => unimplemented!(),
            }
        }
        gene.chars().rev().collect()
    }
}

fn count_bits(n: i32) -> i32 {
    let mut i = n;
    let mut acc = 0;
    while i > 0 {
        i >>= 1;
        acc += 1;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(4), 3);
        assert_eq!(count_bits(66), 7);
        assert_eq!(count_bits(128), 8);
        assert_eq!(count_bits(283), 9);

        let original = "CAGTCGATAGAGTAT";
        let compressed = CompressedGene::compress(original);
        assert_eq!(compressed.bit_string, 1389938867);
        assert_eq!(compressed.decompress(), original);
    }
}
