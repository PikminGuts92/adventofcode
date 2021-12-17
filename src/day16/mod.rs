#[cfg(test)] mod data;

use std::ops::{BitOrAssign, ShlAssign};

/*pub trait Number : BitOrAssign + Default + ShlAssign + Sized {

}*/

const VERSION_LENGTH: u8 = 3;
const TYPE_LENGTH: u8 = 3;

type NUMBER = i64;

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

pub enum Packet {
    /* 0 */ Sum(Vec<Packet>),
    /* 1 */ Product(Vec<Packet>),
    /* 2 */ Minimum(Vec<Packet>),
    /* 3 */ Maximum(Vec<Packet>),

    /* 4 */ Literal(NUMBER),

    // All have exactly two sub packets
    /* 5 */ GreaterThan(Vec<Packet>),
    /* 6 */ LessThan(Vec<Packet>),
    /* 7 */ Equal(Vec<Packet>),
}

impl Packet {
    pub fn evalulate(&self) -> NUMBER {
        match self {
            Packet::Sum(nums) => nums
                .iter()
                .map(|n| n.evalulate())
                .sum(),
            Packet::Product(nums) => nums
                .iter()
                .map(|n| n.evalulate())
                .product(),
            Packet::Minimum(nums) => {
                let mut nums = nums
                    .iter()
                    .map(|n| n.evalulate())
                    .collect::<Vec<_>>();

                nums.sort();

                nums
                    .first()
                    .map(|n| *n)
                    .unwrap_or_default()
            },
            Packet::Maximum(nums) => {
                let mut nums = nums
                    .iter()
                    .map(|n| n.evalulate())
                    .collect::<Vec<_>>();

                nums.sort();

                nums
                    .last()
                    .map(|n| *n)
                    .unwrap_or_default()
            },
            Packet::Literal(num) => *num,
            Packet::GreaterThan(nums) => {
                let a = nums[0].evalulate();
                let b = nums[1].evalulate();

                if a.gt(&b) {
                    1
                } else {
                    0
                }
            }
            Packet::LessThan(nums) => {
                let a = nums[0].evalulate();
                let b = nums[1].evalulate();

                if a.lt(&b) {
                    1
                } else {
                    0
                }
            }
            Packet::Equal(nums) => {
                let a = nums[0].evalulate();
                let b = nums[1].evalulate();

                if a.eq(&b) {
                    1
                } else {
                    0
                }
            }
        }
    }

    pub fn from_stream(reader: &mut BitDecoder) -> Packet {
        let _packet_version: u32 = reader.read(VERSION_LENGTH);
        let packet_type: u32 = reader.read(TYPE_LENGTH);

        if packet_type.eq(&4) {
            // Literal
            let mut value = 0i64;
    
            loop {
                let terminate = reader.read::<u8>(1) == 0;

                value <<= 4;
                value |= reader.read::<i64>(4);

                if terminate {
                    break;
                }
            }

            Packet::Literal(value)
        } else {
            // Operator
            let len_type: u8 = reader.read(1);
            let mut sub_packets = Vec::new();
    
            if len_type.eq(&0) {
                // Total length of sub packets
                let bit_count: u32 = reader.read(15);
                let end_pos = reader.get_pos() + (bit_count as usize);

                while reader.get_pos().lt(&end_pos) {
                    sub_packets.push(Packet::from_stream(reader));
                }
            } else {
                // Sub packet count
                let packet_count: u32 = reader.read(11);
                let mut count = 0;

                while count.lt(&packet_count) {
                    sub_packets.push(Packet::from_stream(reader));
                    count += 1;
                }
            }

            match packet_type {
                0 => Packet::Sum(sub_packets),
                1 => Packet::Product(sub_packets),
                2 => Packet::Minimum(sub_packets),
                3 => Packet::Maximum(sub_packets),
                5 => Packet::GreaterThan(sub_packets),
                6 => Packet::LessThan(sub_packets),
                7 => Packet::Equal(sub_packets),
                _ => panic!("\"{}\" not supported!", &packet_type)
            }
        }
    }
}

pub fn read_packet_versions(reader: &mut BitDecoder, packets: &mut Vec<u32>) {
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
                read_packet_versions(reader, packets);
            }
        } else {
            // Sub packet count
            let packet_count: u32 = reader.read(11);
            let mut count = 0;

            while count.lt(&packet_count) {
                read_packet_versions(reader, packets);
                count += 1;
            }
        }
    }
}

pub fn decode_packet_sum(stream: &str) -> u32 {
    let mut reader = BitDecoder::new(stream);
    let mut packets = Vec::new();

    read_packet_versions(&mut reader, &mut packets);

    packets.iter().sum()
}

pub fn read_and_evaluate_packet(stream: &str) -> NUMBER {
    let mut reader = BitDecoder::new(stream);

    let packet = Packet::from_stream(&mut reader);
    packet.evalulate()
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

    #[rstest]
    #[case("C200B40A82", 3)]
    #[case("04005AC33890", 54)]
    #[case("880086C3E88112", 7)]
    #[case("CE00C43D881120", 9)]
    #[case("D8005AC2A8F0", 1)]
    #[case("F600BC2D8F", 0)]
    #[case("9C005AC2F8F0", 0)]
    #[case("9C0141080250320F1802104A08", 1)]
    #[case(TEST_DATA_2, 673042777597)]
    pub fn read_and_evaluate_packet_test(#[case] stream: &'static str, #[case] expected: i64) {
        let result = read_and_evaluate_packet(stream);
        assert_eq!(expected, result);
    }
}