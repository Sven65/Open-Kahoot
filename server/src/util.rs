use rand::Rng;

pub fn generate_random_number_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let number: String = (0..length)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect();
    number
}
