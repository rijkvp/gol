use rand::Rng;

pub fn load_pattern() -> () {
    todo!()
}

pub fn gen_random(size: usize) -> Vec<bool> {
    let mut rng = rand::thread_rng();
    let mut pattern = Vec::with_capacity(size * size);
    for _ in 0..size * size {
        pattern.push(rng.gen_bool(0.3));
    }
    pattern
}
