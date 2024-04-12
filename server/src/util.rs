use chrono::{Duration, Local, NaiveDateTime};
use rand::Rng;
use uuid_b64::UuidB64;
use zxcvbn::zxcvbn;

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

pub fn has_duration_passed(created_at: NaiveDateTime, duration: Duration) -> bool {
    let future_time = created_at + duration;
    let now = Local::now().naive_local();

    future_time <= now
}

pub fn check_password_strength(password: &str, inputs: Option<Vec<String>>) -> Result<zxcvbn::Entropy, zxcvbn::ZxcvbnError>  {
    let input_vec = inputs.clone().unwrap_or(vec![]);
    let input_slice: &[&str] = &input_vec.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let estimate: Result<zxcvbn::Entropy, zxcvbn::ZxcvbnError> = zxcvbn(&password, input_slice);

    return estimate
}
