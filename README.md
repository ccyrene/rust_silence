# 🎙️ RustSilence: High-Performance Silence Detection (pydub in Rust)

RustSilence is a supercharged Rust implementation of `pydub`'s silence detection module, designed for blazing-fast audio preprocessing. Leveraging the speed and safety of Rust, RustSilence can detect silent segments in WAV, MP3, FLAC, OGG, and more — all in a fraction of the time it takes in Python.

---

## 🚀 Why?

`pydub` is great for quick scripting, but it struggles in performance-critical tasks like **real-time TTS**. This project combines:

- 🦀 **Rust’s speed**
- 🔊 **Symphonia’s audio decoding**
- 🔇 **pydub’s silence detection**
- 🐍 **pyO3 Python bindings**

to build a rock-solid preprocessing module for **F5-TTS** and beyond.

---

## ✅ Features

- 🎧 Decode audio using **Symphonia**  
- 🔇 Detect silence (via `pydub.silence`)  
- 🐍 Python-compatible via **pyO3**  

---

## 📦 Installation

```bash
$pip install rust_silence
```

---

## 🧪 Example (Python)

```python
from rust_silence import preprocess_audio

# Accepts WAV, MP3, FLAC, etc.
# Returns a NumPy array (mono, 16kHz, float32)
waveform = preprocess_audio("example.mp3")
```
Audio loading & resampling powered by **Symphonia**, silence trimming via `pydub.silence`.

---

## ⚙️ Performance Snapshot

| Task                     |    Python    |   Rust (pyO3)  |    Rust    |
|--------------------------|--------------|----------------|------------|
| Load Audio               | ~120 ms      | ~7 ms          |
| detect_silence           | ✅ (pydub)   | ✅ (pydub)    |
| detect_nonsilent         | ✅ (pydub)   | ✅ (pydub)    |
| split_on_silence         | ✅ (pydub)   | ✅ (pydub)    |
| detect_leading_silence   | ✅ (pydub)   | ✅ (pydub)    |

> **Symphonia** provides native decoding, multi-format support, and fast performance—ideal for preprocessing pipelines like F5-TTS.


## 🛣 Roadmap

- [x] Rust engine with Symphonia  
- [x] pyO3 integration  
- [x] Silence detection (from `pydub`)  

---

## 🧠 Powered by

- [Symphonia](https://github.com/pdeljanov/Symphonia) — fast, accurate audio decoding  
- [pydub](https://github.com/jiaaro/pydub) — simple silence detection  
- [pyO3](https://github.com/PyO3/pyo3) — clean Rust ↔ Python bindings  

---

## 📜 License

MIT 