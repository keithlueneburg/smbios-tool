pub const TYPE_OFFSET: usize = 0;
pub const LEN_OFFSET: usize = 1;
pub const HANDLE_OFFSET: usize = 2;

// Gets the strings from a string section 
// starting at position 'start' in 'data'

// TODO - seems to break on first Type32 table - I suspect it's the first table with an empty string section
pub fn get_table_strings(data: &[u8], start: usize, next_table_start: &mut usize) -> Vec<String> {
    let mut pos = start;

    let mut buf = String::from("");
    let mut strings : Vec<String> = Vec::new();

    // If no string section, return pos + 2 to skip 
    // both terminating 0's
    if data[pos] == 0x00 && data[pos+1] == 0x00 {
        pos += 2;
        *next_table_start = pos;
        return strings;
    }

    let mut ch = data[pos];
    while ch != 0x00 {
        buf.push(ch as char);

        pos += 1;
        ch = data[pos];

        if ch == 0x00 {
            // 0 indicates end of string
            // 2nd 0 terminating table string section will get caught by loop
            strings.push(buf.clone());
            buf.clear();

            pos += 1; 
            ch = data[pos];
        }
    }

    // At end of loop, pos will be the 2nd terminating 00h byte,
    // increment to next table start
    pos += 1;
    *next_table_start = pos;
	return strings;
}

// Temp - to be replace by definition json
#[allow(dead_code)]
pub struct Type0 {
    pub table_type : u8,
    pub len : u8,
    pub handle : u16,
    pub vendor : String,
    pub bios_version : String,
    pub bios_start_addr_seg : u16,
    pub bios_rel_date : String,
    pub bios_rom_sz : u8,
    pub bios_char : u64,
    pub bios_char_ext : u16,
    pub system_bios_maj_rel : u8,
    pub system_bios_min_rel : u8,
    pub emb_ctrlr_fw_maj_rel : u8,
    pub emb_ctrlr_fw_min_rel : u8,
    pub ext_bios_rom_sz : u16
}

#[allow(dead_code)]
pub fn print(table : &Type0) {
    println!("Table: Type0");
    println!("\tType   : {}", table.table_type);
    println!("\tLen    : {}", table.len);
    println!("\tHandle : {}", table.handle);
    println!("\tVendor : {}", table.vendor);
    println!("\tBIOS Version : {}", table.bios_version);
    println!("\tBIOSStartAddrSeg : {:#04x}", table.bios_start_addr_seg);
    println!("\tBIOS Release Date : {}", table.bios_rel_date);
    println!("\tBIOSRomSz : {:#02x}", table.bios_rom_sz);
    println!("\tBIOSChar : {:#016x}", table.bios_char);
    println!("\tSysBIOSMajRel : {}", table.system_bios_maj_rel);
    println!("\tSysBIOSMinRel : {}", table.system_bios_min_rel);
    println!("\tEmbCtrlrMajRel : {}", table.emb_ctrlr_fw_maj_rel);
    println!("\tEmbCtrlrMinRel : {}", table.emb_ctrlr_fw_min_rel);
}
