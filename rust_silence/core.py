import numpy as np
import numpy.typing as npt

from rust_silence import _rust_silence
from typing import Tuple, List

def from_file(file: bytes) -> Tuple[npt.NDArray[np.float32], int]:
    return _rust_silence.audio_bytes_to_f32_samples_py(file)

def split_on_silence(
    samples: npt.NDArray[np.float32],
    sample_rate: int,
    min_silence_len_ms: int = 1000,
    silence_thresh_db: float = -50.0,
    keep_silence_ms: int = 1000,
    seek_step_ms: int = 100
) -> List[npt.NDArray[np.float32]]:
    
    return _rust_silence.split_on_silence_py(
        samples,
        sample_rate,
        min_silence_len_ms,
        silence_thresh_db,
        keep_silence_ms,
        seek_step_ms
    )