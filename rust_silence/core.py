import numpy as np
import numpy.typing as npt

from rust_silence import _rust_silence
from typing import Tuple, List, Optional

def from_file(file: str | bytes) -> Tuple[npt.NDArray[np.float32], int]:
    if isinstance(file, str):
        file = open(file, "rb").read()
    return _rust_silence.audio_bytes_to_f32_samples_py(file)

def db_to_float(
    db: float,
    using_amplitude: bool
) -> float:
    return _rust_silence.db_to_float_py(db, using_amplitude)

def ratio_to_db(
    ratio: float,
    using_amplitude: bool
) -> float:
    return _rust_silence.ratio_to_db_py(ratio, using_amplitude)

def detect_silence(
    samples: npt.NDArray[np.float32],
    sample_rate: int,
    min_silence_len_ms: int = 1000,
    silence_thresh_db: float = -16.0,
    seek_step_ms: int = 1
) -> List[Tuple[int, int]]:
    
    return _rust_silence.detect_silence_py(
        samples,
        sample_rate,
        min_silence_len_ms,
        silence_thresh_db,
        seek_step_ms
    )

def detect_nonsilent(
    samples: npt.NDArray[np.float32],
    sample_rate: int,
    min_silence_len_ms: int = 1000,
    silence_thresh_db: float = -16.0,
    seek_step_ms: int = 1
) -> List[Tuple[int, int]]:
    
    return _rust_silence.detect_nonsilent_py(
        samples,
        sample_rate,
        min_silence_len_ms,
        silence_thresh_db,
        seek_step_ms
    )

def split_on_silence(
    samples: npt.NDArray[np.float32],
    sample_rate: int,
    min_silence_len_ms: int = 1000,
    silence_thresh_db: float = -16.0,
    keep_silence_ms: int = 100,
    seek_step_ms: int = 1
) -> List[npt.NDArray[np.float32]]:
    
    if isinstance(keep_silence_ms, bool):
        keep_silence_ms = int(samples.shape[0] / sample_rate * 1000) if keep_silence_ms else 0
    
    return _rust_silence.split_on_silence_py(
        samples,
        sample_rate,
        min_silence_len_ms,
        silence_thresh_db,
        keep_silence_ms,
        seek_step_ms
    )
    
def detect_leading_silence(
    samples: npt.NDArray[np.float32],
    sample_rate: int,
    silence_thresh_db: float = -50.0,
    chunk_size_ms: int = 10
) -> int:
    
    return _rust_silence.detect_leading_silence_py(
        samples,
        sample_rate,
        silence_thresh_db,
        chunk_size_ms
    )
    
def remove_silence_edges(
    samples: npt.NDArray[np.float32],
    sample_rate: int,
    silence_thresh_db: float = -42.0,
    chunk_size_ms: int = 10
) -> npt.NDArray[np.float32]:
    
    return _rust_silence.remove_silence_edges_py(
        samples,
        sample_rate,
        silence_thresh_db,
        chunk_size_ms
    )
    
def preprocess_f5(
    file: str | bytes,
    clip_short: bool = True
) -> npt.NDArray[np.float32]:
    
    if isinstance(file, str):
        file = open(file, "rb").read()
    
    return _rust_silence.preprocess_f5_py(
        file,
        clip_short
    )