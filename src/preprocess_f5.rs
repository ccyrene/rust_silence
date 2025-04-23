use crate::load::audio_bytes_to_f32_samples;
use crate::silence::{
    detect_leading_silence, ms_to_sample, ratio_to_db, rms, sample_to_ms, split_on_silence,
};

pub fn remove_silence_edges(
    samples: &[f32],
    sample_rate: usize,
    silence_threshold_db: f64,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let start_idx = detect_leading_silence(samples, sample_rate, silence_threshold_db, 10)?;
    let trimmed_start = &samples[ms_to_sample(start_idx, sample_rate)..];
    let mut end_idx = trimmed_start.len();

    // resume missing values
    if sample_rate == 22050 {
        let samples_per_ms_normal: usize = 22;
        let samples_per_ms_special: usize = 23;

        let mut samples_per_ms: usize = samples_per_ms_normal;

        // find fraction round up first position of reverse samples
        let start_special = sample_to_ms(trimmed_start.len(), sample_rate) % 20;

        for (i, window_start) in (0..trimmed_start.len().saturating_sub(samples_per_ms))
            .rev()
            .step_by(samples_per_ms)
            .enumerate()
        {
            // fraction round up to 1 every 20 step
            samples_per_ms = if i % 20 == start_special {
                samples_per_ms_special
            } else {
                samples_per_ms_normal
            };

            let window = &trimmed_start[window_start
                ..window_start + samples_per_ms.min(trimmed_start.len() - window_start)];

            let dbfs = {
                let rms_val = rms(window);
                if rms_val == 0.0 {
                    f64::NEG_INFINITY
                } else {
                    ratio_to_db(rms_val, true)
                }
            };

            if dbfs > silence_threshold_db {
                end_idx = window_start + samples_per_ms;
                break;
            }
        }

    // not have missing values
    } else {
        let samples_per_ms = sample_rate / 1000;

        for window_start in (0..trimmed_start.len().saturating_sub(samples_per_ms))
            .rev()
            .step_by(samples_per_ms)
        {
            let window = &trimmed_start[window_start
                ..window_start + samples_per_ms.min(trimmed_start.len() - window_start)];

            let dbfs = {
                let rms_val = rms(window);
                if rms_val == 0.0 {
                    f64::NEG_INFINITY
                } else {
                    ratio_to_db(rms_val, true)
                }
            };

            if dbfs > silence_threshold_db {
                end_idx = window_start + samples_per_ms;
                break;
            }
        }
    }

    let trimmed = trimmed_start[..end_idx.min(trimmed_start.len())].to_vec();

    Ok(trimmed)
}

pub fn preprocess_f5(
    audio_bytes: &[u8],
    clip_short: bool,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let (mut samples, sample_rate) = audio_bytes_to_f32_samples(audio_bytes)?;

    if clip_short {
        let lower_bound = ms_to_sample(6000, sample_rate);
        let upper_bound = ms_to_sample(12000, sample_rate);

        // step 1
        let mut non_silent_segs = split_on_silence(&samples, sample_rate, 1000, -50.0, 1000, 10);
        let mut non_silent_wave: Vec<f32> = vec![];

        for non_silent_seg in non_silent_segs {
            let future_len = non_silent_wave.len() + non_silent_seg.len();
            if non_silent_wave.len() > lower_bound && future_len > upper_bound {
                break;
            }
            non_silent_wave.extend_from_slice(&non_silent_seg);
        }

        // step 2
        if non_silent_wave.len() > upper_bound {
            non_silent_segs = split_on_silence(&samples, sample_rate, 100, -40.0, 1000, 10);
            non_silent_wave = vec![];

            for non_silent_seg in non_silent_segs {
                let future_len = non_silent_wave.len() + non_silent_seg.len();
                if non_silent_wave.len() > lower_bound && future_len > upper_bound {
                    break;
                }
                non_silent_wave.extend_from_slice(&non_silent_seg);
            }
        }

        samples = non_silent_wave;

        // step 3
        if samples.len() > upper_bound {
            samples.truncate(upper_bound);
        }
    }

    samples = remove_silence_edges(&samples, sample_rate, -42.0)?;
    samples.extend(std::iter::repeat(0.0f32).take(ms_to_sample(50, sample_rate / 2)));

    Ok(samples)
}
