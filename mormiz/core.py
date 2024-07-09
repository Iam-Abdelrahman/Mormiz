from mormiz import mormiz

class Mormiz:
    def __init__(self):
        self.bpe = mormiz.load_from_file("tokenizer")
    def encode(self, text):
        return self.bpe.encode(text)
    def decode(self, text):
        return self.bpe.decode(text)

