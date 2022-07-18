use std::io::{self, Write};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
struct Node {
    is_complete: bool,
    children: HashMap<char, Node>
}

impl Node {

    pub fn new() -> Self {
        Node {
            is_complete: false,
            children: HashMap::new()
        }
    }

    pub fn insert(&mut self, mut chars: Peekable<Chars>) -> &mut Node {
        if let Some(ch) = chars.next() {
            if self.children.contains_key(&ch) {
                let node = self.children.get_mut(&ch).unwrap();
                if chars.peek().is_some() {
                    node.insert(chars);
                } else {
                    node.is_complete = true;
                }
            } else {
                let mut node = Node::new();
                if chars.peek().is_some() {
                    node.insert(chars);
                } else {
                    node.is_complete = true;
                }
                self.children.insert(ch, node);
            }
        } 
        return self
    }

    fn is_leaf(&self) -> bool {
        return self.children.is_empty()
    }

    fn is_word(&self) -> bool {
        return self.is_complete
    }

    fn get_start(&self, mut chars: Peekable<Chars>) -> Option<(String, &Node)> {
        if let Some(ch) = chars.next() {
            if let Some(node) = self.children.get(&ch) {
                if let Some((string_so_far, node2)) = node.get_start(chars) {
                    let mut string = String::from(ch);
                    string.push_str(&string_so_far);
                    return Some((string, node2));
                }
            }
            None
        } else {
            return Some(("".to_string(), &self)); 
        }   
    }

    fn produce_words(&self, prefix: String, words: &mut Vec<String>) -> () {
        for (ch, node) in &self.children {
            let mut word = prefix.clone();
            word.push(ch.clone());
            if node.is_leaf() {
                words.push(word);
            } else if node.is_word() {
                let new_prefix = word.clone().to_string();
                words.push(word);
                node.produce_words(new_prefix, words);
            } else {
                node.produce_words(word, words);
            }
        }
    }

    pub fn find(&self, chars: Peekable<Chars>) -> Vec<String> {
        let mut words:Vec<String> = vec![];
        if let Some((prefix, start_node)) = self.get_start(chars) {
            start_node.produce_words(prefix, &mut words);
        }
        words
    }

}

fn main() {
    let mut database = Node::new();
    
    loop {
        print!("autocomplete> ");
        io::stdout().flush().unwrap();

        // let mut line = String::new();
        // if let Ok(_) = io::stdin().read_line(&mut line) {
        //     line = line.trim().to_string();

        //     if line.starts_with(":find") {
        //         let mut spliterator = line.split_whitespace();
        //         spliterator.next();
        //         if let Some(word) = spliterator.next() {
        //             let result = root.find(word.chars().peekable());
        //             println!("Found: {:?}", result);
        //         }
        //     } else if line.starts_with(":") {
        //         let res = root.get_start(String::from(&line[1..]).chars().peekable());
        //         print!("{:#?}", res);
        //     } else if !line.is_empty() {
        //         root.insert(line.chars().peekable());
        //         print!("{:#?}", root);
        //     }
        // } else {
        //     println!("Incorrect input");
        // }

        let mut buffer = String::new();

        if let Ok(_) = io::stdin().read_line(&mut buffer) {
            buffer = buffer.trim().to_string();

            let mut args = buffer.split_whitespace();

            let command = args.next().expect("Command is missing");
            match command {
                ":insert" => {
                    match args.next() {
                        Some(word) => {
                            database.insert(word.chars().peekable());
                            ()
                        },
                        None => println!("Error: ':insert' must be followed by a word")
                    }
                },
                ":complete" => {
                    match args.next() {
                        Some(prefix) => {
                            let result = database.find(prefix.chars().peekable());
                            println!("Found: {:?}", result);
                        },
                        None => println!("Error: ':complete' must be followed by a prefix")
                    }
                },
                ":exit" => return,
                _ => println!("Error: Unsupported command: {:?}, try using (:insert, :complete)", command)
            }
        } else {
            println!("Error: Invalid input: '{:?}'", buffer);
        }

    }
}