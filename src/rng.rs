use rand::Rng;

// We need our own rng because thread_rng cannot be stored in a Mutex
#[derive(Default)]
pub struct DummyRng {
    seed: u32
}

impl Rng for DummyRng {
    fn next_u32(&mut self) -> u32 {
        self.seed
    }
}
