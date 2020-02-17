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
    println!("Hello, world!");

    let data = read_smbios();

    //println!("Data size: {}", data.len());

    let maj = data[1];
    let min = data[2];

    // should we find end of strings first, capture strings, then populate structure?

    let beg = data[8] as usize;
    let temp = Type0 {
        Type : data[beg],
        Len : data[beg + 1],
        Handle : (data[beg + 2] | (data[beg + 3] << 1)) as u16,
        Vendor : "".to_string(),
        BIOSVersion : "".to_string(),
        BIOSStartAddrSeg : 0u16,
        BIOSRelDate : "".to_string(),
        BIOSRomSz : 0u8,
        BIOSChar : 0u64,
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
}
