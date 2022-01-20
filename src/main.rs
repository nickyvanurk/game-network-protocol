use byteorder::{ByteOrder, LittleEndian};

fn main() {
    let mut buffer = Buffer {
        data: &mut [0; 100],
        size: 100,
        index: 0,
    };

    write_integer(&mut buffer, 42);

    buffer.index = 0;

    println!("{:?}", read_integer(&mut buffer));
}

struct Buffer<'a> {
    data: &'a mut [u8],
    size: usize,
    index: usize,
}

fn write_integer(buffer: &mut Buffer, value: u32) {
    assert!(buffer.index + 4 <= buffer.size);
    LittleEndian::write_u32(&mut buffer.data[buffer.index..buffer.index+4], value);
    buffer.index += 4;
}

fn read_integer(buffer: &mut Buffer) -> u32 {
    assert!(buffer.index + 4 <= buffer.size);
    return LittleEndian::read_u32(&mut buffer.data[buffer.index..buffer.index+4]);
}