from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="mormiz",
    rust_extensions=[
        RustExtension(
            "mormiz.mormiz",
            binding=Binding.PyO3,
            # Between our use of editable installs and wanting to use Rust for performance sensitive
            # code, it makes sense to just always use --release
            debug=False,
        )
    ],
    packages=["mormiz"],
    zip_safe=False,
)
