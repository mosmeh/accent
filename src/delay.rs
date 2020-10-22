pub struct Delay {
    buffer: Vec<f64>,
    read_ptr: usize,
    write_ptr: usize,
}

impl Delay {
    pub fn new(length: usize) -> Self {
        let buf_len = length.next_power_of_two();
        Self {
            buffer: vec![0.0; buf_len],
            read_ptr: 0,
            write_ptr: if length == buf_len {
                0 // wrap around
            } else {
                length
            },
        }
    }

    pub fn input(&mut self, x: f64) {
        self.buffer[self.write_ptr] = x;
        self.read_ptr = (self.read_ptr + 1) & (self.buffer.len() - 1);
        self.write_ptr = (self.write_ptr + 1) & (self.buffer.len() - 1);
    }

    pub fn output(&self) -> f64 {
        self.buffer[self.read_ptr]
    }
}
