use rand::Rng;

pub fn rand_between(low: usize, high: usize) -> usize {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(low, high);
    value
}