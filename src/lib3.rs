use std::collections::HashMap;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{pymodule, PyResult, Python};
use serde::{Deserialize, Serialize};

type LookUp = HashMap<u16, Vec<u8>>;
type Pair = (u16, u16);
type Merges = HashMap<Pair, u16>;
type PairCounter = HashMap<Pair, u32>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[pyclass]
pub struct Data {
    lookup: LookUp,
    merges: Merges,
}

impl Data {
    pub fn new() -> Data {
        Data {
            lookup: HashMap::new(),
            merges: HashMap::new(),
        }
    }
    pub fn from(lookup: HashMap<u16, Vec<u8>>, merges: Merges) -> Data {
        Data {
            lookup: lookup,
            merges: merges,
        }
    }
    fn _train(&mut self, train_file: String, print_logs: bool, logs_file: String) {
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
        for i in 0..256 {
            self.lookup.entry(i).or_insert([i as u8].to_vec());
        }
        let mut file = File::open(train_file).expect("Train File Not Existed");
        println!("Yes We are here");
        let mut buffer = [0; 1073741824 / 350];
        let mut time = 0;
        loop {
            time += 1;
            let count = file.read(&mut buffer).expect("To Work");
            if count == 0 {
                break;
            }
            println!("Round for reading training file is -> {:?}", time);

            let mut new_bytes: Vec<u16>;
            if time == 1 {
                new_bytes = buffer.to_vec().iter().map(|&v| v as u16).collect();
            } else {
                println!("Yes Encoding it");
                new_bytes = encode_bytes(&buffer.clone(), &self.merges);
            }
            for c in 0..700 {
                println!("Counting occur {:?}", c);

                let counter = count_occur(&new_bytes);
                let (max, how_much) = find_max(&counter);
                if how_much < 10 { continue };
                if let Some(c) = self.merges.get(&max) {
                    new_bytes = replace_occur(max, &new_bytes, *c);
                } else {
                    let new_token = self.lookup.keys().len() as u16;
                    self.merges.insert(max, new_token);
                    let mut new_value = Vec::new();
                    if max.0 <= 255 {
                        new_value.push(max.0 as u8);
                    } else {
                        match self.lookup.get(&max.0) {
                            Some(v) => new_value.extend(v),
                            None => panic!("This should not happen"),
                        }
                    }
                    if max.1 <= 255 {
                        new_value.push(max.1 as u8);
                    } else {
                        match self.lookup.get(&max.1) {
                            Some(v) => new_value.extend(v),
                            None => panic!("This should not happen"),
                        }
                    }
                    self.lookup.insert(new_token, new_value);
                    println!(
                        "New Merge {:?} with token {:?}  which occurs {:?} times \n ",
                        max, new_token, how_much
                    );
                    if print_logs {
                        log_file
                            .write_all(
                                format!(
                                    "New Merge {:?} with token {:?}  which occurs {:?} times \n ",
                                    max, new_token, how_much
                                )
                                .as_bytes(),
                            )
                            .expect("To Work");
                    }
                    new_bytes = replace_occur(max, &new_bytes, new_token);
                }
            }
            save_to_file(&self, "tokenizer");
           
        }
        // encode(String::from_utf8_lossy(&buffer).to_string(), lookup, merges)
    }

    fn _encode(&self, st: String) -> Vec<u16> {
        let mut bytes: Vec<u16> = st.as_bytes().to_vec().iter().map(|&v| v as u16).collect();
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
        let v :Vec<u16> =  self.lookup.keys().cloned().collect();
        return  v;
    }
}

fn encode_bytes(bytes: &[u8], merges: &Merges) -> Vec<u16> {
    println!("Start Encoding");
    let mut bytes: Vec<u16> = bytes.iter().map(|&v| v as u16).collect();
    println!("Converting");
    println!("Counter Done");
    let mut ignores = Vec::new();
    let mut failed_count = 0;
    while failed_count < 7 {
        let counter = count_occur(&bytes);
        let max = find_max_and_ignore(&counter, &ignores).0;
        let v = merges.get(&max);
        if v.is_none() {
            println!("Failed Inside the loop");
            ignores.push(max);
            failed_count += 1;
            continue;
        }
        failed_count = 0;
        let merge = v.unwrap();
        println!("Inside the loop");
        println!("Max is {:?}", max);
        bytes = replace_occur(max, &bytes, *merge);
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
    println!("--------------------------------- \n Saved Successfully \n---------------------------------------------");
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
    use std::{fs::File, io::Read};

    use crate::load_from_file;

    #[test]
    fn test_encode() {
        let data = load_from_file("tokenizer");
        let mut buf = [0; 1073741824 / 150];
        File::open("arabic_dataset.txt")
            .expect("Shoud lbe there")
            .read(&mut buf)
            .expect("msg");
        // println!("Bytes After Merging {:?} ", &buf[..100]);
        let tokens = data._encode(String::from_utf8_lossy(&buf).to_string());
        let res = data._decode(tokens);
        println!(
            "Decode Result {:?}",
            String::from_utf8_lossy(&buf).to_string() == res
        );
    }
}

#[pymodule]
fn arabic_tokenizer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Data>()?;
    m.add_function(wrap_pyfunction!(load_from_file, m)?)?;
    Ok(())
}
