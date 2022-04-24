use once_cell::sync::Lazy;
use rand::{rngs::OsRng, RngCore};

static ALPHABET: Lazy<Vec<u8>> = Lazy::new(|| b"123456789ABCDEFGHJKMNPQRSTUVWXYZ".to_vec());

// pub fn from_str(device_id_str: &str) -> anyhow::Result<u64> {
//     if device_id_str.len() != 8 {
//         return Err(anyhow::anyhow!("device_id is not 8 bytes"));
//     }

//     if !device_id_str.chars().all(|x| ALPHABET.contains(&(x as u8))) {
//         return Err(anyhow::anyhow!("device_id contains invalid characters"));
//     }

//     Ok(device_id_str
//         .chars()
//         .into_iter()
//         .enumerate()
//         .fold(0, |acc, (idx, ch)| {
//             if let Some(ch_idx) = ALPHABET.iter().position(|&c| c == ch as u8) {
//                 acc + ch_idx as u64 * (32u64).pow(7 - idx as u32)
//             } else {
//                 acc
//             }
//         }))
// }

// pub fn to_string(mut id: u64) -> String {
//     let mut device_id_str = String::new();

//     while id != 0 {
//         let idx = (id % 32) as usize;
//         device_id_str.push(ALPHABET[idx] as char);
//         id /= 32;
//     }

//     device_id_str.chars().rev().collect()
// }

pub fn generate() -> String {
    let mut device_id_str = String::new();
    let mut alphabet = ALPHABET.clone();

    for _ in 0..8 {
        let index = OsRng.next_u32() as usize % alphabet.len();
        device_id_str.push(alphabet[index] as char);
        alphabet.remove(index);
    }

    device_id_str
}
