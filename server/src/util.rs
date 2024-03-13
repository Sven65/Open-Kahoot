use rand::Rng;
use uuid_b64::UuidB64;

pub fn generate_random_number_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let number: String = (0..length)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect();
    number
}

pub fn generate_short_uuid() -> String {    
    let as_b64 = UuidB64::new();

    as_b64.to_string()
}
