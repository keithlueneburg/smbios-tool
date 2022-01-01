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
