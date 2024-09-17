          

          // here we learn about the function inputs and coercion
fn vowel(word:&String)->u8{
    let vowel_count = word.chars()
        .into_iter()
        .filter(|x|(*x == 'a')|(*x == 'e')|(*x == 'i')|(*x== 'o')|(*x == 'u')).count();
    vowel_count as u8
}          

fn main() {
    let check = "Bhupal".to_string();
    println!("Number of vowels in the word: {}", vowel(&check));
}
