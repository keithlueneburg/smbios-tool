

fn main() {
    println!("Hello, world!");

    let data = smbios_windows::read_smbios();

    println!("Data size: {}", data.len());

    for b in data.iter() {
        println!("{}", b);
    }
}

mod smbios_windows {
    use winapi::shared::minwindef::DWORD;
    use winapi::shared::minwindef::UINT;
    use winapi::shared::ntdef::NULL;
    use winapi::ctypes::c_void;
    use winapi::um::sysinfoapi::GetSystemFirmwareTable;

pub fn read_smbios() -> Vec<u8> {
        let rsmb_sig : DWORD = 0x52534d42; // 'RSMB'
        let rsmb_tbl_id : DWORD = 0x00000000;
        
        let sz : UINT;
        let read_sz : UINT;
        unsafe
        {
            sz = GetSystemFirmwareTable(rsmb_sig, rsmb_tbl_id, NULL, 0);
        };
        println!("Need size {}", sz);

        let mut vec : Vec<u8> = Vec::with_capacity(sz as usize);
        unsafe { vec.set_len(sz as usize); }

        unsafe
        {
            read_sz = GetSystemFirmwareTable(rsmb_sig, rsmb_tbl_id, vec.as_mut_ptr() as *mut c_void, sz);
        }
        println!("Read size {}", read_sz);
        assert!(sz == read_sz);
        
        // Return buffer
        vec
    }
}
