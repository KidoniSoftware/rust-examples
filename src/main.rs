use rust_examples::paths;
use rust_examples::trie::Trie;

fn main() {
    println!("{:?}", paths(10, 10));

    let mut trie = Trie::new();
    trie.insert("help");
    trie.insert("harp");
    trie.insert("hermit");
    trie.insert("add");
    trie.insert("additive");

    if let Some(result) = trie.find("hermit") {
        println!("found {}", result);
    }
    else {
        println!("didn't find 'hermit'")
    }
}