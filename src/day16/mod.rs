#[cfg(test)] mod data;

use std::ops::{BitOrAssign, ShlAssign};

/*pub trait Number : BitOrAssign + Default + ShlAssign + Sized {

}*/

pub struct BitDecoder {
    data: Vec<char>,
    data_index: usize,
    bit_index: usize,
    pos: usize,
}

impl BitDecoder {
    pub fn new(stream: &str) -> BitDecoder {
        BitDecoder {
            data: stream.chars().collect(),
            data_index: 0,
            bit_index: 0,
            pos: 0,
        }
    }

    pub fn next_bit(&mut self) -> Option<u8> {
        if self.data_index.ge(&self.data.len()) {
            return None;
        } else if self.data_index.ge(&self.data.len()) && self.bit_index.ge(&3) {
            return None;
        }

        let value = self.data[self.data_index]
            .to_digit(16)
            .map(|d| (d as u8) & (0b1000 >> self.bit_index))
            .map(|d| d >> (3 - self.bit_index))
            .unwrap();

        // Increment indicies
        if self.bit_index.ge(&3) {
            self.data_index += 1;
            self.bit_index = 0;
        } else {
            self.bit_index += 1;
        }

        self.pos += 1;

        Some(value)
    }

    pub fn read<T: BitOrAssign + Default + ShlAssign + From<u8>>(&mut self, count: u8) -> T {
        let mut value = T::default();

        for _ in 0..count {
            value <<= T::from(0b1);

            match self.next_bit() {
                Some(bit) => value |= T::from(bit),
                _ => break,
            };
        }

        value
    }

    pub fn reset(&mut self) {
        self.data_index = 0;
        self.bit_index = 0;
        self.pos = 0;
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }
}

pub fn read_packet(reader: &mut BitDecoder, packets: &mut Vec<u32>) {
    const VERSION_LENGTH: u8 = 3;
    const TYPE_LENGTH: u8 = 3;

    let packet_version: u32 = reader.read(VERSION_LENGTH);
    let packet_type: u32 = reader.read(TYPE_LENGTH);

    /*
        TYPES:
            3 = Operator (anything not 4... for now)
                > Packet contains one or more packets
                > Length:
                    0 = Total bit length of sub packets (15 bits)
                    1 = Number of sub packets (11 bits)
            4 = Literal
                > Single binary number
                > Encoded in 5-bit chunks (1 bit: flag, 4 bit: literal)
                > Read until 0-terminated bit
    */

    packets.push(packet_version);

    if packet_type.eq(&4) {
        // Literal
        //let number = 0u64;

        loop {
            let terminate = reader.read::<u8>(1) == 0;
            reader.read::<u32>(4);

            if terminate {
                break;
            }
        }
    } else {
        // Operator
        let len_type: u8 = reader.read(1);

        if len_type.eq(&0) {
            // Total length of sub packets
            let bit_count: u32 = reader.read(15);
            let end_pos = reader.get_pos() + (bit_count as usize);

            while reader.get_pos().lt(&end_pos) {
                read_packet(reader, packets);
            }
        } else {
            // Sub packet count
            let packet_count: u32 = reader.read(11);
            let mut count = 0;

            while count.lt(&packet_count) {
                read_packet(reader, packets);
                count += 1;
            }
        }
    }
}

pub fn decode_packet_sum(stream: &str) -> u32 {
    let mut reader = BitDecoder::new(stream);
    let mut packets = Vec::new();

    read_packet(&mut reader, &mut packets);

    packets.iter().sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0_0, 6)]
    #[case(TEST_DATA_0_1, 9)]
    #[case(TEST_DATA_1_0, 16)]
    #[case(TEST_DATA_2, 891)]
    pub fn decode_packet_sum_test(#[case] stream: &'static str, #[case] expected: u32) {
        let result = decode_packet_sum(stream);
        assert_eq!(expected, result);
    }
}