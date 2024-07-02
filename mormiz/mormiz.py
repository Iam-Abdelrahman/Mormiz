from arabic_tokenizer import load_from_file 
text = "OK"
class Mormiz:
    def __init__(self):
        self.bpe = load_from_file("tokenizer")
    def encode(self):
        return self.bpe.encode(text)
    def decode(self):
        return self.bpe.decode(self.encode())

# 40,960 => Has 40853

m = Mormiz()
print( len(m.encode()), len((list(text.encode('utf-8')))), len(text), len(m.decode()))
# print(m.decode() == 'واهتمت الامارات بذلك فكان حقا ' )