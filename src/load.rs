use std::io::Cursor;
use std::sync::Arc;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};


pub fn audio_bytes_to_f32_samples(
    audio_bytes: &[u8],
) -> Result<(Vec<f32>, usize), Box<dyn std::error::Error>> {
    let owned_bytes: Arc<[u8]> = Arc::from(audio_bytes);
    let cursor = Cursor::new(owned_bytes);
    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());
    let hint = Hint::new();

    let probed = get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    let mut format = probed.format;
    let track = format.default_track().ok_or("No default track found")?;
    let mut decoder = get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

    let sample_rate = track
        .codec_params
        .sample_rate
        .ok_or("Sample rate not found")?;
    let mut samples = Vec::<f32>::new();

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(Error::ResetRequired) => continue,
            Err(_) => break,
        };

        let decoded = decoder.decode(&packet)?;
        let mut sample_buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
        sample_buf.copy_interleaved_ref(decoded);

        samples.extend_from_slice(sample_buf.samples());
    }

    Ok((samples, sample_rate as usize))
}
