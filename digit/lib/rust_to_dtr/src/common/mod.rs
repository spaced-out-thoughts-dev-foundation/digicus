use rand::{distributions::Alphanumeric, Rng};

pub fn get_random_string() -> String {
    let mut rng = rand::thread_rng();
    (0..10).map(|_| rng.sample(Alphanumeric) as char).collect()
}

pub fn join_with_newline(s1: &str, s2: &str) -> String {
    s1.to_string() + "\n" + s2
}
