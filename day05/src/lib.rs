pub fn find_seat(bsp: &str) -> (usize,usize,usize){
    //bsp = binary space partitioning
    let mut row_range = (0,127,128);
    let mut column_range = (0,7,8);
    for c in bsp.chars(){
        match c{
            'F' => {
                row_range.2 /= 2;
                row_range.1 -= row_range.2;
            },
            'B' => {
                row_range.2 /= 2;
                row_range.0 += row_range.2;
            },
            'R' => {
                column_range.2 /= 2;
                column_range.0 += column_range.2;
            },
            'L' => {
                column_range.2 /= 2;
                column_range.1 -= column_range.2;
            },
            _ => panic!("Pattern matching found something not expected")
        }
    }
    (row_range.0,column_range.0, row_range.0*8+column_range.0)
}

pub fn find_highest_seat_id(input: &String) -> usize{
    let mut highest = 0;
    for l in input.lines(){
        let place = find_seat(l);
        if place.2>highest {highest=place.2}
    }
    highest
}

//Not required, used it to find the lower bound for find_my_seat
pub fn find_lowest_seat_id(input: &String) -> usize{
    let mut lowest = 951;
    for l in input.lines(){
        let place = find_seat(l);
        if place.2<lowest {lowest=place.2}
    }
    lowest
}

//Between 6 and 951
pub fn find_my_seat_id(input: &String) -> usize{
    let mut list:Vec<usize> = vec![];
    for l in input.lines(){
        list.push(find_seat(l).2);
    }
    list.sort_unstable();
    //println!("{:?}",list);
    let mut my_seat = 6;
    for elem in list{
        if elem != my_seat{ break } else { my_seat+=1 }
    }
    my_seat
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1(){
        let input = "BFFFBBFRRR";
        assert_eq!((70,7,567),find_seat(input));
    }
    #[test]
    fn test_example2(){
        let input = "FFFBBBFRRR";
        assert_eq!((14,7,119),find_seat(input));
    }
    #[test]
    fn test_example3(){
        let input = "BBFFBBFRLL";
        assert_eq!((102,4,820),find_seat(input));
    }
}