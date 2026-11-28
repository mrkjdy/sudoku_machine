use rand::Rng;

pub trait SeedRng: Rng {
    #[inline]
    fn gen_seed(&mut self) -> String {
        format!("{:016X}", self.next_u64())
    }
}

impl<T: Rng> SeedRng for T {}
