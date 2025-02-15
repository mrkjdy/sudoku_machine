use rand::{Rng, RngCore};

pub trait SeedRng: RngCore {
    #[inline]
    fn gen_seed(&mut self) -> String {
        format!("{:016X}", self.random::<u64>())
    }
}

impl<T: RngCore> SeedRng for T {}
