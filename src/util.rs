#[allow(dead_code)]
pub mod util {
    // TODO: Make these type safe
    //       Might need to copy to array and pass sized array? 
    //       Or is there some way to borrow a 'sized slice'?

    pub fn le_to_u16(bytes: &[u8]) -> u16 {
        (bytes[0] as u16) | 
        ((bytes[1] as u16) << 8)
    }

    pub fn le_to_u32(bytes: &[u8]) -> u32 {
        (bytes[0] as u32) | 
        ((bytes[1] as u32) << 8)  |
        ((bytes[2] as u32) << 16) |
        ((bytes[3] as u32) << 24)
    }

    pub fn le_to_u64(bytes: &[u8]) -> u64 {
        (bytes[0] as u64) | 
        ((bytes[1] as u64) << 8)  |
        ((bytes[2] as u64) << 16) |
        ((bytes[3] as u64) << 24) |
        ((bytes[4] as u64) << 32) |
        ((bytes[5] as u64) << 40) |
        ((bytes[6] as u64) << 48) |
        ((bytes[7] as u64) << 56)
    }
}