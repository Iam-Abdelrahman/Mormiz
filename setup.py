from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="mormiz",
    rust_extensions=[
        RustExtension(
            "tokenizer.arabic_tokenizer",
            binding=Binding.PyO3,
            # Between our use of editable installs and wanting to use Rust for performance sensitive
            # code, it makes sense to just always use --release
            debug=False,
        )
    ],
    packages=["mormiz"],
    zip_safe=False,
    version = '0.1',
    license='MIT',  
    # description = 'Mormiz an Arabic tokenizer for LLM',    
    # author = 'Iam-Abdelrahman',
    # author_email = 'iamabdelrahmanai@gmail.com',   
    # url = 'https://github.com/Iam-Abdelrahman/Mormiz',
    # download_url = "https://github.com/Iam-Abdelrahman/Mormiz/archive/refs/tags/0.1.tar.gz",
    # install_requires=[
    #     'setuptools_rust',
    #     'setuptools'
    # ],
#     classifiers=[
#     'Development Status :: 3 - Alpha',      
#     'Intended Audience :: Developers',      
#     'License :: OSI Approved :: MIT License',   
#     # 'Programming Language :: Python :: 3',   
#     # 'Programming Language :: Python :: 3.4',
#     # 'Programming Language :: Python :: 3.5',
#     'Programming Language :: Python :: 3.11',
#   ],
)
