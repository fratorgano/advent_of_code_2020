use std::collections::{HashSet, HashMap};

pub fn sim_count_active(input: &String )-> usize{
    //world contains only the active cubes
    let mut world:HashSet<(i8,i8,i8)> = HashSet::new();
    for (i,line) in input.lines().enumerate(){
        for (j,c) in line.chars().enumerate(){
            if c=='#'{
                world.insert((i as i8,j as i8,0));
            }
        }
    }
    //neighbour_count contains the number of neighbours for every active cube and every neighbour of an active cube
    let mut neighbour_count: HashMap<(i8,i8,i8),usize> = HashMap::new();
    for c in &world{
        for dx in -1..=1{
            for dy in -1..=1{
                for dz in -1..=1{
                    if dx==0 && dy==0 && dz==0{
                        //I skip this cause it would be considering the same cube I already have
                        continue;
                    }
                    let cube: (i8,i8,i8) = (c.0+dx,c.1+dy,c.2+dz);
                    *neighbour_count.entry(cube).or_insert(0) += 1;
                }
            }
        }
    }
    // For each couple in neighbour_count, if it has 2 or 3 active neighbours() I put it as active and add his neighbour to the set of neighbours. Doing this for 6 times,
    // After 6 times the world will contain only the active cubes so I can return the len of it
    for _ in 0..6{
        let mut next_world:HashSet<(i8,i8,i8)> = HashSet::new();
        let mut next_neighbour_count: HashMap<(i8,i8,i8),usize> = HashMap::new();
        for (&cube, &neighbours) in &neighbour_count{
            if neighbours==3 || (neighbours==2 && world.contains(&cube)){
                next_world.insert(cube);
                for dx in -1..=1{
                    for dy in -1..=1{
                        for dz in -1..=1{
                            if dx==0 && dy==0 && dz==0{
                                continue;
                            }
                            let neighbour = (cube.0+dx,cube.1+dy,cube.2+dz);
                            *next_neighbour_count.entry(neighbour).or_insert(0) += 1
                        }
                    }
                }
            }
        }
        neighbour_count = next_neighbour_count;
        world = next_world;
    }
    world.len()
}

pub fn sim_count_active_2(input: &String )-> usize{
    // Same as before but using another variable to "index" cubes
    let mut world:HashSet<(i8,i8,i8,i8)> = HashSet::new();
    for (i,line) in input.lines().enumerate(){
        for (j,c) in line.chars().enumerate(){
            if c=='#'{
                world.insert((i as i8,j as i8,0,0));
            }
        }
    }

    let mut neighbour_count: HashMap<(i8,i8,i8,i8),usize> = HashMap::new();
    for c in &world{
        for dx in -1..=1{
            for dy in -1..=1{
                for dz in -1..=1{
                    for dw in -1..=1{
                        if dx==0 && dy==0 && dz==0 && dw==0{
                            continue;
                        }
                        let cube: (i8,i8,i8,i8) = (c.0+dx,c.1+dy,c.2+dz,c.3+dw);
                        *neighbour_count.entry(cube).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    for _ in 0..6{
        let mut next_world:HashSet<(i8,i8,i8,i8)> = HashSet::new();
        let mut next_neighbour_count: HashMap<(i8,i8,i8,i8),usize> = HashMap::new();
        for (&cube, &neighbours) in &neighbour_count{
            if neighbours==3 || (neighbours==2 && world.contains(&cube)){
                next_world.insert(cube);
                for dx in -1..=1{
                    for dy in -1..=1{
                        for dz in -1..=1{
                            for dw in -1..=1{
                                if dx==0 && dy==0 && dz==0 && dw==0{
                                    continue;
                                }
                                let neighbour = (cube.0+dx,cube.1+dy,cube.2+dz,cube.3+dw);
                                *next_neighbour_count.entry(neighbour).or_insert(0) += 1
                            }   
                        }
                    }
                }
            }
        }
        neighbour_count = next_neighbour_count;
        world = next_world;
    }
    world.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_test() {
        let input = String::from(".#.
..#
###");
        assert_eq!(112, sim_count_active(&input));
    }

    #[test]
    fn day17_test_1() {
        let input = String::from(".#.
..#
###");
        assert_eq!(848, sim_count_active_2(&input));

    }

}