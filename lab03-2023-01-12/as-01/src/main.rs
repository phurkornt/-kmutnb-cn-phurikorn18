fn main() {
    let text = "this cat this rat this bird";
    let words: Vec<&str> = separate_words(text);
    let unique: Vec<&str> = find_unique_words(words);
    let count = count_unique_words(unique);
    println!("Unique is : {}",count);

}


fn separate_words(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

fn find_unique_words(words: Vec<&str>) -> Vec<&str> {
    let mut unique: Vec<&str> = vec![words[0]];
    for i in 1..words.len() {
        for x in 0..unique.len() {
            
            if words[i] == unique[x]{
                break;
            }
            if unique.len()-1 == x {
                unique.push(words[i]);
            }

        }
    }
    return unique;
}
fn count_unique_words(words: Vec<&str>) -> usize {
    return words.len();
}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_separate_words() {
        let words = vec!["this", "cat", "this", "rat", "this", "bird"];
        assert_eq!(separate_words("this cat this rat this bird"), words);
   }
   #[test]
   fn test_unique_words() {
        let words = vec!["this", "cat", "this", "rat", "this", "bird"];
        let unique = vec!["this","cat", "rat", "bird"];
        assert_eq!(find_unique_words(words),unique);
   }
   #[test]
   fn test_count_unique_words() {
        let unique = vec!["this","cat", "rat", "bird"];
        assert_eq!(count_unique_words(unique),4);
   }
}
