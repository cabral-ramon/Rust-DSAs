/*
 * You are given a string consisting of lowercase English letters. 
 * Repeatedly remove adjacent duplicate letters, one pair at a time. 
 * Both members of a pair of adjacent duplicate letters need to be removed.
 * Examples: 'aaa' => 'a', 'abbaaca' => 'aca', 'azxxzy' => 'ay'
*/

fn main() {
   assert_eq!(solution("aaa"), "a".to_string());
   assert_eq!(solution("abbaaca"), "aca".to_string());
   assert_eq!(solution("azxxzy"), "ay".to_string());
   println!("Hola Mundo!");
}

fn solution(s: &str) -> String {
   let mut stack: Vec<u8> = Vec::with_capacity(s.len());

   for c in s.bytes() {
      if stack.ends_with(&[c]) {
         stack.pop();
      } else {
         stack.push(c);
      }
   }

   String::from_utf8(stack).expect("Expected chars to be valid utf-8")
}
