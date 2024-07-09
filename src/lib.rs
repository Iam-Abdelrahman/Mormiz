use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{pymodule, PyResult, Python};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};
use unicode_segmentation::UnicodeSegmentation;

type LookUp = HashMap<u16, Vec<u8>>;
type Pair = (u16, u16);
type Merges = HashMap<Pair, u16>;
type PairCounter = HashMap<Pair, u32>;
type CharsLookup = HashMap<String, u16>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[pyclass]
pub struct Data {
    lookup: LookUp,
    merges: Merges,
    unicode_lookup: CharsLookup,
}

impl Data {
    pub fn new() -> Data {
        Data {
            lookup: HashMap::new(),
            merges: HashMap::new(),
            unicode_lookup: HashMap::new(),
        }
    }
    pub fn from(lookup: HashMap<u16, Vec<u8>>, merges: Merges) -> Data {
        Data {
            lookup: lookup,
            merges: merges,
            unicode_lookup: HashMap::new(),
            // bytes_lookup: HashMap::new(),
        }
    }
    fn _train(&mut self, train_file: String, print_logs: bool, logs_file: String) {
        self.unicode_lookup.insert("oov".to_owned(), 0);
        self.lookup.insert(0, [].to_vec());
        let mut log_file: File = File::create(logs_file.clone()).expect("To Work");
        if print_logs {
            log_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(logs_file)
                .expect("Logs File throws an error To Work");
            log_file
                .write("Started \n ".as_bytes())
                .expect("Logs File throws an error To Work");
            log_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("log.txt")
                .expect("To Work");
        }
        // for i in 0..256 {
        //     self.lookup.entry(i).or_insert([i as u8].to_vec());
        // }
        let mut file = File::open(train_file).expect("Train File Not Existed");
        let mut buffer: [u8; 3067833] = [0; 1073741824 / 350];
        let mut time = 0;
        println!("Just Started");
        loop {
            time += 1;
            let count = file.read(&mut buffer).expect("To Work");
            if count == 0 {
                break;
            }
            let st = String::from_utf8_lossy(&buffer);
            let st = st.replace("<|im_start|>", "^");
            let st = st.replace("<|im_end|>", "^");
            let mut compress_buffer: Vec<u16> = Vec::new();
            for grapheme in st.graphemes(true) {
                let is_available = self.unicode_lookup.contains_key(&grapheme.to_owned());
                if !is_available {
                    let token = self.lookup.len().clone() as u16;
                    self.unicode_lookup.insert(grapheme.to_owned(), token);
                    self.lookup.insert(token, grapheme.as_bytes().to_vec());
                }
                let v = self.unicode_lookup.get(grapheme).unwrap();
                compress_buffer.push(*v);
            }
            let mut new_bytes: Vec<u16>;
            if time == 1 {
                new_bytes = compress_buffer;
            } else {
                new_bytes = encode_bytes(&compress_buffer.clone(), &self.merges);
            }
            println!("Finish converting new read chunk to tokens");
            for c in 0..700 {
                let counter = count_occur(&new_bytes);
                let (max, how_much) = find_max(&counter);
                if how_much < 10 {
                    continue;
                };
                if let Some(c) = self.merges.get(&max) {
                    println!("No News Loop count is {:?}", c);
                    new_bytes = replace_occur(max, &new_bytes, *c);
                } else {
                    let new_token = self.lookup.keys().len() as u16;
                    self.merges.insert(max, new_token);
                    let mut new_value = Vec::new();

                    match self.lookup.get(&max.0) {
                        Some(v) => new_value.extend(v),
                        None => panic!("This should not happen"),
                    }
                    match self.lookup.get(&max.1) {
                        Some(v) => new_value.extend(v),
                        None => panic!("This should not happen"),
                    }
                    self.lookup.insert(new_token, new_value);

                    if print_logs {
                        log_file
                            .write_all(
                                format!(
                                    "Token {:?} | Merge {:?} | {:?} Times | Loop {:?}\n",
                                    new_token, max, how_much, c
                                )
                                .as_bytes(),
                            )
                            .expect("To Work");
                    }
                    new_bytes = replace_occur(max, &new_bytes, new_token);
                    println!(
                        "Token {:?} | Merge {:?} | {:?} Times | Loop {:?}",
                        new_token, max, how_much, c
                    );
                }
            }
            save_to_file(&self, "tokenizer");
            log_file
                .write_all("\n-----------------------------------------------------------------------------------\n" .as_bytes())   
                .expect("To Work")
        }
    }

    fn _encode(&self, st: String) -> Vec<u16> {
        let mut bytes: Vec<u16> = Vec::new();
        for grapheme in st.graphemes(true) {
            let is_available = self.unicode_lookup.contains_key(&grapheme.to_owned());
            if !is_available {
                bytes.push(*self.unicode_lookup.get("oov").unwrap());
            } else {
                let v = self.unicode_lookup.get(grapheme).unwrap();
                bytes.push(*v);
            }
        }
        let mut counter = count_occur(&bytes);
        let mut max = find_max(&counter).0;
        while let Some(merge) = self.merges.get(&max) {
            bytes = replace_occur(max, &bytes, *merge);
            counter = count_occur(&bytes);
            max = find_max(&counter).0;
        }
        return bytes;
    }

    fn _decode(&self, tokens: Vec<u16>) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        for token in tokens {
            match self.lookup.get(&token) {
                Some(v) => bytes.extend(v),
                None => panic!("This should not happend"),
            }
        }
        return String::from_utf8_lossy(&bytes).to_string();
    }
}

#[pymethods]
impl Data {
    pub fn train(&mut self, train_file: String, print_logs: bool, logs_file: String) {
        self._train(train_file, print_logs, logs_file)
    }

    pub fn encode(&self, st: String) -> Vec<u16> {
        return self._encode(st);
    }

    pub fn decode(&self, tokens: Vec<u16>) -> String {
        return self._decode(tokens);
    }

    pub fn vocab(&self) -> Vec<u16> {
        let v: Vec<u16> = self.lookup.keys().cloned().collect();
        return v;
    }
}

fn encode_bytes(bytes: &[u16], merges: &Merges) -> Vec<u16> {
    let mut bytes: Vec<u16> = bytes.to_vec();
    let mut ignores = Vec::new();
    let mut failed_count = 0;
    while failed_count < 7 {
        let counter = count_occur(&bytes);
        let max = find_max_and_ignore(&counter, &ignores).0;
        let v = merges.get(&max);
        if v.is_none() {
            ignores.push(max);
            failed_count += 1;
            println!("Failed to find a match between the max occur with already learned | Failed Count {failed_count}");
            continue;
        }
        failed_count = 0;
        let merge = v.unwrap();
        bytes = replace_occur(max, &bytes, *merge);
        println!(
            "Found a match between the max occur with already learned | Pair is {:?}",
            max
        );
    }
    return bytes;
}

fn count_occur(bytes: &[u16]) -> PairCounter {
    let mut counter: PairCounter = HashMap::new();
    for (i, b) in bytes.iter().enumerate() {
        if i == bytes.len() - 1 {
            continue;
        }
        let pair = (*b as u16, bytes[i + 1] as u16);
        *(counter.entry(pair).or_insert(0)) += 1;
    }
    return counter;
}

fn find_max(counter: &PairCounter) -> (Pair, u32) {
    if counter.len() == 0 {
        panic!("Counter should not be zero");
    }

    let first = counter.iter().next().expect("msg");
    let mut max_pair = first.0;
    let mut max_value = first.1;
    for (pair, count) in counter {
        if count > max_value {
            max_pair = pair;
            max_value = count;
        }
    }

    return (*max_pair, *max_value);
}

fn find_max_and_ignore(counter: &PairCounter, ignore: &Vec<Pair>) -> (Pair, u32) {
    if counter.len() == 0 {
        panic!("Counter should not be zero");
    }

    let first = counter
        .iter()
        .skip_while(|v| ignore.contains(&(v.0)))
        .next()
        .expect("msg");
    let mut max_pair = first.0;
    let mut max_value = first.1;
    for (pair, count) in counter {
        if ignore.contains(&pair) {
            continue;
        }
        if count > max_value {
            max_pair = pair;
            max_value = count;
        }
    }

    return (*max_pair, *max_value);
}

fn replace_occur(occur: Pair, bytes: &[u16], with: u16) -> Vec<u16> {
    let mut new_bytes: Vec<u16> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if i < bytes.len() - 1 && bytes[i] as u16 == occur.0 && bytes[i + 1] as u16 == occur.1 {
            new_bytes.push(with);
            i += 2;
        } else {
            new_bytes.push(bytes[i] as u16);
            i += 1
        }
    }
    return new_bytes;
}

fn save_to_file(data: &Data, filename: &str) {
    let encoded: Vec<u8> = bincode::serialize(data).expect("Should work");
    let mut file = File::create(filename).expect("Should work");
    file.write_all(&encoded).expect("Should work");
    println!("--------------------------------------------- \n Saved Successfully \n---------------------------------------------");
}

#[pyfunction]
pub fn load_from_file(filename: &str) -> Data {
    let mut file = File::open(filename).expect("Should work");
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded).expect("Should work");
    let data: Data = bincode::deserialize(&encoded).expect("Should work");
    return data;
}

#[cfg(test)]
mod tests {

    use crate::load_from_file;

    // #[test]
    // fn test_encode() {
    //     let data = load_from_file("tokenizer");
    //     let mut buf = [0; 1073741824 / 150];
    //     File::open("arabic_dataset.txt")
    //         .expect("Shoud lbe there")
    //         .read(&mut buf)
    //         .expect("msg");
    //     let tokens = data._encode(String::from_utf8_lossy(&buf).to_string());
    //     let res = data._decode(tokens);
    //     println!("Decoded Result = {:?}", res);
    // }

    fn test_hi() {
        println!("Hi");
    }

    #[test]
    fn test_encode() {
        let str = "Ok";
        let data = load_from_file("tokenizer");
        data._decode(data._encode(str.to_owned()));
    }
}

#[pymodule]
fn _mormiz(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Data>()?;
    m.add_function(wrap_pyfunction!(load_from_file, m)?)?;
    Ok(())
}
