use crate::win_smbios::win_smbios::read_smbios;

mod win_smbios;
mod util;
//pub use win_smbios;

struct Type0 {
    table_type : u8,
    len : u8,
    handle : u16,
    vendor : String,
    bios_version : String,
    bios_start_addr_seg : u16,
    bios_rel_date : String,
    bios_rom_sz : u8,
    bios_char : u64,
    bios_char_ext : u16,
    system_bios_maj_rel : u8,
    system_bios_min_rel : u8,
    emb_ctrlr_fw_maj_rel : u8,
    emb_ctrlr_fw_min_rel : u8,
    ext_bios_rom_sz : u16
}

fn main() {
    let data = read_smbios();

    //println!("Data size: {}", data.len());

    let maj = data[1];
    let min = data[2];

    // should we find end of strings first, capture strings, then populate structure?

    let beg = data[8] as usize;
    let _temp = Type0 {
        table_type : data[beg],
        len : data[beg + 1],
        handle : (data[beg + 2] as u32 | ((data[beg + 3] as u32) << 8)) as u16,
        vendor : "".to_string(),
        bios_version : "".to_string(),
        bios_start_addr_seg : (data[beg + 6] as u32 | ((data[beg + 7] as u32) << 8)) as u16,
        bios_rel_date : "".to_string(),
        bios_rom_sz : data[beg + 9],
        bios_char : (data[beg + 0xA] as u64 | ((data[beg + 0xB] as u64) << 8) | ((data[beg + 0xC] as u64) << 16) | ((data[beg + 0xD] as u64) << 24)
            | ((data[beg + 0xE] as u64) << 32)| ((data[beg + 0xF] as u64) << 40)| ((data[beg + 0x10] as u64) << 48)| ((data[beg + 0x11] as u64) << 56)),
        bios_char_ext : 0u16,
        system_bios_maj_rel : 0u8,
        system_bios_min_rel : 0u8,
        emb_ctrlr_fw_maj_rel : 0u8,
        emb_ctrlr_fw_min_rel : 0u8,
        ext_bios_rom_sz : 0u16
    };

    println!("SMBIOS Version: {}.{}", maj, min);
    // for b in data.iter() {
    //     println!("{:02x}", b);
    // }

    // print type 0
    println!("Table: Type0");
    println!("\tType   : {}", _temp.table_type);
    println!("\tLen    : {}", _temp.len);
    println!("\tHandle : {}", _temp.handle);
    println!("\tBIOSStartAddrSeg : {}", _temp.bios_start_addr_seg);
    println!("\tBIOSRomSz : {:#02x}", _temp.bios_rom_sz);
    println!("\tBiosChar : {:#016x}", _temp.bios_char);
}
