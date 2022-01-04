use rand::Rng;

pub fn randomVM() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
    let SEED_LEN: usize = 8;
    let mut rng = rand::thread_rng();
    let vm: String = (0..SEED_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    vm
}
