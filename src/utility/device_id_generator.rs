use rand::{rngs::OsRng, Rng};

pub fn generate() -> (u8, u32) {
    let bucket_id: u8 = OsRng.gen_range(1..=99);
    let id: u32 = OsRng.gen_range(12340000..=99990000);

    (bucket_id, id)
}
