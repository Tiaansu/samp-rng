mod natives;
mod plugin;

use plugin::SampRng;
use samp::initialize_plugin;

initialize_plugin!(
    natives: [
        SampRng::random_number,
        SampRng::random_number_max,
        SampRng::random_number_min_max
    ],
    {
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[SampRng] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .apply();

        SampRng {}
    }
);