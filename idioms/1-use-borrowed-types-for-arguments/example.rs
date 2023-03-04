#[allow(non_snake_case)]

fn isContainingThreeConsecutiveVowels(word: &str) -> bool {
    let mut consecutiveVowelCount= 0;

    for character in word.chars( ) {
        match character {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                consecutiveVowelCount += 1;

                if consecutiveVowelCount == 3 {
                    return true;}
            }

            _ => consecutiveVowelCount= 0
        }
    }

    return false;
}

fn main( ) {

    let word= "ferris";
    println!("is 3 consecutive vowels present in {} - {}", word, isContainingThreeConsecutiveVowels(word));

    let word= "ferris".to_string( );
    println!(
        "is 3 consecutive vowels present in {} - {}",
        word,
        // coercion - &String -> &str
        isContainingThreeConsecutiveVowels(&word));

}