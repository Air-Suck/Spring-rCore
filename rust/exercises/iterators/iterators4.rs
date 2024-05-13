// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

struct Num{
    number:u64
}

impl Iterator for Num{
    type Item=u64;
    fn next(&mut self) -> Option<Self::Item>{
        if self.number>1{
            self.number-=1;
            Some(self.number)
        }else if self.number ==1{
            self.number-=1;
            Some(1)
        }else{
            None
        }
    }
}

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    let temp=Num{
        number:num+1
    };
    let result=temp.fold(1, |acc,x| acc*x);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0),
        "the num is {}",factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1),
        "the num is {}",factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2),
        "the num is {}",factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4),
        "the num is {}",factorial(4));
    }
}
