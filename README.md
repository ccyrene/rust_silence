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
- 🎙️ Mono support only
- 🎧 Decode audio using **Symphonia**  (bits per sample for fmt_ext PCM sub-type must be <= 32 bits)
- 🔇 Detect silence (via `pydub.silence`)  
- 🐍 Python-compatible via **pyO3**

---

## 📦 Installation

```bash
$pip install rust-silence
```

---

## 🧪 Example (Python)

```python
import rust_silence

# Accepts WAV, MP3, FLAC, etc.
# Returns a NumPy array (mono, float32)
audio_np, sample_rate = rust_silence.from_file("example.mp3")
silence = rust_silence.detect_silence(audio_np, sample_rate)
```
Audio loading & resampling powered by **Symphonia**, silence trimming via `pydub.silence`.

---

## ⚙️ Performance Snapshot

| Task                     |    Python    |   Rust (pyO3)  |
|--------------------------|--------------|----------------|
| from_file                |    ~160 ms   |     ~4 ms      |
| detect_silence           |    ~700 ms   |     ~17 ms     |
| detect_nonsilent         |    ~700 ms   |     ~17 ms     |
| split_on_silence         |    ~700 ms   |     ~17 ms     |
| detect_leading_silence   |    ~280 μs   |     ~9 μs      |

** audio duration 10s and sample rate 32,000 (~320,000 samples) **

> **Symphonia** provides native decoding, multi-format support, and fast performance—ideal for preprocessing pipelines like F5-TTS.


## 🛣 Roadmap

- [x] Rust engine
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