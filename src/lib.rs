pub mod load;
pub mod preprocess_f5;
pub mod silence;

use ndarray::Array1;
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn audio_bytes_to_f32_samples_py(
    py: Python,
    audio_bytes: &[u8],
) -> PyResult<(Py<PyArray1<f32>>, usize)> {
    match load::audio_bytes_to_f32_samples(audio_bytes) {
        Ok((samples, sample_rate)) => {
            let array: Array1<f32> = Array1::from_vec(samples);
            let py_array: Py<PyArray1<f32>> = array.into_pyarray(py).into();
            Ok((py_array, sample_rate))
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            e.to_string(),
        )),
    }
}

#[pyfunction]
fn detect_silence_py<'py>(
    samples: PyReadonlyArray1<'py, f32>,
    sample_rate: usize,
    min_silence_len_ms: usize,
    silence_thresh_db: f64,
    seek_step_ms: usize,
) -> PyResult<Vec<[usize; 2]>> {
    let samples = samples.as_slice()?;

    let chunks = silence::detect_silence(
        samples,
        sample_rate,
        min_silence_len_ms,
        silence_thresh_db,
        seek_step_ms,
    );

    Ok(chunks)
}

#[pyfunction]
fn detect_nonsilent_py<'py>(
    samples: PyReadonlyArray1<'py, f32>,
    sample_rate: usize,
    min_silence_len_ms: usize,
    silence_thresh_db: f64,
    seek_step_ms: usize,
) -> PyResult<Vec<[usize; 2]>> {
    let samples = samples.as_slice()?;

    let chunks = silence::detect_nonsilent(
        samples,
        sample_rate,
        min_silence_len_ms,
        silence_thresh_db,
        seek_step_ms,
    );

    Ok(chunks)
}

#[pyfunction]
fn detect_leading_silence_py<'py>(
    samples: PyReadonlyArray1<'py, f32>,
    sample_rate: usize,
    silence_thresh_db: f64,
    chunk_size_ms: usize,
) -> PyResult<usize> {
    let samples = samples.as_slice()?;

    match silence::detect_leading_silence(samples, sample_rate, silence_thresh_db, chunk_size_ms) {
        Ok(index) => Ok(index),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}

#[pyfunction]
fn split_on_silence_py<'py>(
    py: Python,
    samples: PyReadonlyArray1<'py, f32>,
    sample_rate: usize,
    min_silence_len_ms: usize,
    silence_thresh_db: f64,
    keep_silence_ms: usize,
    seek_step_ms: usize,
) -> PyResult<Vec<Py<PyArray1<f32>>>> {
    let samples = samples.as_slice()?;

    let chunks: Vec<Vec<f32>> = silence::split_on_silence(
        samples,
        sample_rate,
        min_silence_len_ms,
        silence_thresh_db,
        keep_silence_ms,
        seek_step_ms,
    );

    let py_chunks = chunks
        .into_iter()
        .map(|chunk| {
            let array: Array1<f32> = Array1::from_vec(chunk);
            array.into_pyarray(py).into()
        })
        .collect();

    Ok(py_chunks)
}

#[pyfunction]
fn remove_silence_edges_py<'py>(
    py: Python,
    samples: PyReadonlyArray1<'py, f32>,
    sample_rate: usize,
    silence_threshold_db: f64,
    chunk_size_ms: usize,
) -> PyResult<Py<PyArray1<f32>>> {
    let samples = samples.as_slice()?;

    let trimmed = preprocess_f5::remove_silence_edges(
        samples,
        sample_rate,
        silence_threshold_db,
        chunk_size_ms,
    )
    .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let py_array: Py<PyArray1<f32>> = Array1::from_vec(trimmed).into_pyarray(py).into();

    Ok(py_array)
}

#[pyfunction]
fn preprocess_f5_py(
    py: Python,
    audio_bytes: &[u8],
    silence_threshold_db: f64,
    chunk_size_ms: usize,
    clip_short: bool,
) -> PyResult<Py<PyArray1<f32>>> {
    let audio =
        preprocess_f5::preprocess_f5(audio_bytes, chunk_size_ms, silence_threshold_db, clip_short)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let py_array: Py<PyArray1<f32>> = Array1::from_vec(audio).into_pyarray(py).into();

    Ok(py_array)
}

#[pymodule]
fn _rust_silence(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(audio_bytes_to_f32_samples_py, m)?)?;
    m.add_function(wrap_pyfunction!(detect_nonsilent_py, m)?)?;
    m.add_function(wrap_pyfunction!(detect_silence_py, m)?)?;
    m.add_function(wrap_pyfunction!(detect_leading_silence_py, m)?)?;
    m.add_function(wrap_pyfunction!(split_on_silence_py, m)?)?;
    m.add_function(wrap_pyfunction!(remove_silence_edges_py, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_f5_py, m)?)?;
    Ok(())
}
