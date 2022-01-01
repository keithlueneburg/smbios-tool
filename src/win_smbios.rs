pub mod win_smbios {
    use winapi::shared::minwindef::DWORD;
    use winapi::shared::minwindef::UINT;
    use winapi::shared::ntdef::NULL;
    use winapi::ctypes::c_void;
    use winapi::um::sysinfoapi::GetSystemFirmwareTable;

    // Return data format [https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemfirmwaretable]
    // struct RawSMBIOSData
    // {
    //     BYTE    Used20CallingMethod;
    //     BYTE    SMBIOSMajorVersion;
    //     BYTE    SMBIOSMinorVersion;
    //     BYTE    DmiRevision;
    //     DWORD    Length;
    //     BYTE    SMBIOSTableData[]; // <-- Start of table structures, not entry point structure
    // };
    pub fn read_smbios() -> Vec<u8> {
        let rsmb_sig : DWORD = 0x52534d42; // 'RSMB'
        let rsmb_tbl_id : DWORD = 0x00000000;
        
        let sz : UINT;
        let read_sz : UINT;
        unsafe
        {
            sz = GetSystemFirmwareTable(rsmb_sig, rsmb_tbl_id, NULL, 0);
        };

        let mut vec : Vec<u8> = Vec::with_capacity(sz as usize);
        unsafe { vec.set_len(sz as usize); }

        unsafe
        {
            read_sz = GetSystemFirmwareTable(rsmb_sig, rsmb_tbl_id, vec.as_mut_ptr() as *mut c_void, sz);
        }
        assert!(sz == read_sz);
        
        // Return buffer
        vec
    }
}
