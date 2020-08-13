fn main() {
    println!("Hello, world!");
}

pub fn fizz_buzz(num: i32) -> String {
    if is_fizz(num) && is_buzz(num) {
      return "FizzBuzz".to_string()
    } 

    if is_fizz(num) {
      return "Fizz".to_string()
    } 

    if is_buzz(num) {
      return "Buzz".to_string()
    } 
    num.to_string()
}

fn is_fizz(num: i32) -> bool {
    num % 3 == 0
}

fn is_buzz(num: i32) -> bool {
    num % 5 == 0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn _1_to_1() {
        assert_eq!(fizz_buzz(1), "1");
    }

    #[test]
    fn _2_to_2() {
        assert_eq!(fizz_buzz(2), "2");
    }

    #[test]
    fn _3_to_fizz() {
        assert_eq!(fizz_buzz(3), "Fizz");
    }

    
    #[test]
    fn _6_to_fizz() {
        assert_eq!(fizz_buzz(6), "Fizz");
    }

    #[test]
    fn _5_to_buzz() {
        assert_eq!(fizz_buzz(5), "Buzz");
    }

    #[test]
    fn _15_to_fizz_buzz() {
        assert_eq!(fizz_buzz(15), "FizzBuzz");
    }

}