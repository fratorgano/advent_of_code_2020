use std::collections::HashSet;

fn parse_input_to_tiles(input:&String) -> HashSet<(isize,isize)> {
    let mut tiles:HashSet<(isize,isize)> = HashSet::new();
    for l in input.lines(){
        let letters:Vec<char> = l.chars().collect();
        let mut position = (0,0);
        let mut i = 0;
        while i < letters.len(){
            match letters[i] {
                's' => {
                    match letters[i+1] {
                        'e' => position = (position.0+1,position.1-1),
                        'w' => position = (position.0,position.1-1),
                        _ => unreachable!("Unreachable")
                    }
                    i+=1
                },
                'n' => {
                    match letters[i+1] {
                        'e' => position = (position.0,position.1+1),
                        'w' => position = (position.0-1,position.1+1),
                        _ => unreachable!("Unreachable")
                    }
                    i+=1
                }
                'w' => {position = (position.0-1,position.1);},
                'e' => {position = (position.0+1,position.1);},
                _ => unreachable!("Unreachable")
            }
            i+=1;
        }
        /* if let Some(x) = tiles.get_mut(&position) {
            //println!("INVERTING {:?} to {}",position,!*x);
            *x = !*x; */
        if tiles.contains(&position){
            tiles.remove(&position);
        }else{
            //println!("Inserting {:?} to {}",position, false);
            tiles.insert(position);
        }
    }

    tiles
}

pub fn count_black_tiles(input:&String) -> usize{
    let tiles:HashSet<(isize,isize)> = parse_input_to_tiles(input);
    tiles.len()
}

pub fn count_black_tiles_100_days(input:&String) -> usize{
    let neighbors_position = [(-1, 1), (0, 1), (-1, 0), (1, 0), (0, -1), (1, -1)];
    let mut black_tiles:HashSet<(isize,isize)> = parse_input_to_tiles(input);
    for _ in 0..100{
        let mut next_black_tiles: HashSet<(isize,isize)> = HashSet::new();
        let mut white_tiles: HashSet<(isize,isize)> = HashSet::new();

        for (black_tiles_x, black_tiles_y) in black_tiles.iter(){
            let mut neighbors = 0;
            for (delta_x, delta_y) in neighbors_position.iter(){
                let position = (black_tiles_x+delta_x, black_tiles_y+delta_y);
                if black_tiles.contains(&position){
                    neighbors+=1
                }else{
                    white_tiles.insert(position);
                }
            }
            if neighbors == 1 || neighbors==2{
                next_black_tiles.insert((*black_tiles_x,*black_tiles_y));
            }
        }
        for (white_tiles_x, white_tiles_y) in white_tiles.iter(){
            let mut neighbors = 0;
            for (delta_x,delta_y) in neighbors_position.iter(){
                if black_tiles.contains(&(*white_tiles_x+delta_x,*white_tiles_y+delta_y)){
                    neighbors+=1;
                }
            }
            if neighbors == 2{
                next_black_tiles.insert((*white_tiles_x,*white_tiles_y));
            }
        }
        black_tiles = next_black_tiles;
    }
    black_tiles.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24_test() {
        let input = String::from("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew");
        assert_eq!(10,count_black_tiles(&input));
    }

    #[test]
    fn day24_test_2() {
        let input = String::from("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew");
        assert_eq!(2208,count_black_tiles_100_days(&input));
    }
}