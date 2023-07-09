// - Memory
//   - .get_device_memory(offset: u8)
// - DeviceMemory
//   // A DeviceMemory is kept up-to-date with a receive-only channel from its parent Memory
//   // And it updates Memory with a send-only channel
//   - mask (u16 bit map of which ports to listen to)
//   - send: channel<[u8; 16]>
//   - receive: channel<u8; 16]>

pub struct DeviceMemory {
    memory: [u8; 0x10],
}

impl DeviceMemory {
    pub fn new(memory: [u8; 0x10]) -> DeviceMemory {
        DeviceMemory { memory }
    }

    pub fn read_byte(&self, address: u8) -> u8 {
        self.memory[usize::try_from(address).unwrap()]
    }

    pub fn read_short(&self, address: u8) -> u16 {
        let b1 = self.read_byte(address) as u16;
        let b2 = self.read_byte(address + 1) as u16;
        b1 << 8 | b2
    }
}
