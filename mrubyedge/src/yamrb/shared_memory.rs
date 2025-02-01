use std::pin::Pin;

#[derive(Debug)]
pub struct SharedMemory {
    pub memory: Pin<Box<[u8]>>,
    pub size: usize,
}

impl SharedMemory {
    pub fn new(size: usize) -> Self {
        let memory = vec![0u8; size].into_boxed_slice();
        let memory = Pin::new(memory);
        SharedMemory { memory, size }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.memory.as_mut_ptr()
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) {
        self.memory[offset..offset + data.len()].copy_from_slice(data);
    }

    pub fn read(&self, offset: usize, size: usize) -> Vec<u8> {
        self.memory[offset..offset + size].to_vec()
    }

    pub fn read_u8(&self, offset: usize) -> u8 {
        self.memory[offset]
    }
}