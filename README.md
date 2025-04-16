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
import rust_silence

# Accepts WAV, MP3, FLAC, etc.
# Returns a NumPy array (mono, 16kHz, float32)
audio_np, sample_rate = rust_silence.from_file("example.mp3")
silence = rust_silence.detect_silence(audio_np, sample_rate)
```
Audio loading & resampling powered by **Symphonia**, silence trimming via `pydub.silence`.

---

## ⚙️ Performance Snapshot

| Task                     |    Python    |   Rust (pyO3)  |
|--------------------------|--------------|----------------|
| from_file                |    ~120 ms   |     ~0.5 ms    |
| detect_silence           |    ~230 ms   |     ~80 ms     |
| detect_nonsilent         |    ~230 ms   |     ~80 ms     |
| split_on_silence         |    ~230 ms   |     ~80 ms     |
| detect_leading_silence   |    ~70 μs    |     ~1 μs      |

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