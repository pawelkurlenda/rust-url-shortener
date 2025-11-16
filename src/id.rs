//use rand::{RngCore, SeedableRng, rngs::StdRng};

// const ALPHABET: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

// fn base62(mut n: u64) -> String {
//     if n == 0 {
//         return "0".into();
//     }
//     let mut out = Vec::new();
//     while n > 0 {
//         out.push(ALPHABET[(n % 62) as usize]);
//         n /= 62;
//     }
//     out.reverse();
//     String::from_utf8(out).unwrap()
// }

// pub struct IdGen {
//     rng: StdRng,
// }

// impl IdGen {
//     pub fn new() -> Self {
//         Self {
//             rng: StdRng::from_os_rng(),
//         }
//     }

//     pub fn next(&mut self) -> String {
//         base62(self.rng.next_u64())
//     }
// }

pub mod shortcut_generator {

    #[inline]
    pub fn create(id: u64, length: u8) -> String {
        const SALT: u64 = 0xD6E8_FEB8_6659_FD93;
        const ALPHABET: &[u8; 62] =
            b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let target_len = length as usize;
        let mut out = String::with_capacity(target_len);
        let mut state = id ^ SALT;

        while out.len() < target_len {
            // SplitMix64 (inline)
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^= z >> 31;

            let byte = (z & 0xFF) as u8;
            if byte < 248 {
                let idx = (byte % 62) as usize;
                out.push(ALPHABET[idx] as char);
            }
        }

        out
    }
}
