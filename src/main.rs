use crate::win_smbios::win_smbios::read_smbios;

mod win_smbios;
//pub use win_smbios;

fn main() {
    println!("Hello, world!");

    let data = read_smbios();

    println!("Data size: {}", data.len());

    for b in data.iter() {
        println!("{:02x}", b);
    }
}
