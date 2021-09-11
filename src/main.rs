use crate::win_smbios::win_smbios::read_smbios;

mod win_smbios;
//pub use win_smbios;

struct Type0 {
    Type : u8,
    Len : u8,
    Handle : u16,
    Vendor : String,
    BIOSVersion : String,
    BIOSStartAddrSeg : u16,
    BIOSRelDate : String,
    BIOSRomSz : u8,
    BIOSChar : u64,
    BIOSCharExt : u16,
    SystemBIOSMajRel : u8,
    SystemBIOSMinRel : u8,
    EmbCtrlrFwMajRel : u8,
    EmbCtrlrFwMinRel : u8,
    ExtBIOSRomSz : u16
}

fn main() {
    let data = read_smbios();

    //println!("Data size: {}", data.len());

    let maj = data[1];
    let min = data[2];

    // should we find end of strings first, capture strings, then populate structure?

    let beg = data[8] as usize;
    let _temp = Type0 {
        Type : data[beg],
        Len : data[beg + 1],
        Handle : (data[beg + 2] as u32 | ((data[beg + 3] as u32) << 8)) as u16,
        Vendor : "".to_string(),
        BIOSVersion : "".to_string(),
        BIOSStartAddrSeg : (data[beg + 6] as u32 | ((data[beg + 7] as u32) << 8)) as u16,
        BIOSRelDate : "".to_string(),
        BIOSRomSz : data[beg + 9],
        BIOSChar : (data[beg + 0xA] as u64 | ((data[beg + 0xB] as u64) << 8) | ((data[beg + 0xC] as u64) << 16) | ((data[beg + 0xD] as u64) << 24)
            | ((data[beg + 0xE] as u64) << 32)| ((data[beg + 0xF] as u64) << 40)| ((data[beg + 0x10] as u64) << 48)| ((data[beg + 0x11] as u64) << 56)),
        BIOSCharExt : 0u16,
        SystemBIOSMajRel : 0u8,
        SystemBIOSMinRel : 0u8,
        EmbCtrlrFwMajRel : 0u8,
        EmbCtrlrFwMinRel : 0u8,
        ExtBIOSRomSz : 0u16
    };

    println!("SMBIOS Version: {}.{}", maj, min);
    // for b in data.iter() {
    //     println!("{:02x}", b);
    // }

    // print type 0
    println!("Table: Type0");
    println!("\tType   : {}", _temp.Type);
    println!("\tLen    : {}", _temp.Len);
    println!("\tHandle : {}", _temp.Handle);
    println!("\tBIOSStartAddrSeg : {}", _temp.BIOSStartAddrSeg);
    println!("\tBIOSRomSz : {:#02x}", _temp.BIOSRomSz);
    println!("\tBiosChar : {:#016x}", _temp.BIOSChar);
}
