use samp::native;
use samp::prelude::*;
use rand::prelude::*;

impl super::SampRng {
    #[native(name = "RandomNumber")]
    pub fn random_number(&mut self, _amx: &Amx) -> AmxResult<i32> {
        let mut rng = thread_rng();
        let num: i32 = rng.gen();
        Ok(num)
    }

    #[native(name = "RandomNumberMax")]
    pub fn random_number_max(&mut self, _amx: &Amx, max: i32) -> AmxResult<i32> {
        let mut rng = thread_rng();
        let num: i32 = rng.gen_range(0..=max);
        Ok(num)
    }

    #[native(name = "RandomNumberMinMax")]
    pub fn random_number_min_max(&mut self, _amx: &Amx, min: i32, max: i32) -> AmxResult<i32> {
        let mut rng = thread_rng();
        let num: i32 = rng.gen_range(min..=max);
        Ok(num)
    }
}