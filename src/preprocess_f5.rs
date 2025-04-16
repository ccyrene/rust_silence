use crate::load::audio_bytes_to_f32_samples;
use crate::silence::{detect_leading_silence, split_on_silence, ratio_to_db, rms, sample_to_ms};

pub fn remove_silence_edges(
    samples: &[f32],
    sample_rate: usize,
    silence_threshold_db: f64,
    chunk_size_ms: usize,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let samples_per_ms = sample_rate / 1000;
    let start_idx =
        detect_leading_silence(samples, sample_rate, silence_threshold_db, chunk_size_ms)?;

    let trimmed_start = &samples[start_idx..];
    let mut end_idx = trimmed_start.len();

    for window_start in (0..trimmed_start.len().saturating_sub(samples_per_ms))
        .rev()
        .step_by(samples_per_ms)
    {
        let window = &trimmed_start
            [window_start..window_start + samples_per_ms.min(trimmed_start.len() - window_start)];
        let dbfs = {
            let rms_val = rms(window);
            if rms_val == 0.0 {
                f64::NEG_INFINITY
            } else {
                ratio_to_db(rms_val / 1.0, true)
            }
        };

        if dbfs > silence_threshold_db {
            end_idx = window_start + samples_per_ms;
            break;
        }
    }

    let trimmed = trimmed_start[..end_idx.min(trimmed_start.len())].to_vec();

    Ok(trimmed)
}


pub fn preprocess_f5(
    audio_bytes: &[u8],
    chunk_size_ms: usize,
    silence_threshold_db: f64,
    clip_short: bool,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {

    let (mut samples, sample_rate) = audio_bytes_to_f32_samples(audio_bytes)?;

    if clip_short {

        // step 1
        let mut non_silent_segs = split_on_silence(&samples, sample_rate, 1000, -50.0, 1000, 10);
        let mut non_silent_wave:Vec<f32> = vec![];
        
        for non_silent_seg in non_silent_segs {
            let future_len = non_silent_wave.len() + non_silent_seg.len();
            if non_silent_wave.len() > 6000 && future_len > 12000 {
                break;
            }
            non_silent_wave.extend_from_slice(&non_silent_seg);
        }

        // step 2
        if non_silent_wave.len() > 12000 {
            non_silent_segs = split_on_silence(&samples, sample_rate, 100, -40.0, 1000, 10);
            non_silent_wave = vec![];
        
            for non_silent_seg in non_silent_segs {
                let future_len = non_silent_wave.len() + non_silent_seg.len();
                if non_silent_wave.len() > 6000 && future_len > 12000 {
                    break;
                }
                non_silent_wave.extend_from_slice(&non_silent_seg);
            }
        }

        samples = non_silent_wave;

        // step 3
        if samples.len() > 12000 {
            samples.truncate(12000);
        }

    }

    samples = remove_silence_edges(&samples, sample_rate, silence_threshold_db, chunk_size_ms)?;
    samples.extend(std::iter::repeat(0.0f32).take(sample_to_ms(50, sample_rate / 2)));

    Ok(samples)
}