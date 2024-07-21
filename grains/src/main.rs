pub fn square(s: u32) -> u64 {
    if s > 64 || s == 0{
        panic!("Square must be between 1 and 64")
    }else{
      2u64.pow(s-1)
    }
}
pub fn total() -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=64 {
         sum = sum + square(i);
    }
    sum
}
fn main(){
    let m = square(2);
    println!("The value of square is {m}");
    let n = total();
    println!("The total grains is {n}");
}
