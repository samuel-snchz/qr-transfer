use qr_transfer::protocol::crc32::crc32;

fn main() {
    println!("{}", crc32(b"hello"));
}
