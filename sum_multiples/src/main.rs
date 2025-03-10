pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    let mut intermediate_result: Vec<u32> = Vec::new();

    for &factor in factors {
        //break the infinite loop
        if factor == 0 {
            continue; 
        }

        let mut i = 1;
        while i * factor < limit {
            let multiple = i * factor;
            //check duplicates
            if !intermediate_result.contains(&multiple) {
                //add int if it is not duplicates
                intermediate_result.push(multiple);
            }
            i += 1;
        }
    }

    for result in intermediate_result {
        sum += result;
    }

    sum    
}

fn main(){
    let s = sum_of_multiples(10,&[2,3]);
    println!("The value of s is {s}");
}