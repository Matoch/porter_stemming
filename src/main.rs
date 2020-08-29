mod stemmer;

fn main() {
    println!("Hello, world!");
    if is_consonant('d', Some('d')) {
        println!("foo");
    }
}

fn is_consonant(current: char, previous: Option<char>) -> bool {
    match current {
        'a' | 'e' | 'i' | 'o' | 'u' => false,
        'y' => match previous {
            None => true,
            Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => false,
            _ => true
        },
        _ => true,
    }
}