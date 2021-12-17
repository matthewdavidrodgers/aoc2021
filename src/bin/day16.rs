use colored::*;

fn load_input(input: &str) -> Vec<u8> {
    let mut bytes = vec![];
    let chars: Vec<_> = input.chars().filter_map(|c| c.to_digit(16)).collect();
    for (i, c) in chars.into_iter().enumerate() {
        if i % 2 == 0 {
            bytes.push((c << 4) as u8);
        } else {
            let last_byte_index = bytes.len() - 1;
            bytes[last_byte_index] |= c as u8;
        }
    }

    bytes
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

#[derive(Debug)]
enum PacketPayload {
    Literal {
        value: u64,
    },
    Operator {
        operation: Operation,
        packets: Vec<Packet>,
    },
}

#[derive(Debug)]
struct Packet {
    version: u8,
    payload: PacketPayload,
}

impl Packet {
    fn sum_versions(&self) -> u32 {
        match &self.payload {
            PacketPayload::Literal { .. } => self.version as u32,
            PacketPayload::Operator { packets, .. } => {
                let children_sum: u32 = packets.iter().map(|packet| packet.sum_versions()).sum();
                self.version as u32 + children_sum
            }
        }
    }

    fn evaluate(&self) -> u64 {
        use Operation::*;

        match &self.payload {
            PacketPayload::Literal { value } => *value,
            PacketPayload::Operator { operation, packets } => match operation {
                Sum => packets
                    .iter()
                    .fold(0, |acc, packet| acc + packet.evaluate()),
                Product => packets
                    .iter()
                    .fold(1, |acc, packet| acc * packet.evaluate()),
                Min => packets
                    .iter()
                    .map(|packet| packet.evaluate())
                    .min()
                    .unwrap(),
                Max => packets
                    .iter()
                    .map(|packet| packet.evaluate())
                    .max()
                    .unwrap(),
                Greater => {
                    let a = packets[0].evaluate();
                    let b = packets[1].evaluate();
                    if a > b {
                        1
                    } else {
                        0
                    }
                }
                Less => {
                    let a = packets[0].evaluate();
                    let b = packets[1].evaluate();
                    if a < b {
                        1
                    } else {
                        0
                    }
                }
                Equal => {
                    let a = packets[0].evaluate();
                    let b = packets[1].evaluate();
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

struct Parser<'a> {
    bytes: &'a Vec<u8>,
    bit_index: usize,
}

impl<'a> Parser<'a> {
    fn new(bytes: &'a Vec<u8>) -> Parser {
        Parser {
            bytes,
            bit_index: 0,
        }
    }

    fn print(&self) {
        for (byte_index, byte) in self.bytes.iter().enumerate() {
            for bit_index in 0..8 {
                let bit = (byte_index * 8) + bit_index;
                let mask = 0x1 << 7 - bit_index;
                let bit_is_set = byte & mask != 0;
                if self.bit_index == bit {
                    if bit_is_set {
                        print!("{}", "1".red());
                    } else {
                        print!("{}", "0".red());
                    }
                } else {
                    if bit_is_set {
                        print!("1");
                    } else {
                        print!("0");
                    }
                }
            }
        }
        print!("\n");
    }

    fn bit_set_at(&self, index: usize) -> bool {
        let byte_offset = index / 8;
        let bit_offset = index % 8;

        let byte = self.bytes[byte_offset];
        let mask = 0x1 << (7 - bit_offset);

        byte & mask != 0
    }

    fn read_n_bits(&mut self, n: usize) -> u64 {
        let mut read = 0;
        for i in 0..n {
            if self.bit_set_at(self.bit_index + i) {
                // TODO(MATT): peel bits in larger operations, not one at a time
                let mask = 0x1 << (n - 1) - i;
                read |= mask;
            }
        }
        self.bit_index += n;

        read
    }

    fn parse_literal_payload(&mut self) -> u64 {
        let mut chunks = vec![];

        loop {
            let is_last = self.read_n_bits(1) == 0;
            chunks.push(self.read_n_bits(4));

            if is_last {
                break;
            }
        }

        let chunk_len = chunks.len();
        let mut literal = 0;

        for (i, chunk) in chunks.into_iter().enumerate() {
            literal |= chunk << (4 * (chunk_len - 1 - i));
        }

        literal
    }

    fn parse_operator_payload(&mut self) -> Vec<Packet> {
        let mut packets = vec![];

        let parse_sub_packets_by_bits = self.read_n_bits(1) == 0;

        if parse_sub_packets_by_bits {
            let bit_count = self.read_n_bits(15);
            let start = self.bit_index;
            while self.bit_index - start < bit_count as usize {
                packets.push(self.parse_packet().unwrap());
            }
        } else {
            let packet_count = self.read_n_bits(11);
            for _ in 0..packet_count {
                packets.push(self.parse_packet().unwrap());
            }
        }

        packets
    }

    fn parse_packet(&mut self) -> Option<Packet> {
        use Operation::*;

        if self.bit_index / 8 > self.bytes.len() {
            return None;
        }

        let version = self.read_n_bits(3) as u8;
        let packet_type = self.read_n_bits(3) as u8;

        let payload = match packet_type {
            0 => PacketPayload::Operator {
                operation: Sum,
                packets: self.parse_operator_payload(),
            },
            1 => PacketPayload::Operator {
                operation: Product,
                packets: self.parse_operator_payload(),
            },
            2 => PacketPayload::Operator {
                operation: Min,
                packets: self.parse_operator_payload(),
            },
            3 => PacketPayload::Operator {
                operation: Max,
                packets: self.parse_operator_payload(),
            },
            4 => PacketPayload::Literal {
                value: self.parse_literal_payload(),
            },
            5 => PacketPayload::Operator {
                operation: Greater,
                packets: self.parse_operator_payload(),
            },
            6 => PacketPayload::Operator {
                operation: Less,
                packets: self.parse_operator_payload(),
            },
            7 => PacketPayload::Operator {
                operation: Equal,
                packets: self.parse_operator_payload(),
            },
            _ => unreachable!(),
        };

        Some(Packet { version, payload })
    }
}

fn part_one(bytes: &Vec<u8>) -> u32 {
    let mut parser = Parser::new(bytes);

    let packet = parser.parse_packet().unwrap();

    packet.sum_versions()
}

fn part_two(bytes: &Vec<u8>) -> u64 {
    let mut parser = Parser::new(bytes);

    let packet = parser.parse_packet().unwrap();

    packet.evaluate()
}

fn main() {
    let input = include_str!("day16.txt");
    let input = load_input(input);

    let part_one_answer = part_one(&input);
    let part_two_answer = part_two(&input);

    println!("PART ONE ANSWER: {:?}", part_one_answer);
    println!("PART TWO ANSWER: {:?}", part_two_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let input = "D2FE28";
        let input = load_input(input);

        let mut parser = Parser::new(&input);

        let packet = parser.parse_packet().unwrap();

        assert_eq!(packet.version, 6);

        if let PacketPayload::Literal { value } = packet.payload {
            assert_eq!(value, 2021);
        } else {
            panic!("didn't parse a literal");
        }
    }

    #[test]
    fn test_parse_bit_length_operator() {
        let input = "38006F45291200";
        let input = load_input(input);

        let mut parser = Parser::new(&input);

        let packet = parser.parse_packet().unwrap();

        assert_eq!(packet.version, 1);

        if let PacketPayload::Operator { operation, packets } = packet.payload {
            assert_eq!(operation, Operation::Less);
            assert_eq!(packets.len(), 2);
        } else {
            panic!("didn't parse an operator");
        }
    }

    #[test]
    fn test_parse_packet_count_operator() {
        let input = "EE00D40C823060";
        let input = load_input(input);

        let mut parser = Parser::new(&input);

        let packet = parser.parse_packet().unwrap();

        assert_eq!(packet.version, 7);

        if let PacketPayload::Operator { operation, packets } = packet.payload {
            assert_eq!(operation, Operation::Max);
            assert_eq!(packets.len(), 3);
        } else {
            panic!("didn't parse an operator");
        }
    }

    #[test]
    fn test_packet_sums() {
        let input = load_input("8A004A801A8002F478");
        assert_eq!(part_one(&input), 16);

        let input = load_input("620080001611562C8802118E34");
        assert_eq!(part_one(&input), 12);

        let input = load_input("C0015000016115A2E0802F182340");
        assert_eq!(part_one(&input), 23);

        let input = load_input("A0016C880162017C3686B18A3D4780");
        assert_eq!(part_one(&input), 31);
    }

    #[test]
    fn test_part_two_samples() {
        let input = load_input("C200B40A82");
        assert_eq!(part_two(&input), 3);

        let input = load_input("04005AC33890");
        assert_eq!(part_two(&input), 54);

        let input = load_input("880086C3E88112");
        assert_eq!(part_two(&input), 7);

        let input = load_input("CE00C43D881120");
        assert_eq!(part_two(&input), 9);

        let input = load_input("D8005AC2A8F0");
        assert_eq!(part_two(&input), 1);

        let input = load_input("F600BC2D8F");
        assert_eq!(part_two(&input), 0);

        let input = load_input("9C005AC2F8F0");
        assert_eq!(part_two(&input), 0);

        let input = load_input("9C0141080250320F1802104A08");
        assert_eq!(part_two(&input), 1);
    }
}
