use rayon::prelude::*;

pub fn ms_to_sample(ms: usize, sample_rate: usize) -> usize {
    ms * sample_rate / 1000
}

pub fn sample_to_ms(length: usize, sample_rate: usize) -> usize {
    length * 1000 / sample_rate
}

pub fn db_to_float(db: f64, using_amplitude: bool) -> f64 {
    if using_amplitude {
        10.0_f64.powf(db / 20.0)
    } else {
        10.0_f64.powf(db / 10.0)
    }
}

pub fn ratio_to_db(ratio: f64, using_amplitude: bool) -> f64 {
    if ratio == 0.0 {
        f64::NEG_INFINITY
    } else if using_amplitude {
        20.0 * ratio.abs().log10()
    } else {
        10.0 * ratio.abs().log10()
    }
}

pub fn rms(samples: &[f32]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_squares: f32 = samples.iter().map(|&s| s * s).sum();
    (sum_squares / samples.len() as f32).sqrt() as f64
}

pub fn detect_silence(
    samples: &[f32],
    sample_rate: usize,
    min_silence_len_ms: usize,
    silence_thresh_db: f64,
    seek_step_ms: usize,
) -> Vec<[usize; 2]> {
    let sample_length = samples.len();
    let seg_len_ms = sample_to_ms(sample_length, sample_rate);

    if seg_len_ms < min_silence_len_ms {
        return vec![];
    }

    let silence_thresh = db_to_float(silence_thresh_db, true);
    let min_silence_samples = ms_to_sample(min_silence_len_ms, sample_rate);
    let seek_step_samples = ms_to_sample(seek_step_ms, sample_rate);
    let last_slice_start = sample_length.saturating_sub(min_silence_samples);

    // Manually create the list of start indices with step
    let indices: Vec<usize> = (0..=last_slice_start).step_by(seek_step_samples).collect();

    // Parallel silence detection
    let silence_starts: Vec<usize> = indices
        .into_par_iter()
        .filter_map(|i| {
            let end = (i + min_silence_samples).min(sample_length);
            let slice = &samples[i..end];
            let rms_val =
                slice.iter().map(|&x| (x as f64) * (x as f64)).sum::<f64>() / slice.len() as f64;
            if rms_val.sqrt() as f32 <= silence_thresh as f32 {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    if silence_starts.is_empty() {
        return vec![];
    }

    let mut silent_ranges = vec![];
    let mut prev_i = silence_starts[0];
    let mut current_range_start = prev_i;

    for &silence_start_i in &silence_starts[1..] {
        let continuous = silence_start_i == prev_i + seek_step_samples;
        let silence_has_gap = silence_start_i > (prev_i + min_silence_samples);

        if !continuous && silence_has_gap {
            let start_ms = sample_to_ms(current_range_start, sample_rate);
            let end_ms = sample_to_ms(prev_i + min_silence_samples, sample_rate);
            silent_ranges.push([start_ms, end_ms]);
            current_range_start = silence_start_i;
        }
        prev_i = silence_start_i;
    }

    // Final range
    let start_ms = sample_to_ms(current_range_start, sample_rate);
    let end_ms = sample_to_ms(prev_i + min_silence_samples, sample_rate).min(seg_len_ms);
    silent_ranges.push([start_ms, end_ms]);

    silent_ranges
}

/// Detects non-silent segments in f32 samples by inverting the silent regions
pub fn detect_nonsilent(
    samples: &[f32],
    sample_rate: usize,
    min_silence_len_ms: usize,
    silence_thresh_db: f64,
    seek_step_ms: usize,
) -> Vec<[usize; 2]> {
    let silence = detect_silence(
        samples,
        sample_rate,
        min_silence_len_ms,
        silence_thresh_db,
        seek_step_ms,
    );

    let total_ms = sample_to_ms(samples.len(), sample_rate);

    if silence.is_empty() {
        return vec![[0, total_ms]];
    }

    if silence.len() == 1 && silence[0][0] == 0 && silence[0][1] == total_ms {
        return vec![];
    }

    let mut nonsilent = vec![];
    let mut prev_end = 0;

    for range in &silence {
        let start = range[0];
        nonsilent.push([prev_end, start]);
        prev_end = range[1];
    }

    if prev_end != total_ms {
        nonsilent.push([prev_end, total_ms]);
    }

    if nonsilent.first() == Some(&[0, 0]) {
        nonsilent.remove(0);
    }

    nonsilent
}

pub fn split_on_silence(
    samples: &[f32],
    sample_rate: usize,
    min_silence_len_ms: usize,
    silence_thresh_db: f64,
    keep_silence_ms: usize,
    seek_step_ms: usize,
) -> Vec<Vec<f32>> {
    let nonsilent_ranges = detect_nonsilent(
        samples,
        sample_rate,
        min_silence_len_ms,
        silence_thresh_db,
        seek_step_ms,
    );

    if nonsilent_ranges.is_empty() {
        return vec![];
    }

    let mut ranges_with_padding: Vec<[usize; 2]> = nonsilent_ranges
        .iter()
        .map(|[start_ms, end_ms]| {
            let start = start_ms.saturating_sub(keep_silence_ms);
            let end = end_ms + keep_silence_ms;
            [start, end]
        })
        .collect();

    // Adjust overlapping regions
    for i in 0..ranges_with_padding.len().saturating_sub(1) {
        let end = ranges_with_padding[i][1];
        let next_start = ranges_with_padding[i + 1][0];
        if next_start < end {
            let midpoint = (end + next_start) / 2;
            ranges_with_padding[i][1] = midpoint;
            ranges_with_padding[i + 1][0] = midpoint;
        }
    }

    // Extract segments
    ranges_with_padding
        .iter()
        .map(|[start_ms, end_ms]| {
            let start = ms_to_sample(*start_ms, sample_rate);
            let end = ms_to_sample(*end_ms, sample_rate).min(samples.len());
            samples[start.min(samples.len())..end].to_vec()
        })
        .collect()
}

pub fn detect_leading_silence(
    samples: &[f32],
    sample_rate: usize,
    silence_thresh_db: f64,
    chunk_size_ms: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    if chunk_size_ms == 0 {
        return Err("chunk_size must be greater than 0".into());
    }

    let chunk_size = ms_to_sample(chunk_size_ms, sample_rate);

    let mut trim_index: usize = 0;
    let mut trim_index_ms: usize = 0;

    while trim_index + chunk_size <= samples.len() {
        let chunk = &samples[trim_index..trim_index + chunk_size];
        let dbfs = {
            let rms_val = rms(chunk);
            if rms_val == 0.0 {
                f64::NEG_INFINITY
            } else {
                ratio_to_db(rms_val, true)
            }
        };

        if dbfs > silence_thresh_db {
            break;
        }

        trim_index += chunk_size;
        trim_index_ms += chunk_size_ms;
    }

    Ok(trim_index_ms)
}
