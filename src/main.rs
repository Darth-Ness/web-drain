use std::io::Result;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let data = fs::read_to_string(args[1].to_owned())
        .expect("Couldn't locate file");
    println!("{:?}", find(parse(&data), &args[2]));
    Ok(())
}

fn parse(data: &str) -> Vec<String>{
    let parts :Vec<String> = data.split('}').map(str::to_string).collect();
    let mut to_return = vec![];
    let mut counter = 0;
    let vector = loop { 
        to_return.push(parts[counter].to_owned() + "}");
        counter +=1;
        if counter == parts.len() -1 {
            break to_return.to_owned();
        }
    };
    vector
}
fn find(data: Vec<String>, find: &str) -> Vec<String>{
    let mut counter = 0;
    let mut to_return = vec![];
    
    let looper = loop {
        if data[counter].contains(find) == true {
            to_return.push(data[counter].to_owned());
        }
        if data.len() -1 == counter {
            break to_return.to_owned();
        }
        counter += 1;
    };
    looper
}
