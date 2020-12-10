//Both of these are expensive, it can be done with less than O(n^2) and O(n^3) respectively
pub fn find2020_multiply(numbers:&Vec<i32>) -> i32{
    //let first = numbers[0];
    for i in numbers{
        for j in numbers{
            if i+j==2020 {
                println!("{}+{}={}",i,j,i+j);
                return i*j
            }
        }
    }
    1
} 

pub fn find2020_three_multiply(numbers:&Vec<i32>) -> i32{
    //let first = numbers[0];
    for i in numbers{
        for j in numbers{
            for k in numbers{
                if i+j+k==2020 {
                    println!("{}+{}+{}={}",i,j,k,i+j+k);
                    return i*j*k
                }
            }
        }
    }
    1
} 

