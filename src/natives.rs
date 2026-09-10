use rand::prelude::*;
use samp::error::AmxError;
use samp::native;
use samp::prelude::*;

impl super::SampRng {
    #[native(name = "RandomNumberArray")]
    pub fn random_number_array(
        &mut self,
        _amx: &Amx,
        array: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        let buf = array.into_sized_buffer(size);

        let mut rng = thread_rng();

        if size == 0 {
            return Ok(0);
        } else if size == 1 {
            return Ok(buf[0]);
        } else {
            let num = buf.choose(&mut rng).copied().unwrap();
            return Ok(num);
        }
    }

    #[native(name = "RandomFloatNumberArray")]
    pub fn random_float_number_array(
        &mut self,
        _amx: &Amx,
        array: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<f32> {
        let buf = array.into_sized_buffer(size);

        let mut rng = thread_rng();

        if size == 0 {
            return Ok(0.0);
        } else if size == 1 {
            return Ok(f32::from_bits(buf[0] as u32));
        } else {
            let cell = *buf.choose(&mut rng).unwrap();
            return Ok(f32::from_bits(cell as u32));
        }
    }

    #[native(raw, name = "RandomNumber")]
    pub fn random_number(&mut self, _amx: &Amx, mut args: samp::args::Args) -> AmxResult<i32> {
        let mut rng = thread_rng();

        if args.count() == 0 {
            // no args
            let num = rng.gen();
            return Ok(num);
        } else if args.count() == 1 {
            // max
            let max_ref = args.next::<Ref<i32>>().ok_or(AmxError::Params)?;
            let max = *max_ref;

            if max == 0 {
                let num: u32 = rng.gen();
                return Ok(num as i32);
            } else if max > 0 {
                let num = rng.gen_range(0..max);
                return Ok(num);
            } else {
                // negative to 0.
                let num = rng.gen_range(max..=0);
                return Ok(num);
            }
        } else if args.count() == 2 {
            // min, max
            let min_ref = args.next::<Ref<i32>>().ok_or(AmxError::Params)?;
            let max_ref = args.next::<Ref<i32>>().ok_or(AmxError::Params)?;

            let min = *min_ref;
            let max = *max_ref;

            let num = rng.gen_range(min..max);
            return Ok(num);
        }

        let mut numbers: Vec<i32> = Vec::new();

        for _ in 0..args.count() {
            let num_ref = args.next::<Ref<i32>>().ok_or(AmxError::Params)?;
            let num = *num_ref;
            numbers.push(num);
        }

        Ok(numbers.choose(&mut rng).copied().unwrap())
    }

    #[native(raw, name = "RandomFloatNumber")]
    pub fn random_float_number(
        &mut self,
        _amx: &Amx,
        mut args: samp::args::Args,
    ) -> AmxResult<f32> {
        let mut rng = thread_rng();

        if args.count() == 0 {
            let num: f32 = rng.gen();
            return Ok(num);
        } else if args.count() == 1 {
            let max_ref = args.next::<Ref<f32>>().ok_or(AmxError::Params)?;
            let max = *max_ref;

            if max == 0.0 {
                let num: f32 = rng.gen();
                return Ok(num);
            } else if max > 0.0 {
                let num = rng.gen_range(0.0..max);
                return Ok(num);
            } else {
                // negative to 0.
                let num = rng.gen_range(max..0.0);
                return Ok(num);
            }
        } else if args.count() == 2 {
            let min_ref = args.next::<Ref<f32>>().ok_or(AmxError::Params)?;
            let max_ref = args.next::<Ref<f32>>().ok_or(AmxError::Params)?;

            let min = *min_ref;
            let max = *max_ref;

            let num = rng.gen_range(min..max);
            return Ok(num);
        }

        let mut numbers: Vec<f32> = Vec::new();

        for _ in 0..args.count() {
            let num_ref = args.next::<Ref<f32>>().ok_or(AmxError::Params)?;
            let num = *num_ref;
            numbers.push(num);
        }

        Ok(numbers.choose(&mut rng).copied().unwrap())
    }
}
