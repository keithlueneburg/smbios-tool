use crate::win_smbios::win_smbios::read_smbios;
use crate::util::util::le_to_u16;
use crate::util::util::le_to_u64;

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
        handle : le_to_u16(&data[beg + 2 .. beg + 4]), 
        vendor : "".to_string(),
        bios_version : "".to_string(),
        bios_start_addr_seg : le_to_u16(&data[beg + 6 .. beg + 8]),
        bios_rel_date : "".to_string(),
        bios_rom_sz : data[beg + 9],
        bios_char : le_to_u64(&data[beg + 0xA .. beg + 0x12]),
        bios_char_ext : 0u16,
        system_bios_maj_rel : data[beg + 0x14],
        system_bios_min_rel : data[beg + 0x15],
        emb_ctrlr_fw_maj_rel : data[beg + 0x16],
        emb_ctrlr_fw_min_rel : data[beg + 0x17],
        ext_bios_rom_sz : 0u16 // Dev box is SMBIOS 2.7 so ignore for now
    };

    // TODO
    // 1. Get strings
    // 2. Get dependent fields (bios_char_ext byte count depends on table length)
    // 3. Only get fields supported by current SMBIOS version.. maybe.. Might make sense to wait for dynamic table definition and storage structure
    // 4. 
    //   a. Make to u16/u32/u64 function safer by copying to array
    //   b. Don't copy bytes if possible while keeping compile time check
    // 5. Move table defs to input file

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
    println!("\tBIOSChar : {:#016x}", _temp.bios_char);
    println!("\tSysBIOSMajRel : {}", _temp.system_bios_maj_rel);
    println!("\tSysBIOSMinRel : {}", _temp.system_bios_min_rel);
    println!("\tEmbCtrlrMajRel : {}", _temp.emb_ctrlr_fw_maj_rel);
    println!("\tEmbCtrlrMinRel : {}", _temp.emb_ctrlr_fw_min_rel);
}
