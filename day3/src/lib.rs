// Counts the trees that it encounters by going down and right every iteration and returns it
pub fn count_trees(s:&String, right: usize, down: usize) -> usize{
    let lines:Vec<&str>=s.lines().collect();
    let mut x=0;
    let mut y=0;
    let mut tree_count = 0;
    while y<lines.len(){
        let l:Vec<char> = lines.get(y).unwrap().chars().collect();
        if *l.get(x%l.len()).unwrap()=='#'{
            tree_count+=1;
        }
        x+=right;
        y+=down;
    }
    tree_count
}

// Takes a series of slopes (right,down), counts the numbers of tree hits for each, multiplies them together and returns the result
pub fn multiply_trees(input: &String, s:Vec<(usize,usize)>) -> usize{
    /* let mut mult = 1;
    for slope in s{
        mult *= count_trees(input, slope.0, slope.1)
    } 
    mult 
    */
    s.iter().fold(1,|acc,slope| acc * count_trees(input, slope.0, slope.1))
}