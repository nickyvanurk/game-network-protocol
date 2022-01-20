fn main() {
    let mut buffer = Buffer {
        data: &mut [0; 100][0],
        size: 100,
        index: 0,
    };

    write_integer(&mut buffer, 42);

    buffer.index = 0;

    println!("{:?}", read_integer(&mut buffer));
}

struct Buffer {
    data: *mut u8,
    size: usize,
    index: usize,
}

fn write_integer(buffer: &mut Buffer, value: u32) {
    assert!(buffer.index + 4 <= buffer.size);
    unsafe {
        *(buffer.data.add(buffer.index) as *mut u32) = value;
    }
    buffer.index += 4;
}

fn read_integer(buffer: &mut Buffer) -> u32 {
    assert!(buffer.index + 4 <= buffer.size);
    let value = unsafe {
        *(buffer.data.add(buffer.index) as *const u32)
    };
    return value;
}