use clap::Parser;

use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i32, i32) {
    let cont = cont.trim();
    dbg!(&cont);
    let part1 = find_zeros(cont, "00000");
    let part2 = find_zeros(cont, "000000");
    (part1, part2)
}

fn find_zeros(vec: &str, target: &str) -> i32 {
    let mut j = 0;
    loop {
        if j > 10_000_000 {
            println!("Reached 10 million iterations without finding a solution.");
            break;
        }
        if j % 100_000 == 0 {
            println!("Checked {} hashes...", j);
        }
        let hash = md5_hex(&format!("{}{}", vec, j));
        if hash.starts_with(target) {
            return j;
        }
        j += 1;
    }
    0
}

fn md5_hex(input: &str) -> String {
    let digest = get_md5(input.as_bytes());

    let mut output = String::with_capacity(32);
    for byte in &digest {
        output.push_str(&format!("{:02x}", byte));
    }
    output
}

fn get_md5(input: &[u8]) -> [u8; 16] {
    // Convert input to bits
    let mut msg = input.to_vec();

    let bit_len = (input.len() as u64) * 8;

    // Append '1'
    msg.push(0x80);

    // PAd with zeros
    while msg.len() % 64 != 56 {
        msg.push(0);
    }

    msg.extend_from_slice(&bit_len.to_le_bytes());

    let s: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    //// Use sine function constants
    //let K: [u32; 64] = {
    //    let mut k = [0u32; 64];
    //    let mut i = 0;
    //    while i < 64 {
    //        k[i] = ((f64::sin((i + 1) as f64).abs() * (1u64 << 32) as f64) as u64) as u32;
    //        i += 1;
    //    }
    //    k
    //};

    let mut a0: u32 = 0x67452301; // A
    let mut b0: u32 = 0xefcdab89; // B
    let mut c0: u32 = 0x98badcfe; // C
    let mut d0: u32 = 0x10325476; // D

    for chunk in msg.chunks(64) {
        // Break chunk
        let mut m = [0u32; 16];
        for (i, word) in m.iter_mut().enumerate() {
            let j = i * 4;
            *word = u32::from_le_bytes([chunk[j], chunk[j + 1], chunk[j + 2], chunk[j + 3]]);
        }

        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        for i in 0..64 {
            let (f, g) = if i < 16 {
                ((b & c) | (!b & d), i)
            } else if i < 32 {
                ((d & b) | (!d & c), (5 * i + 1) % 16)
            } else if i < 48 {
                (b ^ c ^ d, (3 * i + 5) % 16)
            } else {
                (c ^ (b | !d), (7 * i) % 16)
            };

            let temp = d;
            d = c;
            c = b;

            // Core transformation
            b = b.wrapping_add(
                (a.wrapping_add(f).wrapping_add(K[i]).wrapping_add(m[g])).rotate_left(s[i]),
            );

            a = temp;
        }

        // Add this chunk's hash to result
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    // === Step 5: Output ===
    let mut output = [0u8; 16];
    output[0..4].copy_from_slice(&a0.to_le_bytes());
    output[4..8].copy_from_slice(&b0.to_le_bytes());
    output[8..12].copy_from_slice(&c0.to_le_bytes());
    output[12..16].copy_from_slice(&d0.to_le_bytes());

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let hash = get_md5(b"");
        for byte in &hash {
            print!("{:02x}", byte);
        }
        println!();
        assert_eq!(md5_hex(""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(md5_hex(" "), "7215ee9c7d9dc229d2921a40e899ec5f");
        assert_eq!(md5_hex("hello"), "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(md5_hex("Hello"), "8b1a9953c4611296a827abf8c47804d7");

        assert_eq!(md5_hex("a"), "0cc175b9c0f1b6a831c399e269772661");
        assert_eq!(md5_hex("ab"), "187ef4436122d1cc2f40dc2b92f0eba0");
        assert_eq!(md5_hex("b"), "92eb5ffee6ae2fec3ad71c777531578f");
        assert_eq!(md5_hex("c"), "4a8a08f09d37b73795649038408b5f33");
        assert_eq!(md5_hex("abc"), "900150983cd24fb0d6963f7d28e17f72");
        let a = "abcde";
        assert_eq!(md5_hex(&a), "ab56b4d92b40713acc5af89985d4b786");
        let fox = "The quick brown fox jumps over the lazy dog";
        assert_eq!(md5_hex(fox), "9e107d9d372bb6826bd81d3542a419d6");

        assert!(md5_hex("pqrstuv1048970").starts_with("00000"));
    }

    #[test]
    fn part1() {
        assert_eq!(find_zeros("abcdef", "00000"), 609043);
        assert_eq!(find_zeros("pqrstuv", "00000"), 1048970);
    }
}
