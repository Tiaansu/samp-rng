use log::info;
use samp::plugin::SampPlugin;

pub struct SampRng {}

impl SampPlugin for SampRng {
    fn on_load(&mut self) {
        info!("Version: 0.0.1");
    }
}