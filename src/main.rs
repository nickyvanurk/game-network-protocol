use byteorder::{ByteOrder, LittleEndian};

fn main() {
    //---------------Packet A---------------
    let mut buffer = Buffer::new(100);
    let mut packet_a = PacketA { x: 10, y: 15, z: 20 };
    packet_a.write(&mut buffer);
    println!("{:?}", packet_a);

    buffer.index = 0;
    
    let mut packet_b = PacketA { x: 0, y: 0, z: 0 };
    packet_b.read(&mut buffer);
    println!("{:?}", packet_b);

    assert_eq!(packet_a, packet_b);

    //---------------Packet B---------------
    let mut buffer = Buffer::new(100);
    let mut packet_a = PacketB { elements: vec![1, 2, 3, 4, 5], num_elements: 5 };
    packet_a.write(&mut buffer);
    println!("{:?}", packet_a);

    buffer.index = 0;
    
    let mut packet_b = PacketB { num_elements: 0, elements: vec![] };
    packet_b.read(&mut buffer);
    println!("{:?}", packet_b);

    assert_eq!(packet_a, packet_b);

    //---------------Packet C---------------
    let mut buffer = Buffer::new(100);
    let mut packet_a = PacketC { x: true, y: 7, z: 13 };
    packet_a.write(&mut buffer);
    println!("{:?}", packet_a);

    buffer.index = 0;

    let mut packet_b = PacketC { x: false, y: 0, z: 0 };
    packet_b.read(&mut buffer);
    println!("{:?}", packet_b);

    assert_eq!(packet_a, packet_b);
}

#[derive(Debug, Eq, PartialEq)]
struct PacketA {
    x: u32,
    y: u32,
    z: u32,
}

impl PacketA {
    fn write(&mut self, buffer: &mut Buffer) {
        write_integer(buffer, self.x);
        write_integer(buffer, self.y);
        write_integer(buffer, self.z);
    }

    fn read(&mut self, buffer: &mut Buffer) {
        self.x = read_integer(buffer);
        self.y = read_integer(buffer);
        self.z = read_integer(buffer);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PacketB {
    num_elements: u32,
    elements: Vec<u32>,
}

impl PacketB {
    fn write(&mut self, buffer: &mut Buffer) {
        write_integer(buffer, self.num_elements);
        for i in 0..self.num_elements {
            write_integer(buffer, self.elements[i as usize]);
        }
    }

    fn read(&mut self, buffer: &mut Buffer) {
        self.num_elements = read_integer(buffer);
        for i in 0..self.num_elements {
            self.elements.push(read_integer(buffer));
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PacketC {
    x: bool,
    y: u16,
    z: u32,
}

impl PacketC {
    fn write(&mut self, buffer: &mut Buffer) {
        write_byte(buffer, self.x as u8);
        write_short(buffer, self.y);
        write_integer(buffer, self.z);
    }

    fn read(&mut self, buffer: &mut Buffer) {
        self.x = read_byte(buffer) == 1;
        self.y = read_short(buffer);
        self.z = read_integer(buffer);
    }
}

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

fn write_integer(buffer: &mut Buffer, value: u32) {
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

fn read_integer(buffer: &mut Buffer) -> u32 {
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