//use std::collections::{HashMap, HashSet};
use std::str;
use std::str::FromStr;
use log::debug;

#[derive(Debug)]
pub enum PacketType {
    Invalid,
    Literal(i64),
    OperatorSum(i64),
    OperatorProduct(i64),
    OperatorMinimum(i64),
    OperatorMaximum(i64),
    OperatorGreaterThan(i64),
    OperatorLessThan(i64),
    OperatorEqual(i64),
}

#[derive(Debug)]
pub struct Packet {
    version: i32,
    packet_type: PacketType,
    inner_packets: Vec<Packet>,
    length: usize, //in bits (WITHOUT sub-packets)
}

impl Packet {
    fn size_in_bits(&self) -> usize {
        self.length
            + self
                .inner_packets
                .iter()
                .map(|p| p.size_in_bits())
                .sum::<usize>()
    }

    fn get_value(&self) -> i64 {
        match &self.packet_type {
            PacketType::Literal(n) => *n,
            PacketType::OperatorSum(n) => *n,
            PacketType::OperatorProduct(n) => *n,
            PacketType::OperatorMinimum(n) => *n,
            PacketType::OperatorMaximum(n) => *n,
            PacketType::OperatorGreaterThan(n) => *n,
            PacketType::OperatorLessThan(n) => *n,
            PacketType::OperatorEqual(n) => *n,
            PacketType::Invalid => panic!(),
        }
    }
}

pub fn conv_to_bits(v: &str) -> String {
    v.chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Invalid"),
        })
        .collect::<String>()
}

fn conv_to_char(v: &str) -> char {
    if v.chars().count() > 4 {
        panic!("Too big");
    }

    let padded = format!("{:0>4}", v);
    match &padded[..] {
        "0000" => '0',
        "0001" => '1',
        "0010" => '2',
        "0011" => '3',
        "0100" => '4',
        "0101" => '5',
        "0110" => '6',
        "0111" => '7',
        "1000" => '8',
        "1001" => '9',
        "1010" => 'A',
        "1011" => 'B',
        "1100" => 'C',
        "1101" => 'D',
        "1110" => 'E',
        "1111" => 'F',
        _ => panic!("Invalid"),
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(bits: &str) -> Result<Self, Self::Err> {
        //Don't assume it's bits
        //let bits = conv_to_bits(s);
        let bits = bits.to_string();
        let it = bits.chars();

        if it.clone().count() < 6 {
            return Err("Packet too short".into());
        }

        let raw_version = it.clone().take(3).collect::<String>();
        let version = conv_to_char(&raw_version[..])
            .to_string()
            .parse::<i32>()
            .map_err(|_| "Bad Version".to_string())?;

        debug!("Version: {}", version);

        let raw_packet_type = it.clone().skip(3).take(3).collect::<String>();
        let packet_type = conv_to_char(&raw_packet_type[..]);

        debug!("Packet_Type: {}", packet_type);

        let packet_type_enum;

        let mut sub_packets = vec![];
        let mut bit_count: usize = 6;

        debug!("bits: {}", bits);
        debug!("pos:  XXXYYY-----------------------------");

        if packet_type == '4' {
            debug!("Literal version {} type {}", version, packet_type);
            //TODO: Check for what happens on a 32 bit literal
            //Literal value
            //Parse like so: groups of 5 bits, if 1, not last, if 0, then last
            let mut val = String::new();
            let mut chunks = bits[6..].as_bytes().chunks(5);
            loop {
                let chunk = str::from_utf8(chunks.next().unwrap()).unwrap();

                if chunk.chars().count() != 5 {
                    panic!();
                }

                debug!("Chunk {}", chunk);
                let first = chunk.chars().next().unwrap();
                val += &chunk[1..];

                bit_count += 5;

                if first == '0' {
                    break;
                }
            }
            //May be more bits, ignor them
            debug!("asdf {:?}",val);

            packet_type_enum = PacketType::Literal(i64::from_str_radix(&val[..], 2).unwrap());
        } else {
            //Operator parsing
            //First bit is length type id
            debug!("Operator parsing version: {} type {}", version, packet_type);
            let length_type_id = bits
                .chars()
                .nth(6)
                .ok_or("Packet too short parsing operator")?;

            debug!("length_type_id {}", length_type_id);
            bit_count += 1; //length type id

            if length_type_id == '0' {
                // 15 bits of how many bits
                debug!("Parsing by length");
                //Determine if it's even that long
                if bits.chars().count() < 22 {
                    return Err("Truncated packet determining length".into());
                }
                //15 bits are a number that represents the total length in bits of the sub-packets
                let length = u64::from_str_radix(&bits[7..(7 + 15)], 2).unwrap();
                bit_count += 15;
                debug!("Length: {}", length);
                // We know it's length bits, but we don't know how many packets are for each 27
                // bits could contain 1 or more packets
                let mut idx: usize = 0;
                //22 is start index
                loop {
                    //TODO: Do I need to truncate the packet here if we are about to reach length?
                    //I think so, this fails some tests, but seems more correct
                    if let Ok(packet) = Packet::from_str(&bits[(22 + idx)..length as usize + 22]) {
                        //if let Ok(packet) = Packet::from_str(&bits[(22 + idx)..]) {
                        idx += packet.size_in_bits();
                        debug!("Packet size: {}", packet.size_in_bits());
                        debug!("Size so far: {}", idx);
                        sub_packets.push(packet);
                        if idx >= length as usize {
                            debug!("Done parsing sub packets by length");
                            break;
                        }
                    } else {
                        debug!("Insufficient bits");
                        //Add additional_bits
                        //TODO: Is this correct? Containing packets need to count the extra bits?
                        //bit_count += bits[22+idx..].chars().count();
                        break; //hit traililng
                    }
                }
            } else {
                //11 bits of how many packets
                //This indicates how many packets there are following (but not their length...)
                if bits.chars().count() < 18 {
                    return Err("Truncated packet determining number of packets".into());
                }
                let num_packets = i64::from_str_radix(&bits[7..(7 + 11)], 2).unwrap();
                bit_count += 11;
                debug!("Sub packets contained: {}", num_packets);
                let mut idx: usize = 0;
                for _p_idx in 0..num_packets {
                    debug!("Parsing packet number {} out of {}", _p_idx + 1, num_packets);
                    match Packet::from_str(&bits[18 + idx..]) {
                        Ok(packet) => {
                            idx += packet.size_in_bits();
                            debug!("cPacket size: {}", packet.size_in_bits());
                            debug!("cSize so far: {}", idx);
                            sub_packets.push(packet);
                        }
                        //You need to count the extra bits as part of the message for containing
                        //packets?
                        //TODO: This really feels like it's a parse error if you get here, but the
                        //test fails without it.
                        //Err(_) => { bit_count += bits[18+idx..].chars().count(); break; },
                        Err(_) => {
                            panic!()
                        }
                    }
                }
                debug!("Done parsing sub packets by num");
            }

            //Part 2, parse the stuff
            packet_type_enum = match packet_type {
                '0' => PacketType::OperatorSum(sub_packets.iter().map(|sp| sp.get_value()).sum()),
                '1' => PacketType::OperatorProduct(
                    sub_packets.iter().map(|sp| sp.get_value()).product(),
                ),
                '2' => PacketType::OperatorMinimum(
                    sub_packets.iter().map(|sp| sp.get_value()).min().unwrap(),
                ),
                '3' => PacketType::OperatorMaximum(
                    sub_packets.iter().map(|sp| sp.get_value()).max().unwrap(),
                ),
                '5' => PacketType::OperatorGreaterThan(
                    if sub_packets[0].get_value() > sub_packets[1].get_value() {
                        1
                    } else {
                        0
                    },
                ),
                '6' => PacketType::OperatorLessThan(
                    if sub_packets[0].get_value() < sub_packets[1].get_value() {
                        1
                    } else {
                        0
                    },
                ),
                '7' => PacketType::OperatorEqual(
                    if sub_packets[0].get_value() == sub_packets[1].get_value() {
                        1
                    } else {
                        0
                    },
                ),
                _ => panic!(),
            };
        }

        Ok(Packet {
            packet_type: packet_type_enum,
            version: version,
            inner_packets: sub_packets,
            length: bit_count,
        })
    }
}

#[aoc_generator(day16)]
fn day16_parse(input: &str) -> Packet {
    Packet::from_str(&conv_to_bits(input)).unwrap()
}

fn sum_versions(packet: &Packet) -> u128 {
    let mut version_sum = 0;
    let mut queue = vec![packet];
    while !queue.is_empty() {
        let packet = queue.pop().unwrap();
        debug!("Packet: {:?}",packet);
        version_sum += packet.version;
        for s_packet in packet.inner_packets.iter() {
            queue.push(s_packet);
        }
    }
    version_sum as u128
}

#[aoc(day16, part1)]
pub fn day16_part1(packet: &Packet) -> u128 {
    env_logger::init(); 
    sum_versions(packet)
}

#[aoc(day16, part2)]
pub fn day16_part2(packet: &Packet) -> u128 {
    env_logger::init(); 
    packet.get_value() as u128
}

#[cfg(test)]
mod tests {

    use test_env_log::test;
    use super::*;


    #[test]
    fn day16_parse_literal() {
        let hex = "D2FE28";
        let packet = super::Packet::from_str(&conv_to_bits(hex));
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        if let super::PacketType::Literal(v) = packet.packet_type {
            assert_eq!(v, 2021);
        } else {
            assert!(false);
        }
    }

    #[allow(deprecated)]

    #[test]
    fn day16_parse_type0() {
        let hex = "38006F45291200";
        let packet = super::Packet::from_str(&conv_to_bits(hex));
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        assert_eq!(packet.inner_packets.iter().count(), 2);

        let pack_zero = packet.inner_packets.get(0).unwrap();
        assert_eq!(pack_zero.size_in_bits(), 11);
        assert!(matches!(
            pack_zero.packet_type,
            super::PacketType::Literal(_)
        ));
        if let super::PacketType::Literal(v) = pack_zero.packet_type {
            assert_eq!(v, 10);
        }

        let pack_one = packet.inner_packets.get(1).unwrap();
        assert_eq!(pack_one.size_in_bits(), 16);
        assert!(matches!(
            pack_one.packet_type,
            super::PacketType::Literal(_)
        ));
        if let super::PacketType::Literal(v) = pack_one.packet_type {
            assert_eq!(v, 20);
        }
    }

    #[test]
    fn day16_parse_type1() {
        let hex = "EE00D40C823060";
        let packet = super::Packet::from_str(&conv_to_bits(hex));
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        assert_eq!(packet.inner_packets.iter().count(), 3);

        let pack_zero = packet.inner_packets.get(0).unwrap();
        assert_eq!(pack_zero.size_in_bits(), 11);
        assert!(matches!(
            pack_zero.packet_type,
            super::PacketType::Literal(_)
        ));
        if let super::PacketType::Literal(v) = pack_zero.packet_type {
            assert_eq!(v, 1);
        }
        let pack_one = packet.inner_packets.get(1).unwrap();
        assert_eq!(pack_one.size_in_bits(), 11);
        assert!(matches!(
            pack_one.packet_type,
            super::PacketType::Literal(_)
        ));
        if let super::PacketType::Literal(v) = pack_one.packet_type {
            assert_eq!(v, 2);
        }

        let pack_three = packet.inner_packets.get(2).unwrap();
        assert_eq!(pack_three.size_in_bits(), 11);
        assert!(matches!(
            pack_three.packet_type,
            super::PacketType::Literal(_)
        ));
        if let super::PacketType::Literal(v) = pack_three.packet_type {
            assert_eq!(v, 3);
        }
    }

    #[test]
    fn day16_parse_example1() {
        let hex = "8A004A801A8002F478";
        //represents an operator packet (version 4) which contains an operator packet (version 1) which contains an operator packet (version 5) which contains a literal value (version 6); this packet has a version sum of 16.
        assert_eq!(
            sum_versions(&super::Packet::from_str(&conv_to_bits(hex)).unwrap()),
            16
        );
    }
    #[test]
    fn day16_parse_example2() {
        let hex = "620080001611562C8802118E34";
        //620080001611562C8802118E34 represents an operator packet (version 3) which contains two sub-packets; each sub-packet is an operator packet that contains two literal values. This packet has a version sum of 12.
        assert_eq!(
            sum_versions(&super::Packet::from_str(&conv_to_bits(hex)).unwrap()),
            12
        );
    }
    #[test]
    fn day16_parse_example3() {
        let hex = "C0015000016115A2E0802F182340";
        //C0015000016115A2E0802F182340 has the same structure as the previous example, but the outermost packet uses a different length type ID. This packet has a version sum of 23.
        assert_eq!(
            sum_versions(&super::Packet::from_str(&conv_to_bits(hex)).unwrap()),
            23
        );
    }

    #[test]
    fn day16_parse_example4() {
        let hex = "A0016C880162017C3686B18A3D4780";
        //A0016C880162017C3686B18A3D4780 is an operator packet that contains an operator packet that contains an operator packet that contains five literal values; it has a version sum of 31
        assert_eq!(
            sum_versions(&super::Packet::from_str(&conv_to_bits(hex)).unwrap()),
            31
        );
    }

    
    
    #[test]
    fn day16_part2_example1() {
        let hex = "C200B40A82";
        let packet = super::Packet::from_str(&conv_to_bits(hex)).unwrap();
        assert_eq!(packet.get_value(), 3);
    }
    
    #[test]
    fn day16_part2_example2() {
        let hex = "04005AC33890";
        let packet = super::Packet::from_str(&conv_to_bits(hex)).unwrap();
        assert_eq!(packet.get_value(), 54);
    }
    
    #[test]
    fn day16_part2_example3() {
        let hex = "880086C3E88112";
        let packet = super::Packet::from_str(&conv_to_bits(hex)).unwrap();
        assert_eq!(packet.get_value(), 7);
    }
    
    #[test]
    fn day16_part2_example4() {
        let hex = "CE00C43D881120";
        let packet = super::Packet::from_str(&conv_to_bits(hex)).unwrap();
        assert_eq!(packet.get_value(), 9);
    }
    
    #[test]
    fn day16_part2_example5() {
        let hex = "D8005AC2A8F0";
        let packet = super::Packet::from_str(&conv_to_bits(hex)).unwrap();
        assert_eq!(packet.get_value(), 1);
    }
    
    #[test]
    fn day16_part2_example6() {
        let hex = "F600BC2D8F";
        let packet = super::Packet::from_str(&conv_to_bits(hex)).unwrap();
        assert_eq!(packet.get_value(), 0);
    }
    
    #[test]
    fn day16_part2_example7() {
        let hex = "9C005AC2F8F0";
        let packet = super::Packet::from_str(&conv_to_bits(hex)).unwrap();
        assert_eq!(packet.get_value(), 0);
    }
    
    #[test]
    fn day16_part2_example8() {
        let hex = "9C0141080250320F1802104A08";
        let packet = super::Packet::from_str(&conv_to_bits(hex)).unwrap();
        assert_eq!(packet.get_value(), 1);
    }


}
