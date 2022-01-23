use byteorder::{ByteOrder, LittleEndian};

fn main() {
    //---------------Packet A---------------
    let mut buffer = Buffer::new(100);
    let packet = Packet::A(PacketA { x: 10, y: 15, z: 20 });
    packet.write(&mut buffer);
    println!("{:?}", packet);
    buffer.index = 0;
    println!("{:?}\n", Packet::new(&mut buffer));
    //---------------Packet B---------------
    let mut buffer = Buffer::new(100);
    let packet = Packet::B(PacketB { elements: vec![1, 2, 3, 4, 5], num_elements: 5 });
    packet.write(&mut buffer);
    println!("{:?}", packet);
    buffer.index = 0;
    println!("{:?}\n", Packet::new(&mut buffer));
    //---------------Packet C---------------
    let mut buffer = Buffer::new(100);
    let packet = Packet::C(PacketC { x: true, y: 7, z: 13 });
    packet.write(&mut buffer);
    println!("{:?}", packet);
    buffer.index = 0;
    println!("{:?}", Packet::new(&mut buffer));
}

// #[derive(Debug)]
enum PacketType { PacketA, PacketB, PacketC }

#[derive(Debug)]
enum Packet {
    A(PacketA),
    B(PacketB),
    C(PacketC),
}

impl Packet {
    fn write(&self, buffer: &mut Buffer) {
        match self {
            Packet::A(packet) => {
                write_byte(buffer, PacketType::PacketA as u8);
                packet.write(buffer);
            },
            Packet::B(packet) => {
                write_byte(buffer, PacketType::PacketB as u8);
                packet.write(buffer)
            },
            Packet::C(packet) => {
                write_byte(buffer, PacketType::PacketC as u8);
                packet.write(buffer)
            },
            _ => unreachable!(),
        }
    }

    fn read(&mut self, buffer: &mut Buffer) {
        match self {
            Packet::A(packet) => packet.read(buffer),
            Packet::B(packet) => packet.read(buffer),
            Packet::C(packet) => packet.read(buffer),
            _ => unreachable!(),
        };
    }

    fn new(buffer: &mut Buffer) -> Self {
        let packet_type = read_byte(buffer);
        let mut packet = match packet_type {
            x if x == PacketType::PacketA as u8 => Packet::A(PacketA { x: 0, y: 0, z: 0}),
            x if x == PacketType::PacketB as u8 => Packet::B(PacketB { num_elements: 0, elements: vec![] }),
            x if x == PacketType::PacketC as u8 => Packet::C(PacketC { x: false, y: 0, z: 0 }),
            _ => unreachable!(),
        };
        packet.read(buffer);
        packet
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct PacketA {
    x: u32,
    y: u32,
    z: u32,
}

impl PacketA {
    fn write(&self, buffer: &mut Buffer) {
        write_int(buffer, self.x);
        write_int(buffer, self.y);
        write_int(buffer, self.z);
    }

    fn read(&mut self, buffer: &mut Buffer) {
        self.x = read_int(buffer);
        self.y = read_int(buffer);
        self.z = read_int(buffer);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PacketB {
    num_elements: u32,
    elements: Vec<u32>,
}

impl PacketB {
    fn write(&self, buffer: &mut Buffer) {
        write_int(buffer, self.num_elements);
        for i in 0..self.num_elements {
            write_int(buffer, self.elements[i as usize]);
        }
    }

    fn read(&mut self, buffer: &mut Buffer) {
        self.num_elements = read_int(buffer);
        for i in 0..self.num_elements {
            self.elements.push(read_int(buffer));
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct PacketC {
    x: bool,
    y: u16,
    z: u32,
}

impl PacketC {
    fn write(&self, buffer: &mut Buffer) {
        write_byte(buffer, self.x as u8);
        write_short(buffer, self.y);
        write_int(buffer, self.z);
    }

    fn read(&mut self, buffer: &mut Buffer) {
        self.x = read_byte(buffer) == 1;
        self.y = read_short(buffer);
        self.z = read_int(buffer);
    }
}

#[derive(Debug)]
struct Buffer {
    data: Vec<u8>,
    size: usize,
    index: usize,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            size,
            index: 0,
        }
    }
}

fn write_int(buffer: &mut Buffer, value: u32) {
    assert!(buffer.index + 4 <= buffer.size);
    LittleEndian::write_u32(&mut buffer.data[buffer.index..buffer.index+4], value);
    buffer.index += 4;
}

fn write_short(buffer: &mut Buffer, value: u16) {
    assert!(buffer.index + 2 <= buffer.size);
    LittleEndian::write_u16(&mut buffer.data[buffer.index..buffer.index+2], value);
    buffer.index += 2;
}

fn write_byte(buffer: &mut Buffer, value: u8) {
    assert!(buffer.index <= buffer.size);
    buffer.data[buffer.index] = value;
    buffer.index += 1;
}

fn read_int(buffer: &mut Buffer) -> u32 {
    assert!(buffer.index + 4 <= buffer.size);
    buffer.index += 4;
    LittleEndian::read_u32(&mut buffer.data[buffer.index-4..buffer.index])
}

fn read_short(buffer: &mut Buffer) -> u16 {
    assert!(buffer.index + 2 <= buffer.size);
    buffer.index += 2;
    LittleEndian::read_u16(&mut buffer.data[buffer.index-2..buffer.index])
}

fn read_byte(buffer: &mut Buffer) -> u8 {
    assert!(buffer.index <= buffer.size);
    buffer.index += 1;
    buffer.data[buffer.index-1]
}