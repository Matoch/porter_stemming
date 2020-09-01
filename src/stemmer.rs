// A \consonant\ in a word is a letter other than A, E, I, O or U, and other
// than Y preceded by a consonant. (The fact that the term `consonant' is
// defined to some extent in terms of itself does not make it ambiguous.) So in
// TOY the consonants are T and Y, and in SYZYGY they are S, Z and G. If a
// letter is not a consonant it is a \vowel\.

fn is_consonant(current: Option<char>, previous: Option<char>) -> bool {
    match current {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => false,
        Some('y') => match previous {
            Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => true,
            _ => false
        },
        _ => true,
    }
}

fn has_vowel(word: &String) -> bool {
    let my_chars = word.chars();
    let mut previous: Option<char>;
    let mut current: Option<char> = None;
    for my_char in my_chars {
        previous = current;
        current = Some(my_char);
        if !is_consonant(current, previous) {
            return true;
        };
    }
    return false;
}

fn measure(word: &String) -> usize {
    let measure = measure_with_limit(&word, word.len());
    return measure;
}

fn measure_with_limit(word: &String, max: usize) -> usize {
    let my_chars = word.chars();
    let mut previous: Option<char>;
    let mut current: Option<char> = None;
    let mut count = 0;
    let mut current_consonant = false;
    let mut begin_counting = false;
    let mut position: usize = 0;

    for my_char in my_chars {
        if position == max {
            return count;
        }
        previous = current;
        current = Some(my_char);
        if !begin_counting {
            if !is_consonant(current, previous) {
                begin_counting = true;
            }
        }
        else if current_consonant != is_consonant(current, previous) {
            current_consonant = !current_consonant;
            // Only increase count when we go from vowel to consonant
            if current_consonant {
                count += 1;
            }
        }
        position += 1;
    }
    return count;
}


fn stem(word: String) -> String {
    if word.len() > 2 {
        let mut my_word = word.to_lowercase().trim().to_string();
        my_word = stem1a(my_word);
        my_word = stem1b(my_word);
        my_word = stem1c(my_word);
        my_word = stem2(my_word);
        my_word = stem3(my_word);
        my_word = stem4(my_word);
        my_word = stem5a(my_word);
        my_word = stem5b(my_word);    
        return my_word;
    }
    return word;
}

fn stem1a(mut word: String) -> String {
    if word.ends_with("sses") {
        word.truncate(word.len()-2);
        return word;
    }
    else if word.ends_with("ies") {
        word.truncate(word.len()-2);
        return word;
    }
    else  if word.ends_with("ss") {
        return word;
    }
    else if word.ends_with("s") {
        word.truncate(word.len()-1);
        return word;
    }
    return word;
}

fn stem1b(mut word: String) -> String {
    if word.ends_with("eed") {
        let mut measure_word = word.clone();
        measure_word.truncate(measure_word.len()-3);
        if measure(&measure_word) > 0 {
            word.truncate(word.len()-1);
            return word;
        }
    }
    else if word.ends_with("ed") {
        let mut measure_word = word.to_lowercase();
        measure_word.truncate(measure_word.len()-2);
        if has_vowel(&measure_word) {
            word.truncate(word.len()-2);
            return stem1bresolve(word);
        }
    }
    else  if word.ends_with("ing") {
        let mut measure_word = word.to_lowercase();
        measure_word.truncate(measure_word.len()-3);
        if has_vowel(&measure_word) {
            word.truncate(word.len()-3);
            return stem1bresolve(word);
        }
    }
    return word;
}

fn get_char_at_position(word: &String, position: usize) -> Option<char> {
    if position == 0 {
        return None;
    }
    let mut my_chars = word.chars().skip(position-1);          
    let my_char = my_chars.next();
    return my_char;
}

// Can probably be improved if we just grab the last 3 letters and use them directly instead of the get_char_at_position. Should result in fewer skips etc.
fn stem1bresolve(mut word: String) -> String {
    println!("{}", word);
    if word.ends_with("at") || word.ends_with("bl") || word.ends_with("iz") {
        word.push('e');
        return word;
    }
    else if (get_char_at_position(&word, word.len()) == get_char_at_position(&word, word.len()-1)) && 
        is_consonant(get_char_at_position(&word, word.len()-1), get_char_at_position(&word, word.len()-2)) {
            let foo = get_char_at_position(&word, word.len()-1);
            match  foo {
                Some('l') | Some('s') | Some('z') => return word,
                _ => {
                    word.pop();
                    return word;
                }

            }
    }
    else if measure(&word) == 1 && 
        is_consonant(get_char_at_position(&word, word.len()), get_char_at_position(&word, word.len()-1)) &&
        !is_consonant(get_char_at_position(&word, word.len()-1), get_char_at_position(&word, word.len()-2)) &&
        is_consonant(get_char_at_position(&word, word.len()-2), get_char_at_position(&word, word.len()-3)) {
            if word.ends_with('w') | word.ends_with('x') | word.ends_with('y') {
                return word;
            }
            else {
                word.push('e');
                return word;
            }
    }
    return word;
    
}

fn stem1c(mut word: String) -> String {
    let l = word.pop();
    if has_vowel(&word) && l == Some('y') {
        word.push('i');
        return word;
    }
    word.push(l.unwrap());
    return word;
}

fn stem2(mut word: String) -> String {
    let letter = get_char_at_position(&word, word.len()-1);
    match letter {
        Some('a') => 
            if word.ends_with("ational") && measure_with_limit(&word,  word.len() - 7) > 0 {
                word.truncate(word.len()-7);
                word.push_str("ate");
                return word;
            }
            else if word.ends_with("tional") && measure_with_limit(&word,  word.len() - 6) > 0 {
                word.truncate(word.len()-6);
                word.push_str("tion");
                return word;
            },  

        Some('c') => 
            if word.ends_with("enci") && measure_with_limit(&word,  word.len() - 4) > 0 {
                word.truncate(word.len()-4);
                word.push_str("ence");
                return word;
            }
            else if word.ends_with("anci") && measure_with_limit(&word,  word.len() - 4) > 0 {
                word.truncate(word.len()-4);
                word.push_str("ance");
                return word;
            },
        Some('e') => 
            if word.ends_with("izer") && measure_with_limit(&word,  word.len() - 4) > 0 {
                word.truncate(word.len()-4);
                word.push_str("ize");
                return word;
            },
        Some('l') => 
            if word.ends_with("abli") && measure_with_limit(&word,  word.len() - 4) > 0 {
                word.truncate(word.len()-4);
                word.push_str("able");
                return word;
            }
            else if word.ends_with("alli") && measure_with_limit(&word,  word.len() - 4) > 0 {
                word.truncate(word.len()-4);
                word.push_str("al");
                return word;
            }
            else if word.ends_with("entli") && measure_with_limit(&word,  word.len() - 5) > 0 {
                word.truncate(word.len()-5);
                word.push_str("ent");
                return word;
            }
            else if word.ends_with("ousli") && measure_with_limit(&word,  word.len() - 5) > 0 {
                word.truncate(word.len()-5);
                word.push_str("ous");
                return word;
            }
            else if word.ends_with("eli") && measure_with_limit(&word,  word.len() - 3) > 0 {
                word.truncate(word.len()-3);
                word.push_str("e");
                return word;
            },
        Some('o') => 
            if word.ends_with("ization") && measure_with_limit(&word,  word.len() - 7) > 0 {
                word.truncate(word.len()-7);
                word.push_str("ize");
                return word;
            }
            else if word.ends_with("ation") && measure_with_limit(&word,  word.len() - 5) > 0 {
                word.truncate(word.len()-5);
                word.push_str("ate");
                return word;
            }
            else if word.ends_with("ator") && measure_with_limit(&word,  word.len() - 4) > 0 {
                word.truncate(word.len()-4);
                word.push_str("ate");
                return word;
            },
        Some('s') => 
            if word.ends_with("alism") && measure_with_limit(&word,  word.len() - 5) > 0 {
                word.truncate(word.len()-5);
                word.push_str("al");
                return word;
            }
            else if word.ends_with("iveness") && measure_with_limit(&word,  word.len() - 7) > 0 {
                word.truncate(word.len()-7);
                word.push_str("ive");
                return word;
            }
            else if word.ends_with("fulness") && measure_with_limit(&word,  word.len() - 7) > 0 {
                word.truncate(word.len()-7);
                word.push_str("ful");
                return word;
            }
            else if word.ends_with("ousness") && measure_with_limit(&word,  word.len() - 7) > 0 {
                word.truncate(word.len()-7);
                word.push_str("ous");
                return word;
            },
        Some('t') => 
            if word.ends_with("aliti") && measure_with_limit(&word,  word.len() - 5) > 0 {
                word.truncate(word.len()-5);
                word.push_str("al");
                return word;
            }
            else if word.ends_with("iviti") && measure_with_limit(&word,  word.len() - 5) > 0 {
                word.truncate(word.len()-5);
                word.push_str("ive");
                return word;
            }
            else if word.ends_with("biliti") && measure_with_limit(&word,  word.len() - 6) > 0 {
                word.truncate(word.len()-6);
                word.push_str("ble");
                return word;
            },
        _ => return word,
    }
    return word;
}

fn stem3(mut word: String) -> String {
    let letter = get_char_at_position(&word, word.len()-1);
    match letter {
        Some('a') => 
            if word.ends_with("ical") && measure_with_limit(&word, word.len()-4) > 0 {
                word.truncate(word.len()-4);
                word.push_str("ic");
            },
        Some('t') =>
            if word.ends_with("icate") && measure_with_limit(&word, word.len()-5) > 0 {
                word.truncate(word.len()-5);
                word.push_str("ic");
            }
            else if word.ends_with("iciti") && measure_with_limit(&word, word.len()-5) > 0 {
                word.truncate(word.len()-5);
                word.push_str("ic");
            },
        Some('u') => 
            if word.ends_with("ful") && measure_with_limit(&word, word.len()-3) > 0 {
                word.truncate(word.len()-3);
            },
        Some('s') => 
            if word.ends_with("ness") && measure_with_limit(&word, word.len()-4) > 0 {
                word.truncate(word.len()-4);
            },
        Some('v') => 
            if word.ends_with("ative") && measure_with_limit(&word, word.len()-5) > 0 {
                word.truncate(word.len()-5);
            },
        Some('z') => 
            if word.ends_with("alize") && measure_with_limit(&word, word.len()-5) > 0 {
                word.truncate(word.len()-5);
                word.push_str("al");
            },
        _ => return word,

    }
    return word;
}

fn stem4_helper(word: String, end: String) -> bool {
    if word.ends_with(end.as_str()) && measure_with_limit(&word,  word.len() - end.len()) > 1 {
        return true;
    }
    return false;
}
fn stem4_helper_2(mut word: String, end: String) -> String {
    word.truncate(word.len()-end.len());
    return word;
}

// Drop the calls to stem4_helper and implement directly. Nothing is destructive so we don't need the clone and the login isn't 
// as bad as it was
// Drop the calls to stem4_helper_2 we can simply truncate each word and have the single return at the end.
fn stem4(word: String) -> String {
    let letter = get_char_at_position(&word, word.len()-1);
    match letter {
        Some('a') => if stem4_helper(word.clone(), String::from("al")) {
            return stem4_helper_2(word, String::from("al"));
        },
        Some('c') => if stem4_helper(word.clone(), String::from("ance")) {
            return stem4_helper_2(word, String::from("ance"));
        }
        else if stem4_helper(word.clone(), String::from("ence")) {
            return stem4_helper_2(word, String::from("ence"));
        },
        Some('e') => if stem4_helper(word.clone(), String::from("er")) {
            return stem4_helper_2(word, String::from("er"));
        },
        Some('i') => if stem4_helper(word.clone(), String::from("ic")) {
            return stem4_helper_2(word, String::from("ic"));
        },
        Some('l') => if stem4_helper(word.clone(), String::from("able")) {
            return stem4_helper_2(word, String::from("able"));
        }
        else if stem4_helper(word.clone(), String::from("ible")) {
            return stem4_helper_2(word, String::from("ible"));
        },
        Some('n') => if stem4_helper(word.clone(), String::from("ant")) {
            return stem4_helper_2(word, String::from("ant"));
        }
        else if stem4_helper(word.clone(), String::from("ement")) {
            return stem4_helper_2(word, String::from("ement"));
        }
        else if stem4_helper(word.clone(), String::from("ment")) {
            return stem4_helper_2(word, String::from("ment"));
        }
        else if stem4_helper(word.clone(), String::from("ent")) {
            return stem4_helper_2(word, String::from("ent"));
        },
        Some('o') => if word.ends_with("ion") && measure_with_limit(&word,  word.len() - 4) > 1 {
            let letter2 = get_char_at_position(&word, word.len()-3);
            match letter2 {
                Some('s') | Some('t') => return stem4_helper_2(word, String::from("ion")),
                _ => return word,
            }
        }
        ,
        Some('s') => if stem4_helper(word.clone(), String::from("ism")) {
            return stem4_helper_2(word, String::from("ism"));
        },
        Some('t') => if stem4_helper(word.clone(), String::from("ate")) {
            return stem4_helper_2(word, String::from("ate"));
        }
        else if stem4_helper(word.clone(), String::from("iti")) {
            return stem4_helper_2(word, String::from("iti"));
        },
        Some('u') => if stem4_helper(word.clone(), String::from("ous")) {
            return stem4_helper_2(word, String::from("ous"));
        },
        Some('v') => if stem4_helper(word.clone(), String::from("ive")) {
            return stem4_helper_2(word, String::from("ive"));
        },
        Some('z') => if stem4_helper(word.clone(), String::from("ize")) {
            return stem4_helper_2(word, String::from("ize"));
        },
        _ => return word
    }
    return word;
}

fn stem5a_helper(word: String) -> bool {
    let mut one: Option<char> = None;
    if word.len() >=4 {
        let my_chars = word.chars();
        let mut foo = my_chars.skip(word.len()-4);
        one = foo.next();
    }
    let my_chars = word.chars();
    let mut foo = my_chars.skip(word.len()-3);
    let two = foo.next();
    let three = foo.next();
    let four = foo.next();
    if !is_consonant(two, one) | is_consonant(three, two) | !is_consonant(four, three) {
        return true;
    }
    else {
        match four {
            Some('x') | Some('y') | Some('w') => return true,
            _ => return false,
        }
    }
}

fn stem5a(mut word: String) -> String {
    let y = word.pop().unwrap();
    if y == 'e' {
        if measure(&word) > 1 || (measure(&word)== 1 && stem5a_helper(word.clone())) {
            return word;
        }
    }
    word.push(y);
    return word;
}

fn stem5b(mut word: String) -> String {
    if word.ends_with("l") {
        let my_chars = word.chars();
        let mut foo = my_chars.skip(word.len()-3);

        let one = foo.next();
        let two = foo.next();
        let three = foo.next();
        let y = word.pop().unwrap();
        if measure(&word) > 1 && is_consonant(two, one) && is_consonant(three, two) {
            return word;
        }
        word.push(y);
    }
    return word;
}

mod tests {
    use super::*;

    #[test]
    fn test_is_consonant() {
        assert_eq!(is_consonant(Some('a'), None), false);
        assert_eq!(is_consonant(Some('e'), None), false);
        assert_eq!(is_consonant(Some('i'), None), false);
        assert_eq!(is_consonant(Some('o'), None), false);
        assert_eq!(is_consonant(Some('u'), None), false);
        assert_eq!(is_consonant(Some('y'), None), false);
        assert_eq!(is_consonant(Some('y'), Some('a')), true);
        assert_eq!(is_consonant(Some('y'), Some('b')), false);
        assert_eq!(is_consonant(Some('b'), None), true);
        assert_eq!(is_consonant(Some('c'), None), true);
        assert_eq!(is_consonant(Some('d'), None), true);
        assert_eq!(is_consonant(Some('f'), Some('b')), true);
        
    }

    #[test]
    fn test_has_vowel() {
        assert_eq!(has_vowel(&String::from("test")), true);
        assert_eq!(has_vowel(&String::from("tst")), false);
        assert_eq!(has_vowel(&String::from("tsty")), true);
    }

    #[test]
    fn test_measure() {
        assert_eq!(measure(&String::from("tr")), 0);
        assert_eq!(measure(&String::from("ee")), 0);
        assert_eq!(measure(&String::from("tree")), 0);
        assert_eq!(measure(&String::from("y")), 0);
        assert_eq!(measure(&String::from("by")), 0);
        assert_eq!(measure(&String::from("trouble")), 1);
        assert_eq!(measure(&String::from("oats")), 1);
        assert_eq!(measure(&String::from("trees")), 1);
        assert_eq!(measure(&String::from("ivy")), 1);
        assert_eq!(measure(&String::from("troubles")), 2);
        assert_eq!(measure(&String::from("private")), 2);
        assert_eq!(measure(&String::from("oaten")), 2);
        assert_eq!(measure(&String::from("orrery")), 2);
    }

    #[test]
    fn test_measure_with_limit() {
        assert_eq!(measure_with_limit(&String::from("troubles"), 4), 0);
        assert_eq!(measure_with_limit(&String::from("troubles"), 5), 1);
        assert_eq!(measure_with_limit(&String::from("private"), 4), 1);
        assert_eq!(measure_with_limit(&String::from("oaten"), 5), 2);
        assert_eq!(measure_with_limit(&String::from("oaten"), 4), 1);
        assert_eq!(measure_with_limit(&String::from("oaten"), 3), 1);
        assert_eq!(measure_with_limit(&String::from("oaten"), 2), 0);
        assert_eq!(measure_with_limit(&String::from("orrery"), 6), 2);
        assert_eq!(measure_with_limit(&String::from("orrery"), 5), 2);
        assert_eq!(measure_with_limit(&String::from("orrery"), 4), 1);
        assert_eq!(measure_with_limit(&String::from("orrery"), 3), 1);
        assert_eq!(measure_with_limit(&String::from("orrery"), 2), 1);
        assert_eq!(measure_with_limit(&String::from("orrery"), 1), 0);
    }

    #[test]
    fn test_stem() {
        assert_eq!(stem(String::from("is")), String::from("is"));
        assert_eq!(stem(String::from("caresses")), String::from("caress"));
        assert_eq!(stem(String::from("ponies")), String::from("poni"));
        assert_eq!(stem(String::from("caress")), String::from("caress"));
        assert_eq!(stem(String::from("cats")), String::from("cat"));
        assert_eq!(stem(String::from("generalization")), String::from("gener"));
        assert_eq!(stem(String::from("oscillators")), String::from("oscil"));
        assert_eq!(stem(String::from("a")), String::from("a"));
        assert_eq!(stem(String::from("ababab")), String::from("ababab"));
        assert_eq!(stem(String::from("airs")), String::from("air"));
        assert_eq!(stem(String::from("ars")), String::from("ar"));
        assert_eq!(stem(String::from("trouble")), String::from("troubl"));
        assert_eq!(stem(String::from("dependent")), String::from("depend"));
    }

    #[test]
    fn test_stem1a() {
        assert_eq!(String::from(stem1a(String::from("caresses"))), String::from("caress"));
        assert_eq!(String::from(stem1a(String::from("ponies"))), String::from("poni"));
        assert_eq!(String::from(stem1a(String::from("caress"))), String::from("caress"));
        assert_eq!(String::from(stem1a(String::from("cats"))), String::from("cat"));
    }

    #[test]
    fn test_stem1b() {
        assert_eq!(stem1b(String::from("feed")), String::from("feed"));
        assert_eq!(stem1b(String::from("agreed")), String::from("agree"));
        assert_eq!(stem1b(String::from("plastered")), String::from("plaster"));
        assert_eq!(stem1b(String::from("bled")), String::from("bled"));
        assert_eq!(stem1b(String::from("motoring")), String::from("motor"));
        assert_eq!(stem1b(String::from("sing")), String::from("sing"));
        assert_eq!(stem1b(String::from("conflated")), String::from("conflate"));
        assert_eq!(stem1b(String::from("troubled")), String::from("trouble"));
        assert_eq!(stem1b(String::from("sized")), String::from("size"));
        assert_eq!(stem1b(String::from("be")), String::from("be"));
    }

    #[test]
    fn test_stem1bresolve() {
        assert_eq!(stem1bresolve(String::from("conflat")), String::from("conflate"));
        assert_eq!(stem1bresolve(String::from("troubl")), String::from("trouble"));
        assert_eq!(stem1bresolve(String::from("siz")), String::from("size"));
        assert_eq!(stem1bresolve(String::from("hopp")), String::from("hop"));
        assert_eq!(stem1bresolve(String::from("tann")), String::from("tan"));
        assert_eq!(stem1bresolve(String::from("fall")), String::from("fall"));
        assert_eq!(stem1bresolve(String::from("hiss")), String::from("hiss"));
        assert_eq!(stem1bresolve(String::from("fail")), String::from("fail"));
        assert_eq!(stem1bresolve(String::from("fil")), String::from("file"));
    }

    #[test]
    fn test_get_char_at_position() {
        assert_eq!(get_char_at_position(&String::from("fubar"), 3), Some('b'));
        assert_eq!(get_char_at_position(&String::from("fubar"), 1), Some('f'));
        assert_eq!(get_char_at_position(&String::from("fubar"), 5), Some('r'));
        assert_eq!(get_char_at_position(&String::from("fubar"), 6), None);
        let foo = String::from("fubar");
        assert_eq!(get_char_at_position(&foo, foo.len()), Some('r'));


    }

    #[test]
    fn test_stem1c() {
        assert_eq!(stem1c(String::from("happy")), String::from("happi"));
        assert_eq!(stem1c(String::from("sky")), String::from("sky"));
    }

    #[test]
    fn test_stem2() {
        assert_eq!(stem2(String::from("relational")), String::from("relate"));
        assert_eq!(stem2(String::from("conditional")), String::from("condition"));
        assert_eq!(stem2(String::from("valenci")), String::from("valence"));
        assert_eq!(stem2(String::from("hesitanci")), String::from("hesitance"));
        assert_eq!(stem2(String::from("digitizer")), String::from("digitize"));
        assert_eq!(stem2(String::from("conformabli")), String::from("conformable"));
        assert_eq!(stem2(String::from("radicalli")), String::from("radical"));
        assert_eq!(stem2(String::from("differentli")), String::from("different"));
        assert_eq!(stem2(String::from("analogousli")), String::from("analogous"));
        assert_eq!(stem2(String::from("vileli")), String::from("vile"));
        assert_eq!(stem2(String::from("vietnamization")), String::from("vietnamize"));
        assert_eq!(stem2(String::from("predication")), String::from("predicate"));
        assert_eq!(stem2(String::from("operator")), String::from("operate"));
        assert_eq!(stem2(String::from("feudalism")), String::from("feudal"));
        assert_eq!(stem2(String::from("decisiveness")), String::from("decisive"));
        assert_eq!(stem2(String::from("hopefulness")), String::from("hopeful"));
        assert_eq!(stem2(String::from("callousness")), String::from("callous"));
        assert_eq!(stem2(String::from("formaliti")), String::from("formal"));
        assert_eq!(stem2(String::from("sensitiviti")), String::from("sensitive"));
        assert_eq!(stem2(String::from("sensibiliti")), String::from("sensible"));
    }

    #[test]
    fn test_stem3() {
        assert_eq!(stem3(String::from("electrical")), String::from("electric"));
        assert_eq!(stem3(String::from("triplicate")), String::from("triplic"));
        assert_eq!(stem3(String::from("electriciti")), String::from("electric"));
        assert_eq!(stem3(String::from("hopeful")), String::from("hope"));
        assert_eq!(stem3(String::from("goodness")), String::from("good"));
        assert_eq!(stem3(String::from("formative")), String::from("form"));
        assert_eq!(stem3(String::from("formalize")), String::from("formal"));
    }

    #[test]
    fn test_stem4() {
        assert_eq!(stem4(String::from("revival")), String::from("reviv"));
        assert_eq!(stem4(String::from("allowance")), String::from("allow"));
        assert_eq!(stem4(String::from("inference")), String::from("infer"));
        assert_eq!(stem4(String::from("airliner")), String::from("airlin"));
        assert_eq!(stem4(String::from("gyroscopic")), String::from("gyroscop"));
        assert_eq!(stem4(String::from("adjustable")), String::from("adjust"));
        assert_eq!(stem4(String::from("defensible")), String::from("defens"));
        assert_eq!(stem4(String::from("irritant")), String::from("irrit"));
        assert_eq!(stem4(String::from("replacement")), String::from("replac"));
        assert_eq!(stem4(String::from("adjustment")), String::from("adjust"));
        assert_eq!(stem4(String::from("dependent")), String::from("depend"));
        assert_eq!(stem4(String::from("adoption")), String::from("adopt"));
        assert_eq!(stem4(String::from("homologous")), String::from("homolog"));
        assert_eq!(stem4(String::from("communism")), String::from("commun"));
        assert_eq!(stem4(String::from("activate")), String::from("activ"));
        assert_eq!(stem4(String::from("angulariti")), String::from("angular"));
        assert_eq!(stem4(String::from("homologous")), String::from("homolog"));
        assert_eq!(stem4(String::from("effective")), String::from("effect"));
        assert_eq!(stem4(String::from("bowdlerize")), String::from("bowdler"));
    }

    #[test]
    fn test_stem5a() {
        assert_eq!(stem5a(String::from("probate")), String::from("probat"));
        assert_eq!(stem5a(String::from("rate")), String::from("rate"));
        assert_eq!(stem5a(String::from("cease")), String::from("ceas"));
        assert_eq!(stem5a(String::from("trouble")), String::from("troubl"));
        assert_eq!(stem5a(String::from("trocawe")), String::from("trocaw"));
        assert_eq!(stem5a(String::from("pcace")), String::from("pcace"));
    }

    #[test]
    fn test_stem5b() {
        assert_eq!(stem5b(String::from("controll")), String::from("control"));
        assert_eq!(stem5b(String::from("roll")), String::from("roll"));
    }

}