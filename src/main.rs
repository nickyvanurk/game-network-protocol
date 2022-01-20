use byteorder::{ByteOrder, LittleEndian};

fn main() {
    let mut buffer = Buffer::new(100);

    write_integer(&mut buffer, 42);
    write_short(&mut buffer, 17);
    write_char(&mut buffer, 2);
    
    buffer.index = 0;

    println!("{:?}", read_integer(&mut buffer));
    println!("{:?}", read_short(&mut buffer));
    println!("{:?}", read_char(&mut buffer));
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

fn write_char(buffer: &mut Buffer, value: u8) {
    assert!(buffer.index <= buffer.size);
    buffer.data[buffer.index+1] = value;
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

fn read_char(buffer: &mut Buffer) -> u8 {
    assert!(buffer.index <= buffer.size);
    buffer.data[buffer.index+1]
}