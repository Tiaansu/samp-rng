use rand::prelude::*;
use samp::error::AmxError;
use samp::native;
use samp::prelude::*;

impl super::SampRng {
    #[native(raw, name = "RandomNumber")]
    pub fn random_number(&mut self, _amx: &Amx, mut args: samp::args::Args) -> AmxResult<u32> {
        let mut rng = thread_rng();

        if args.count() == 0 {
            // no args
            let num = rng.gen();
            return Ok(num);
        } else if args.count() == 1 {
            // max
            let max_ref = args.next::<Ref<u32>>().ok_or(AmxError::Params)?;
            let max = *max_ref;
            // we are subtracting one so that when we are
            // using it to get random item from an array,
            // we won't be getting OOB error.
            let num = rng.gen_range(0..=max - 1);
            return Ok(num);
        } else if args.count() == 2 {
            // min, max
            let min_ref = args.next::<Ref<u32>>().ok_or(AmxError::Params)?;
            let max_ref = args.next::<Ref<u32>>().ok_or(AmxError::Params)?;

            let min = *min_ref;
            let max = *max_ref;
            // same as above
            let num = rng.gen_range(min..=max - 1);
            return Ok(num);
        }

        let mut numbers: Vec<u32> = Vec::new();

        for _ in 0..args.count() {
            let num_ref = args.next::<Ref<u32>>().ok_or(AmxError::Params)?;
            let num = *num_ref;
            numbers.push(num);
        }

        Ok(numbers.choose(&mut rng).copied().unwrap())
    }
}
