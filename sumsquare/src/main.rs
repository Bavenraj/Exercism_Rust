pub fn square_of_sum(n: &u32) -> u32 {
    let mut number: u32 = 0;

    for num in 1..=*n {
        println!("{num}");
        number = &number + &num;
    }

    let sqnum = number * number;
    //println!("{:p}", sqnum);
    sqnum
    
}
pub fn sum_of_squares(n: &u32) -> u32 {
    let mut number: u32 = 1;
    let mut total: u32 = 0;
    for num in 1..=*n{
        number = &num * &num;

        total = &total + &number;
    }
    total
}

pub fn difference(n: &u32, n2: &u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n2)
 }
 
fn main(){

    let n = 10;

    let res=square_of_sum(&n);
    println!("{}", res);

    let res1=sum_of_squares(&n);
    println!("{}", res1);

    let res2=difference(&n, &n);
    println!("{}", res2);
}