fn main() {
   println!("Helooo");
}

#[derive(Copy, Clone)]
enum Sign {
   Negative,
   Positive
}

impl Sign {
   fn op(&self, x: i32, y:i32) -> i32 {
      match self {
         Sign::Negative => x - y,
         Sign::Positive => x + y,
      }
   }
}

#[allow(dead_code)]
fn solution(s: &str) -> i32 {
   let mut stack: Vec<(i32, Sign)> = Vec::new();
   let mut curr_num = String::new();
   let mut result = 0;
   let mut sign = Sign::Positive;

   for c in s.chars() {
      if c.is_ascii_digit() {
         // this is being cloned?
         curr_num.push(c);
         continue
      }

      match c {
         '(' => {
            stack.push((result, sign));
            result = 0;
         },
         ')' => {
            result = sign.op(result, curr_num.parse::<i32>().unwrap());
            curr_num.clear();

            let (r, s) = stack.pop().unwrap();
            result = s.op(r, result)
         },
         '+' | '-' => {
            if !curr_num.is_empty() {
               let n = curr_num.parse::<i32>().unwrap();
               result = sign.op(result, n);
               curr_num.clear();
            }
            if c == '+' {
               sign = Sign::Positive;
            } else {
               sign = Sign::Negative;
            }
         },
         ' ' => continue,
         _ => panic!("Unable to parse {}", c),
      }
   };

   if !curr_num.is_empty() {
      result = sign.op(result, curr_num.parse::<i32>().unwrap());
   }
   result
}


#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn works_on_basic_expression() {
      let expr = "12 - (6 + 2) + 5";
      let actual = solution(expr);
      assert_eq!(actual, 9);
   }

   #[test]
   fn case_two() {
      let expr = "(8 + 100) + (13 - 8 - (2 + 1))";
      let actual = solution(expr);
      assert_eq!(actual, 110);
   }
}
