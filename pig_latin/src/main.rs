fn main() {
    let s1 = "first";
    let s2 = "apple";

    println!("{}", to_pig_latin(s1));
    println!("{}", to_pig_latin(s2));
}

fn to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_letter = word.chars().next().unwrap();

    if vowels.contains(&first_letter) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}{}", &word[1..], first_letter, "ay")
    }
}