pub struct Collatz {
    v: u64,
}

impl Iterator for Collatz {

 type Item = u64;
 fn next(&mut self) -> Option<Self::Item> {
     match self.v {
         0...1 => None,
         v if v % 2 == 0 => {
             self.v /= 2;
             Some(self.v)
         },
         v => {
             self.v = 3 * v + 1;
             Some(self.v)
         }
     }
 }
}

pub fn collatz(n: u64) -> Option<u64> {
 match n {
     0 => None,
     _ => Some(Collatz { v: n }.into_iter().count() as u64)
 }
}

// fn main() {
//  println!("{:?}", collatz(4));
//  println!("{:?}", collatz(5));
//  println!("{:?}", collatz(6));
//  println!("{:?}", collatz(7));
//  println!("{:?}", collatz(12));
// }

#[cfg(test)]

mod tests {
    use super::*;
	
	fn test() {
		assert_eq!(Some(0), collatz(1));
		assert_eq!(Some(4), collatz(16));
		assert_eq!(Some(9), collatz(12));
		assert_eq!(Some(152), collatz(1000000));
		assert_eq!(None, collatz(0));
	}
    
    #[test]
    fn test_none() {
        assert_eq!(None, collatz(0));
    }
}
