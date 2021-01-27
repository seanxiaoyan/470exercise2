use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use rand::Rng;
use std::io::Write;
use std::collections::HashMap;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Monster {
    name: String,
    init: i32,
    armour: i32,
    attack: i32,
    challenge: i32
}

impl Monster {
    pub fn new(name: String, init: i32, armour: i32, attack: i32, challenge: i32) -> Self {
        Monster {
            name,
            init,
            armour,
            attack,
            challenge
        }
    }
}

fn read_db(filename: &str) -> Result<String,io::Error>{
    let mut file = match File::open(&filename){
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text){
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}
fn process_db(lines_vec: Vec<&str>, monster_count:usize, capability: usize){
    let mut total_rating = 0;
    let mut monsters = vec![];
    while total_rating < capability{
        let index_rand = rand::thread_rng().gen_range(0, monster_count);
        let line = lines_vec[index_rand].split(" ");
        let line_vec: Vec<&str> = line.collect();
        let rating = line_vec[4];
        total_rating += rating.parse::<usize>().unwrap();

        let mut new_monster = Monster::new(line_vec[0].to_string(), line_vec[1].parse::<i32>().unwrap(),line_vec[2].parse::<i32>().unwrap(),line_vec[3].parse::<i32>().unwrap(),line_vec[4].parse::<i32>().unwrap());
        monsters.push(new_monster);
    }
    monsters.sort_by_key(|m| m.init);
  

    let mut index_to_remove = vec![];
    let mut num_occurrence = HashMap::new();
    for i in 1..monsters.len() {
        let j = i - 1;
        if monsters[j].name == monsters[i].name {
            monsters[i].challenge += monsters[j].challenge;
            index_to_remove.push(j);

            let num = num_occurrence.entry(monsters[j].name.to_string()).or_insert(1);
            *num += 1;
        }
    }
    // println!("  monsters: {:?} ", monsters);
    // println!("  dic: {:?} ", num_occurrence);

    let mut dup = vec![];
    for m in monsters {
        if num_occurrence.contains_key(&m.name) {
            if dup.contains(&m.name) {
                continue;
            }
            else {
                let number = num_occurrence.get(&m.name).unwrap();
               
                println!("  {:?} {:?}: init:{:?} armour:{:?} attack:{:?} challenge:{:?}", number, m.name, m.init, m.armour, m.attack, m.challenge*number);
                dup.push(m.name);
            }
            
        }
        else{
            println!("  {:?}: init:{:?} armour:{:?} attack:{:?} challenge:{:?}", m.name, m.init, m.armour, m.attack, m.challenge);
        }
    }
    println!("  total challenge rating: {:?} ", total_rating);
}

fn main() {
    let file = env::args().nth(1).expect("please supply a filename");
    let db = read_db(&file).expect("bad file name");
    let lines = db.split("\n");
    let mut lines_vec: Vec<&str> = lines.collect();
  
    let monster_count = lines_vec[0].parse::<usize>().unwrap();
    println!("Read {:?} monsters", monster_count);

    lines_vec.remove(0); // remove the first line which is the count of monsters

    loop{
        print!("Enter party capability: ");
        io::stdout().flush().unwrap();
        let mut capability = String::new();
        io::stdin()
            .read_line(&mut capability)
            .expect("Failed to read line");

        if capability.trim() == "Q" {
            break;
        }
        let capability: usize = match capability.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("  type a number or type Q to quit");
                continue;
            },
        };    

        process_db(lines_vec.clone(), monster_count, capability);
    }

}
