use winapi::shared::minwindef::UINT;
use winapi::shared::ntdef::NULL;
use winapi::shared::minwindef::DWORD;
use winapi::um::sysinfoapi::GetSystemFirmwareTable;

fn main() {
    println!("Hello, world!");

    let rsmb_sig : DWORD = 0x52534d42; // 'RSMB'
    let rsmb_tbl_id : DWORD = 0x00000000;
    
    let sz : UINT;
    let read_sz : UINT;
    unsafe
    {
        sz = GetSystemFirmwareTable(rsmb_sig, rsmb_tbl_id, NULL, 0);
    };
    println!("Need size {}", sz);

    let mut vec = Vec::with_capacity(sz as usize);
    unsafe
    {
        read_sz = GetSystemFirmwareTable(rsmb_sig, rsmb_tbl_id, vec.as_mut_ptr(), sz);
    }
    println!("Read size {}", read_sz);
    assert!(sz == read_sz);
    
}
