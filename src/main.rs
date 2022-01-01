use std::fs;

use crate::win_smbios::win_smbios::read_smbios;
use crate::util::util::le_to_u16;
use crate::util::util::le_to_u64;

mod win_smbios;
mod util;
mod tables;

#[allow(unused_variables)] //temp
fn main() {
    test_json();

    let data = read_smbios();

    let maj = data[1];
    let min = data[2];

    //println!("SMBIOS Version: {}.{}", maj, min);

    // Table begin
    // 1st byte of table data in structure returned by windows API
    let beg = 8 as usize; 
    let table_len = data[beg + 1];

    // Get strings for this Type 0 table
    let strings = tables::get_table_strings(data.as_slice(), beg + table_len as usize);

    let type0 = tables::Type0 {
        table_type : data[beg],
        len : table_len,
        handle : le_to_u16(&data[beg + 2 .. beg + 4]), 
        vendor : strings[data[beg + 4] as usize - 1].clone(),
        bios_version : strings[data[beg + 5] as usize - 1].clone(),
        bios_start_addr_seg : le_to_u16(&data[beg + 6 .. beg + 8]),
        bios_rel_date : strings[data[beg + 8] as usize - 1].clone(),
        bios_rom_sz : data[beg + 9],
        bios_char : le_to_u64(&data[beg + 0xA .. beg + 0x12]),
        bios_char_ext : 0u16,
        system_bios_maj_rel : data[beg + 0x14],
        system_bios_min_rel : data[beg + 0x15],
        emb_ctrlr_fw_maj_rel : data[beg + 0x16],
        emb_ctrlr_fw_min_rel : data[beg + 0x17],
        ext_bios_rom_sz : 0u16 // Dev box was SMBIOS 2.7 so ignoring for now
    };

    // print type 0
    tables::print(&type0);

    // TODO
    // 1. Get strings
    // 2. Get dependent fields (bios_char_ext byte count depends on table length)
    // 3. Only get fields supported by current SMBIOS version.. maybe.. Might make sense to wait for dynamic table definition and storage structure
    // 4. 
    //   a. Make to u16/u32/u64 function safer by copying to array
    //   b. Don't copy bytes if possible while keeping compile time check
    // 5. Move table defs to input file

    // for b in data.iter() {
    //     println!("{:02x}", b);
    // }

}

fn test_json() {
    let path = "./defs.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json : serde_json::Value = serde_json::from_str(&data).expect("Unable to parse JSON");

    println!("{}", json);
    
}