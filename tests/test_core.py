import os
import math
import pytest
import rust_silence

import numpy as np
import soundfile as sf

DATA_DIR = "tests/data"

@pytest.mark.parametrize("filename", [
    "test-192khz-16bit.wav",
    "test-192khz-24bit.wav",
    "test-192khz-32bit.wav",
])
def test_192khz_audio_loading(filename):
    path = os.path.join(DATA_DIR, filename)

    samples_rust, sr_rust = rust_silence.from_file(path)
    samples_ref, sr_ref = sf.read(path, dtype='float32')
    
    # just mono supported
    if samples_ref.shape[1] == 2:
        samples_ref = samples_ref.mean(axis=1)

    assert sr_rust == 192000
    assert sr_ref == 192000
    
    assert samples_rust.dtype == np.float32
    assert samples_rust.shape == samples_ref.shape
    np.testing.assert_allclose(samples_rust, samples_ref, rtol=1e-4, atol=1e-5)

def test_ratio_to_db_amplitude():
    assert math.isclose(rust_silence.ratio_to_db(1.0, True), 0.0)
    assert math.isclose(rust_silence.ratio_to_db(10.0, True), 20.0)
    assert math.isclose(rust_silence.ratio_to_db(0.1, True), -20.0)
    assert math.isclose(rust_silence.ratio_to_db(-10.0, True), 20.0)
    assert rust_silence.ratio_to_db(0.0, True) == float("-inf")

def test_ratio_to_db_power():
    assert math.isclose(rust_silence.ratio_to_db(1.0, False), 0.0)
    assert math.isclose(rust_silence.ratio_to_db(10.0, False), 10.0)
    assert math.isclose(rust_silence.ratio_to_db(0.1, False), -10.0)
    assert math.isclose(rust_silence.ratio_to_db(-10.0, False), 10.0)
    assert rust_silence.ratio_to_db(0.0, False) == float("-inf")
    
def test_round_trip_amplitude():
    for db in [-60, -20, 0, 20, 60]:
        ratio = rust_silence.db_to_float(db, True)
        db_back = rust_silence.ratio_to_db(ratio, True)
        assert math.isclose(db, db_back, rel_tol=1e-9)

def test_round_trip_power():
    for db in [-60, -10, 0, 10, 60]:
        ratio = rust_silence.db_to_float(db, False)
        db_back = rust_silence.ratio_to_db(ratio, False)
        assert math.isclose(db, db_back, rel_tol=1e-9)

@pytest.fixture(scope="module")
def seg1():
    return rust_silence.from_file(os.path.join(DATA_DIR, 'test1.wav'))

@pytest.fixture(scope="module")
def seg4():
    return rust_silence.from_file(os.path.join(DATA_DIR, 'test4.wav'))

def test_split_on_silence_complete_silence():
    sr = 16000
    silent = np.zeros(sr * 5, dtype=np.float32)  # 5 seconds of silence
    assert rust_silence.split_on_silence(silent, sr) == []

def test_split_on_silence_test1(seg1):
    samples, sr = seg1
    chunks = rust_silence.split_on_silence(
        samples, sr,
        min_silence_len_ms=500,
        silence_thresh_db=-20.0,
    )
    assert len(chunks) == 5

def test_split_on_silence_no_silence(seg1):
    samples, sr = seg1
    chunks = rust_silence.split_on_silence(
        samples, sr,
        min_silence_len_ms=5000,
        silence_thresh_db=-200,
        keep_silence_ms=True
    )
    lengths = [len(chunk) for chunk in chunks]
    assert lengths == [len(samples)]

def test_detect_completely_silent_segment():
    sr = 16000
    silent = np.zeros(sr * 5, dtype=np.float32)
    result = rust_silence.detect_silence(
        silent, sr,
        min_silence_len_ms=1000,
        silence_thresh_db=-20,
    )
    assert result == [[0, 5000]]

def test_detect_tight_silent_segment():
    sr = 16000
    silent = np.zeros(sr * 1, dtype=np.float32)
    result = rust_silence.detect_silence(
        silent, sr,
        min_silence_len_ms=1000,
        silence_thresh_db=-20,
    )
    assert result == [[0, 1000]]

def test_detect_too_long_silence():
    sr = 16000
    silent = np.zeros(sr * 3, dtype=np.float32)
    result = rust_silence.detect_silence(
        silent, sr,
        min_silence_len_ms=5000,
        silence_thresh_db=-20,
    )
    assert result == []

def test_detect_silence_seg1(seg1):
    samples, sr = seg1
    result = rust_silence.detect_silence(
        samples, sr,
        min_silence_len_ms=500,
        silence_thresh_db=-20,
    )
    assert result == [[0, 1165], [1490, 4089], [4197, 4917], [5031, 7252], [7261, 8097], [8114, 10009]]

def test_detect_silence_seg1_with_seek_split(seg1):
    samples, sr = seg1
    result = rust_silence.detect_silence(
        samples, sr,
        min_silence_len_ms=500,
        silence_thresh_db=-20,
        seek_step_ms=10
    )
    assert result == [[0, 1160], [1490, 4080], [4200, 4910], [5040, 7250], [7270, 8090], [8120, 10000]]

def test_realistic_audio(seg4):
    samples, sr = seg4
    dBFS = 20 * np.log10(np.maximum(np.abs(samples).mean(), 1e-9))
    result = rust_silence.detect_silence(
        samples, sr,
        min_silence_len_ms=1000,
        silence_thresh_db=dBFS,
    )

    prev_end = -1
    for start, end in result:
        assert start > prev_end
        prev_end = end