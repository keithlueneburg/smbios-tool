// Gets the strings from a string section 
// starting at position 'start' in 'data'
pub fn get_table_strings(data: &[u8], start: usize) -> Vec<String> {
    let mut pos = start;

    let mut buf = String::from("");
    let mut strings : Vec<String> = Vec::new();

    let mut ch = data[pos] as char;
    while ch != 0x00 as char {
        buf.push(ch);

        pos += 1;
        ch = data[pos] as char;

        if ch == 0x00 as char {
            // 0 indicates end of string
            // 2nd 0 terminating table string section will get caught by loop
            strings.push(buf.clone());
            buf.clear();

            pos += 1; 
            ch = data[pos] as char;
        }
    }
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
