fn main() {
  let x: int =  signum(2);
}

fn signum(x: int) -> int {
  if x<0 { -1 }
  else if x>0 { 1 }
  else { return 0 }
}
