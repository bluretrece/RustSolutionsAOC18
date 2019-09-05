use std::io::{self, BufReader, BufRead};
use std::collections::HashSet;
use std::fs::File;

//Receives a vector. Returns an i64, otherwise returns an error

fn to_int(input:&Vec<String>)->isize{
    let mut sum = 0;
    for freq in input.iter(){
        let num = freq.parse::<isize>().unwrap();
        sum += num;
    }
    println!("{}",sum );
    sum
    
}

fn repeated_element(input:&Vec<String>)->isize{
    let mut set = HashSet::new();
    let mut state = 0;
    for change in input.iter().cycle(){
        if !set.insert(state){
            break;
        }

        state += change.parse::<isize>().unwrap();
    }

    println!("{}", state);
    state
}



fn main(){
    //Opens the file
    let file =  File::open("input.txt").unwrap();
    //Creates the file buffer reader. (It takes the file as reference)
    let mut f = BufReader::new(&file);
    //Initialize vector of strings. It holds each line from input.
    let mut vct:Vec<String> = Vec::new();
    //Iterates through line and saves each line in vector.
    for line in f.lines(){
        vct.push(line.unwrap());
    }
    //Sends a vector reference as argument to to_int function.
    
    repeated_element(&vct);


}