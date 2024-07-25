# mormiz

**mormiz** is a fast tokenizer written in Rust, based on the Arabic name مرمِّز. This tokenizer is specifically trained on Arabic text, providing efficient and accurate tokenization for natural language processing tasks in Arabic.

## Features

- **Fast**: Written in Rust for high performance.
- **Simple API**: Easy to use with a straightforward Python API.
- **Pre-trained Weights**: Includes pre-trained weights for immediate use.

## Installation

You can install **mormiz** using pip:

```sh
pip install mormiz
```

To use mormiz, you also need to have the tokenizer weights file in the same directory where the API is called. Follow the instructions below to get started.



## Usage

### Python API

First, ensure you have the `tokenizer` weights file in the same directory as your script (located in the source code root directory). Then, you can use the following Python API:

```python
from mormiz import Mormiz

# Initialize the tokenizer
tokenizer = Mormiz()

# Example text
text = "مرحباً بالعالم!"

# Encode the text
encoded_text = tokenizer.encode(text)
print("Encoded:", encoded_text)

# Decode the text
decoded_text = tokenizer.decode(encoded_text)
print("Decoded:", decoded_text)
```

## TODO

- Automate the download of the `tokenizer` weights file instead of requiring manual placement.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue on GitHub.

## License

This project is licensed under the MIT License.
