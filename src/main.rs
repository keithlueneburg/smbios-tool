use std::fs;

use crate::win_smbios::win_smbios::read_smbios;
use crate::util::util::le_to_u16;
use crate::util::util::le_to_u64;

use serde_json::json;

mod win_smbios;
mod util;
mod tables;

#[allow(unused_variables)] //temp
fn main() {
    let defs = read_defs_json(String::from("./defs.json"));
    let data = read_smbios();
    let smbios_table: serde_json::Value = parse_raw_smbios_data(data.as_slice(), defs);

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

// fn test_json() {
//     let path = "./defs.json";
//     let data = fs::read_to_string(path).expect("Unable to read file");
//     let json : serde_json::Value = serde_json::from_str(&data).expect("Unable to parse JSON");

//     println!("{}", json);
//     println!("defs.json:\n==========\n\tSMBIOS version {}.{}", json["SMBIOS major version"], json["SMBIOS minor version"]);
//     let arr = json["Tables"].as_array();
//     for table_def in  arr.unwrap() {
//         println!("\tType: {}", table_def["Type"]);
//     }
// }

fn read_defs_json(path: String) -> serde_json::Value {
    let data = fs::read_to_string(path).expect("Unable to read file");
    return serde_json::from_str(&data).expect("Unable to parse JSON");
}

fn parse_raw_smbios_data(data: &[u8], defs: serde_json::Value) -> serde_json::Value {
    let table_defs = defs["Tables"].as_object().unwrap();
    println!("DBG: # of table types defined: {}", table_defs.len());

    let maj = data[1];
    let min = data[2];

    println!("DBG: Actual SMBIOS Version: {}.{}", maj, min);

    let mut table_start: usize = 8;
    let data_size = data.len();

    // Keep reading tables until
    while table_start < data_size - 1 {
        // Get common header fields
        let table_type = data[table_start + tables::TYPE_OFFSET];
        let _table_sz = data[table_start + tables::LEN_OFFSET];
        let _handle = le_to_u16(&data[table_start + tables::HANDLE_OFFSET .. table_start + tables::HANDLE_OFFSET + 2]);

        // Find definition for type
        let def = &table_defs[&table_type.to_string()];

        // create new Map<String, Value>> for table data
        // (ideally create within top level container to avoid a copy)
        let mut table = serde_json::Map::<String, serde_json::Value>::new();

        // Add each field (as applicable by version) to table Value
        for key in def["Fields"].as_array().unwrap() {
            // TODO - get bytes based on field Type
            let field_data = data[table_start + key["Offset"].as_u64().unwrap() as usize];

            table.insert(
                String::from(key["Name"].as_str().unwrap()), 
                json!(field_data));
        }

        for (k, v) in table {
            println!("{}: {}", k, v);
        }

        table_start = 1000000;
    }


    // Table begin
    // 1st byte of table data in structure returned by windows API
    let beg = 8 as usize; 
    let table_len = data[beg + 1];

    // Get strings for this Type 0 table
    // TODO - get_table_strings needs to return the index of next structure start
    let strings = tables::get_table_strings(data, beg + table_len as usize);

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
    //tables::print(&type0);

    //stub
    return serde_json::from_str("{}").unwrap();
}