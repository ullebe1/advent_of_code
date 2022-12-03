use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
enum PacketType {
    TypeFour(FourPacket),
    Operator(OperatorPacket),
}

#[derive(Clone, Debug)]
enum LengthType {
    ByteLength(usize),
    NoOfPackets(usize),
}

#[derive(Clone, Debug)]
struct OperatorPacket {
    length_type: LengthType,
    content: Vec<PacketType>,
    packet_type: usize,
    version: usize,
}

#[derive(Clone, Debug)]
pub struct FourPacket {
    version: usize,
    number: usize,
}

pub struct Input {
    packet: PacketType,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let hex_string = input.lines().next().unwrap().chars();

    let binary_string: String = hex_string
        .map(|x| match x {
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
            _ => "",
        })
        .collect();

    let (packet, _) = parse_packet(binary_string.chars().collect::<Vec<char>>().as_slice());

    Input { packet: packet }
}

fn parse_packet(content: &[char]) -> (PacketType, &[char]) {
    let packet_version =
        usize::from_str_radix(&content[0..3].into_iter().collect::<String>(), 2).unwrap();
    let packet_type =
        usize::from_str_radix(&content[3..6].into_iter().collect::<String>(), 2).unwrap();
    match packet_type {
        4 => parse_type_four(packet_type, packet_version, &content[6..]),
        _ => parse_type_operator(packet_type, packet_version, &content[6..]),
    }
}

fn parse_type_four(
    packet_type: usize,
    packet_version: usize,
    content: &[char],
) -> (PacketType, &[char]) {
    let mut literal = Vec::new();
    let mut offset = 0;

    loop {
        match content[offset] {
            '0' => {
                literal.append(&mut content[offset + 1..offset + 5].to_vec());
                offset += 5;
                println!("PacketFour");
                break;
            }
            '1' => {
                literal.append(&mut content[offset + 1..offset + 5].to_vec());
                offset += 5;
            }
            _ => unreachable!(),
        }
    }

    (
        PacketType::TypeFour(FourPacket {
            version: packet_version,
            number: usize::from_str_radix(&literal.into_iter().collect::<String>(), 2).unwrap(),
        }),
        if content.len() > offset {
            &content[offset..]
        } else {
            &content[0..0]
        },
    )
}

fn parse_type_operator(
    packet_type: usize,
    packet_version: usize,
    content: &[char],
) -> (PacketType, &[char]) {
    match content[0] {
        '0' => {
            println!("Entered type 0");
            let subpacket_length =
                usize::from_str_radix(&content[1..16].into_iter().collect::<String>(), 2).unwrap();

            let mut packets = Vec::new();
            let mut remaining = &content[16..16 + subpacket_length];

            loop {
                let (packet, rest) = parse_packet(remaining);
                packets.push(packet);
                if rest.is_empty() {
                    println!("break");
                    break;
                } else {
                    println!("Length of rest {:?}", rest.len());
                    remaining = rest;
                }
            }
            (
                PacketType::Operator(OperatorPacket {
                    length_type: LengthType::ByteLength(subpacket_length),
                    content: packets,
                    packet_type: packet_type,
                    version: packet_version,
                }),
                &content[subpacket_length + 1..],
            )
        }
        '1' => {
            println!("Entered type 1");
            let subpacket_number =
                usize::from_str_radix(&content[1..12].into_iter().collect::<String>(), 2).unwrap();

            let mut packets = Vec::with_capacity(subpacket_number);
            let mut remaining = &content[12..];

            for _ in 0..subpacket_number {
                let (packet, rest) = parse_packet(remaining);
                packets.push(packet);
                remaining = rest;
            }

            (
                PacketType::Operator(OperatorPacket {
                    length_type: LengthType::NoOfPackets(subpacket_number),
                    content: packets,
                    packet_type: packet_type,
                    version: packet_version,
                }),
                remaining,
            )
        }
        _ => unreachable!(),
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> usize {
    version_sum(input.packet.clone())
}

fn version_sum(packet: PacketType) -> usize {
    println!("Packet: {:#?}", packet);  
    match packet {
        PacketType::TypeFour(x) => x.version,
        PacketType::Operator(x) => x.content.into_iter().fold(x.version, |sum, y| {
            sum + match y {
                PacketType::TypeFour(z) => z.version,
                PacketType::Operator(z) => version_sum(PacketType::Operator(z)),
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = input_generator("8A004A801A8002F478");

        assert_eq!(part1(&input), 16);
    }

    #[test]
    fn part1_example2() {
        let input = input_generator("620080001611562C8802118E34");

        assert_eq!(part1(&input), 12);
    }

    #[test]
    fn part1_example3() {
        let input = input_generator("C0015000016115A2E0802F182340");

        assert_eq!(part1(&input), 23);
    }

    #[test]
    fn part1_example4() {
        let input = input_generator("A0016C880162017C3686B18A3D4780");

        assert_eq!(part1(&input), 31);
    }

    
}