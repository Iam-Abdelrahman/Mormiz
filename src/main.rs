
use tokenizer::Data;

fn main() {
    Data::new().train("arabic_dataset.txt".to_owned(), true, "log.txt".to_owned());
    // let mut a = HashMap::new();
    // a.insert((1, 3), 4);
    // let mut l = Vec::new();
    // l.push((1, 3));
    // println!("{:?}", l.contains(a.iter().next().unwrap().0));
}
