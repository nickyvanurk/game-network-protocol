use byteorder::{ByteOrder, LittleEndian};

const MAX_PACKET_SIZE: usize = 20;

fn main() {
    let mut buffer = [0; MAX_PACKET_SIZE];

    //---------------BitWriter------------
    let mut writer = BitWriter::new(&mut buffer);
    println!("Writer: {:?}", writer);

    writer.write_bits(42, 6);
    writer.flush_bits();
    println!("Write 42: {:?}", writer);

    writer.write_align();
    writer.flush_bits();
    println!("Write align: {:?}\n", writer);

    //---------------WriteStream------------
    let mut write_stream = WriteStream::new(writer);
    println!("{:?}", write_stream);

    write_stream.serialize_integer(42, 0, 60);
    write_stream.flush();
    println!("{:?}\n", write_stream);

    //---------------BitReader------------
    let mut reader = BitReader::new(&mut buffer);
    println!("Reader: {:?}", reader);

    let output = reader.read_bits(6);
    println!("Read 6 bits: {:?}", reader);
    println!("Output: {:?}", output);

    if reader.read_align() {
        println!("Read align: {:?}\n", reader);
    }

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

trait Stream {
    fn serialize_integer(&mut self, value: i32, min: i32, max: i32) -> bool;
    fn flush(&mut self);
}

#[derive(Debug)]
struct WriteStream<'a > {
    is_writing: bool,
    is_reading: bool,
    writer: BitWriter<'a >,
}

impl<'a> WriteStream<'a> {
    fn new(writer: BitWriter<'a>) -> Self {
        Self {
            is_writing: true,
            is_reading: false,
            writer,
        }
    }
}

impl<'a> Stream for WriteStream<'a> {
    fn serialize_integer(&mut self, value: i32, min: i32, max: i32) -> bool {
        assert!(min < max);
        assert!(value >= min);
        assert!(value <= max);
        let bits = bits_required(min as u32, max as u32);
        let unsigned_value = (value - min) as u32;
        self.writer.write_bits(unsigned_value, bits);
        true
    }

    fn flush (&mut self) {
        self.writer.flush_bits();
    }
}

#[derive(Debug)]
struct BitWriter<'a> {
    buffer: &'a mut [u32],
    scratch: u64,
    num_bits: u32,
    num_words: usize,
    bits_written: u32,
    word_index: usize,
    scratch_bits: u32,
}

impl<'a> BitWriter<'a> {
    fn new(buffer: &'a mut [u32]) -> Self {
        let buffer_size = buffer.len();
        assert!(buffer_size % 4 == 0);
        Self {
            buffer: buffer,
            scratch: 0,
            num_words: (buffer_size / 4),
            num_bits: (buffer_size / 4) as u32 * 32,
            bits_written: 0,
            word_index: 0,
            scratch_bits: 0,
        }
    }

    fn write_bits(&mut self, mut value: u32, bits: u32) {
        assert!(bits <= 32);
        assert!(self.bits_written + bits <= self.num_bits);

        value &= ((1_u64 << bits) - 1) as u32; // is u64 required here?

        self.scratch |= (value as u64) << self.scratch_bits;
        self.scratch_bits += bits;

        if self.scratch_bits >= 32 {
            self.flush_bits();
        }

        self.bits_written += bits;
    }

    fn flush_bits(&mut self) {
        if self.scratch_bits > 0 {
            assert!(self.word_index < self.num_words);
            self.buffer[self.word_index] = (self.scratch & 0xFFFFFFFF) as u32;
            self.scratch >>= 32;
            self.scratch_bits -= if self.scratch_bits >= 32 { 32 } else { self.scratch_bits };
            self.word_index += 1;
        }
    }

    fn write_align(&mut self) {
        let remainder_bits = self.bits_written % 8;
        if  remainder_bits != 0 {
            let zero = 0_u32;
            self.write_bits(zero, 8 - remainder_bits);
            assert!((self.bits_written % 8) == 0);
        }
    }
}

#[derive(Debug)]
struct BitReader<'a> {
    buffer: &'a mut [u32],
    scratch: u64,
    num_bits: u32,
    num_words: usize,
    bits_read: u32,
    word_index: usize,
    scratch_bits: u32,
}

impl<'a> BitReader<'a> {
    fn new(buffer: &'a mut [u32]) -> Self {
        let buffer_size = buffer.len();
        assert!(buffer_size % 4 == 0);
        Self {
            buffer: buffer,
            scratch: 0,
            num_words: (buffer_size + 3) / 4,
            num_bits: (buffer_size * 8) as u32,
            bits_read: 0,
            word_index: 0,
            scratch_bits: 0,
        }
    }

    fn read_bits(&mut self, bits: u32) -> u32 {
        assert!(bits <= 32);
        assert!(self.bits_read + bits <= self.num_bits);

        self.bits_read += bits;

        assert!(self.scratch_bits <= 64);

        if (self.scratch_bits < bits) {
            assert!(self.word_index < self.num_words);
            self.scratch |= (self.buffer[self.word_index] as u64) << self.scratch_bits;
            self.scratch_bits += 32;
            self.word_index += 1;
        }

        assert!(self.scratch_bits >= bits);

        let output = self.scratch & ((1_u64 << bits) - 1);

        self.scratch >>= bits;
        self.scratch_bits -= bits;

        output as u32
    }

    fn read_align(&mut self) -> bool {
        let remainder_bits = self.bits_read % 8;
        if remainder_bits != 0 {
            let value = self.read_bits(8 - remainder_bits);
            assert!(self.bits_read % 8 == 0);
            if value != 0 {
                return false;
            }
        }

        true
    }
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

fn bits_required(min: u32, max: u32) -> u32 {
    if min == max { 0 } else { log2(max - min) + 1 }
}

fn log2(x: u32) -> u32 {
    let a = x | (x >> 1);
    let b = a | (a >> 2);
    let c = b | (b >> 4);
    let d = c | (c >> 8);
    let e = d | (d >> 16);
    let f = e >> 1;
    popcount(f)
}

fn popcount(x: u32) -> u32 {
    let a = x - (( x >> 1)      & 0x55555555);
    let b =     (( a >> 2)      & 0x33333333) + (a & 0x33333333);
    let c =     (( b >> 4) + b) & 0x0f0f0f0f;
    let d =   c + (c >> 8);
    let e =   d + (d >> 16);
    e & 0x0000003f
}