use std::collections::{HashMap, HashSet};

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input.to_string()
}

fn parse(input: String, hash: &mut HashMap<String, Vec<String>>) {
    let split = input
        .split("=").collect::<Vec<&str>>();

    let key = split[0];

    match split.len() {
        1 => {
            let vals = hash.get(key.to_string().as_str());
            let mut makeup = String::new();

            match vals {
                Some(vals) => {
                    let mut vals = vals.iter().collect::<Vec<&String>>();
                    vals.sort();

                    for val in vals {
                        makeup.push_str(val);
                    }
                    println!("{}", makeup);
                },
                None => {
                    println!("Error: item not found. Please create it before getting its value.");
                }
            }
        },
        2 => {
            let values = split[1].split("+").collect::<Vec<&str>>();
            hash.insert(key.to_string(), vec![]);

            let mut successful: bool = true;

            for value in values {
                let value = value.to_string();
                if !hash.contains_key(value.as_str()) {
                    println!("{value} is not an existing item. Please add it first.");
                    successful = false;
                } else {
                    let vals = hash.get(value.as_str()).unwrap().iter().cloned().collect::<Vec<String>>();
                    hash.get_mut(key.to_string().as_str()).unwrap().extend(vals);
                }
            }

            if !successful {
                hash.remove(key.to_string().as_str());
            }

            if key == split[1] {
                hash.insert(key.to_string(), Vec::from([key.to_string()]));
            }
        }
        _ => {
            println!("Invalid input");
        }
    }
}

fn main() {
    let hash = std::fs::read_to_string("data.json");
    let mut mega_hash_map: HashMap<String, Vec<String>>;
    match hash {
        Ok(hash) => {
            mega_hash_map = serde_json::from_str(hash.as_str()).unwrap();
        },
        _ => {
            mega_hash_map = serde_json::from_str::<HashMap<String, Vec<String>>>("{}").unwrap();

            mega_hash_map.insert(String::from("F"), vec!["F".to_string()]);
            mega_hash_map.insert(String::from("W"), vec!["W".to_string()]);
            mega_hash_map.insert(String::from("E"), vec!["E".to_string()]);
            mega_hash_map.insert(String::from("A"), vec!["A".to_string()]);

            parse(String::from("fire=F"), &mut mega_hash_map);
            parse(String::from("water=W"), &mut mega_hash_map);
            parse(String::from("earth=E"), &mut mega_hash_map);
            parse(String::from("air=A"), &mut mega_hash_map);
        }
    }

    loop {
        println!("Enter a new item in the form 'item=val+...+val_n', get an item's makeup with 'item', or type 'item=item' to create a new element. Type 'save' to save and exit.");
        let input = get_input();

        if input == "save" {
            serde_json::to_writer_pretty(std::fs::File::create("data.json").unwrap(), &mega_hash_map).unwrap();
            break;
        }

        parse(input, &mut mega_hash_map);
        println!();
    }
}
