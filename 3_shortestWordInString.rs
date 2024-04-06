fn find_shortest_word(text: &str) -> Option<&str> {
    let mut s_word = None;
    let mut s_len = usize::MAX;
  
    for word in text.split_whitespace() {
      let word_len = word.len();
      if word_len < s_len {
        s_word = Some(word);
        s_len = word_len;
      }
    }
  
    s_word
  }
  
  fn main() {
    let text = "This is a test for rust developer profile";
    let shortest = find_shortest_word(text);
  
    if let Some(word) = shortest {
      println!("The shortest word is: {}", word);
    } else {
      println!("The string is empty...");
    }
  }
  