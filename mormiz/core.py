from mormiz import load_from_file 

class Mormiz:
    def __init__(self):
        self.bpe = load_from_file("tokenizer")
    def encode(self, text):
        return self.bpe.encode(text)
    def decode(self, text):
        return self.bpe.decode(text)

