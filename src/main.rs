use std::collections::HashMap;

fn mean(numbers: &[i32]) -> f32 {
   return numbers.iter().sum::<i32>() as f32 / numbers.len() as f32;
}

fn mode(numbers: &[i32]) -> i32 {
//work out the mode
let mut occurrences = HashMap::new();

for &value in numbers {
    *occurrences.entry(value).or_insert(0) += 1;
}

occurrences
    .into_iter()
    .max_by_key(|&(_, count) | count)
    .map(|(val, _)| val)
    .expect("Cannot compute the mode of zero numbers")
}

fn median( numbers: &mut [i32]) -> i32 {
        // work out the median
        numbers.sort();

        let median_index = (numbers.len() / 2) + 1;
        return numbers[median_index - 1];
}

fn convert_to_pig_latin(word: &str) -> String {

    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }

    let mut s = String::from(word);
    let initial_letter = s.remove(0);

    if is_vowel(initial_letter) {
        let mut t = String::from(word);
        t.push_str("-hay");
        return t
    } else {
        s.push(initial_letter);
        s.push_str("-ay")
    }
    s
}

fn main() {
    let mut int_list = vec!(1,2,5,4,7,5,1,7,6,51,15, 5);
    
    println!("Hello, world!");
    println!("Mean is: {}", mean(&int_list));
    println!("Median is: {}", median(&mut int_list));
    println!("Mode is: {}", mode(&int_list));

    println!("Pig Latin toad is: {}", convert_to_pig_latin("toad"));
    println!("Pig Latin cheese is: {}", convert_to_pig_latin("cheese"));
    println!("Pig Latin apple is: {}", convert_to_pig_latin("apple"));
}
