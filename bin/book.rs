use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytes = fs::read(args.get(1).unwrap()).unwrap();
    let struct_size = 8 + 2 + 2 + 4;
    println!("{:?}", bytes.len());

    let mut book: Vec<(u64, u16, u16, u32)> = vec![];
    for i in 0..(bytes.len() / struct_size) {
        let key: u64 = ((bytes[i] as u64) << (7 * 8))
            + ((bytes[i + 1] as u64) << (6 * 8))
            + ((bytes[i + 2] as u64) << (5 * 8))
            + ((bytes[i + 3] as u64) << (4 * 8))
            + ((bytes[i + 4] as u64) << (3 * 8))
            + ((bytes[i + 5] as u64) << (2 * 8))
            + ((bytes[i + 6] as u64) << (1 * 8))
            + bytes[i + 7] as u64;

        let mov: u16 = ((bytes[i + 8] as u16) << 8) + ((bytes[i + 9] as u16) << 0);
        let weight: u16 = ((bytes[i + 10] as u16) << 8) + bytes[i + 11] as u16;
        let learn: u32 = ((bytes[i + 12] as u32) << 24)
            + ((bytes[i + 13] as u32) << 16)
            + ((bytes[i + 14] as u32) << 8)
            + (bytes[i + 15] as u32);

        book.push((key, mov, weight, learn));
    }

    println!(
        "{:?}",
        book[1..10]
            .into_iter()
            .collect::<Vec<&(u64, u16, u16, u32)>>()
    );
}
