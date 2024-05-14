use log::info;
use samp::plugin::SampPlugin;

pub struct SampRng {}

impl SampPlugin for SampRng {
    fn on_load(&mut self) {
        info!("Version: 0.0.1");
    }

    fn on_unload(&mut self) {
        info!("Unloads the plugin successfully.");
    }
}