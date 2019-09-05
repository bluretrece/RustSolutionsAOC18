fn do_react(f:char, n: char) -> bool {
    let mut res: bool = false;
    //if Aa AND a == a
    if f.is_uppercase() && n.is_lowercase() && f.to_lowercase().to_string() == n.to_string(){
        res = true;
        return res
    //if Aa OR aA
    } else if f.is_lowercase() && n.is_uppercase() && f.to_string() == n.to_lowercase().to_string(){
        res = true;
        return res
    } else {
        res = false;
        return false
    }
    res
}

fn react(mut puzzle: String) -> String {
    let mut tokens:Vec<char> = puzzle.chars().collect();
    let mut i = 0;
    let mut current = tokens[i];
    let mut next = tokens[i+1];
    
    for c in 1..tokens.len(){
        if do_react(current, next) {
            tokens.remove(c);
            tokens.remove(c+1);
            i += 1;
        } else {
            i += 1;
        }
    }
  
    let s: String = tokens.into_iter().collect();
    s
}
fn main(){
    let mut puzzle_input = "dabAcCaCBAcCcaDA".to_string();
    
    println!("{}",react(puzzle_input));
}