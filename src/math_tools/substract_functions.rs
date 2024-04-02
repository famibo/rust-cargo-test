pub fn substract_five(num: u32) -> u32 {
  num - 5 // substracts 5
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn substract_five_test() {
    let x = 100;
    let y = substract_five(x);
    println!("x and y are from test: {} {}", x, y );
    assert_eq!(y, 95);
  }
}