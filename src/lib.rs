use samp::prelude::*;
use samp::{native, initialize_plugin};
use rand::prelude::*;

struct SampRng;

impl SampPlugin for SampRng {
    fn on_load(&mut self) {
        println!("samp rng loaded.")
    }
}

impl SampRng {
    #[native(name = "RandomNumber")]
    fn random_number(&mut self, _amx: &Amx) -> AmxResult<i32> {
        let mut rng = thread_rng();
        let num: i32 = rng.gen();
        Ok(num)
    }

    #[native(name = "RandomNumberMax")]
    fn random_number_max(&mut self, _amx: &Amx, max: i32) -> AmxResult<i32> {
        let mut rng = thread_rng();
        let num: i32 = rng.gen_range(0..=max);
        Ok(num)
    }

    #[native(name = "RandomNumberMinMax")]
    fn random_number_min_max(&mut self, _amx: &Amx, min: i32, max: i32) -> AmxResult<i32> {
        let mut rng = thread_rng();
        let num: i32 = rng.gen_range(min..=max);
        Ok(num)
    }
}

initialize_plugin!(
    natives: [
        SampRng::random_number,
        SampRng::random_number_max,
        SampRng::random_number_min_max
    ],
    {
        let plugin = SampRng;
        return plugin;
    }
);