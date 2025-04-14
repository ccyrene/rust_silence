from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="rust_silence",
    rust_extensions=[
        RustExtension(
            "rust_silence._rust_silence",
            binding=Binding.PyO3,
            debug=False,
        )
    ],
    packages=["rust_silence"],
    zip_safe=False,
)