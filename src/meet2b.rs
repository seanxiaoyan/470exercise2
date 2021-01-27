use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use rand::Rng;
use std::io::Write;

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
fn process_db(mut lines_vec: Vec<&str>, mut monster_count:usize, capability: usize){
    let mut total_rating = 0;
    while total_rating < capability{
    	if lines_vec.len() == 0 {
    		println!("Trivial encounters for given capability!");
    		break;
    	} 
    	let index_rand = rand::thread_rng().gen_range(0, monster_count);
    	let line = lines_vec[index_rand].split(" ");
    	let line_vec: Vec<&str> = line.collect();
    	let rating = line_vec[4];
    	total_rating += rating.parse::<usize>().unwrap();
    	lines_vec.remove(index_rand);
    	monster_count -= 1;
    	println!("    {}: init {}, Armour {}, Attack {}, Challenge {}", line_vec[0], line_vec[1], line_vec[2], line_vec[3], line_vec[4]);
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
