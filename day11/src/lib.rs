// Hated this, had to redo it cause it was too slow (I looked up some solution in other languages to get some ideas)
// Most of the commented part is my really slow code
/* use std::convert::TryFrom; */

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Place {
    Floor,
    Empty,
    Occupied
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Map {
    width: usize,
    map: Vec<Place>,
}

const ADJACENT: [(usize, usize); 8] = [
    (!0, 1),
    (0, 1),
    (1, 1),
    (!0, 0),
    (1, 0),
    (!0, !0),
    (0, !0),
    (1, !0),
];


impl Map{

    fn parse(input: &String) -> Self{
        let mut width = None;
        let map = input.lines().flat_map(|line|{
            assert!(width.is_none() || width==Some(line.len()));
            width = Some(line.len());
            line.chars().map(|c| 
                match c{
                '.' => Place::Floor,
                'L' => Place::Empty,
                '#' => Place::Occupied,
                _ => unreachable!() //MAGGICO
            })
        }).collect();
        Map{
            width: width.unwrap_or(1),
            map
        }
    }

    fn get(& self, x:usize, y:usize) -> Option<Place>{
        self.map
            .get(y.checked_mul(self.width)?..)? //I take the elements on the right line
            .get(..self.width)? //Limit the size of what I got to a line
            .get(x)
            .copied()
    }

    fn get_mut(&mut self, x:usize, y:usize) -> Option<&mut Place>{
        self.map
            .get_mut(y.checked_mul(self.width)?..)? //I take the elements on the right line
            .get_mut(..self.width)? //Limit the size of what I got to a line
            .get_mut(x)
    }

    fn height(&self) -> usize {
        self.map.len() / self.width
    }

}

pub fn occupied_seats_sim(input:&String) -> usize{
    let mut map = Map::parse(input);
    let width = map.width;
    let height = map.height();
    loop {
        let occupied = |x:usize,y:usize| {
            let map = &map;
            ADJACENT
                .iter()
                .filter_map(move |&(mod_x, mod_y)| {
                    map.get(x.wrapping_add(mod_x),y.wrapping_add(mod_y))
                })
                .filter(|&elem| elem == Place::Occupied)
                .map(|_| ())
        };
        let mut new_map = map.clone();
        for x in 0..width{
            for y in 0..height{
                let place = new_map.get_mut(x,y).unwrap();
                *place = match *place{
                    Place::Empty if occupied(x,y).next().is_none() => {Place::Occupied},
                    Place::Occupied if occupied(x, y).nth(4 - 1).is_some() => {Place::Empty},
                    place => place,
                }
            }
        }
        if new_map == map {
            return new_map
                .map
                .iter()
                .filter(|&&state| state == Place::Occupied)
                .count();
        }

        map = new_map;
    }


    /* let mut matrix:Vec<Vec<Place>> = vec![];
    for (i, line) in input.lines().enumerate(){
        matrix.push(vec![]);
        for c in line.chars(){
            match c{
                '.' => matrix[i].push(Place::Floor),
                'L' => matrix[i].push(Place::Empty),
                '#' => matrix[i].push(Place::Occupied),
                _ => panic!("Error in the input string!")
            }
        }
    }
    let mut modified = true;
    let mut occupied = 0;
    while modified == true{
        let mut matrixm:Vec<Vec<Place>> = matrix.to_vec();
        modified = false;
        for (i,row) in matrix.iter().enumerate(){
            for (j,place) in row.iter().enumerate(){
                match place{
                    Place::Floor => (),
                    Place::Empty => if is_nearby_occupied(&matrix, i, j)==0 { matrixm[i][j]= Place::Occupied; occupied+=1;modified = true},
                    Place::Occupied => if is_nearby_occupied(&matrix, i, j)>=4 { matrixm[i][j] = Place::Empty; occupied-=1; modified = true }
                }
            }
        }
        matrix = matrixm;
    }
    occupied */
}


pub fn occupied_seats_sim_2(input:&String) -> usize{
    let mut map = Map::parse(input);
    let width = map.width;
    let height = map.height();
    loop {
        let occupied = |x:usize,y:usize| {
            let map = &map;
            ADJACENT
                .iter()
                .filter_map(move |&(mod_x, mod_y)| {
                    let mut x = x;
                    let mut y = y;
                    loop{
                        x = x.wrapping_add(mod_x);
                        y = y.wrapping_add(mod_y);
                        let place = map.get(x,y)?;
                        if place != Place::Floor{
                            return Some(place);
                        }
                    }
                })
                .filter(|&elem| elem == Place::Occupied)
                .map(|_| ())
        };
        let mut new_map = map.clone();
        for x in 0..width{
            for y in 0..height{
                let place = new_map.get_mut(x,y).unwrap();
                *place = match *place{
                    Place::Empty if occupied(x,y).next().is_none() => {Place::Occupied},
                    Place::Occupied if occupied(x, y).nth(5 - 1).is_some() => {Place::Empty},
                    place => place,
                }
            }
        }
        if new_map == map {
            return new_map
                .map
                .iter()
                .filter(|&&state| state == Place::Occupied)
                .count();
        }

        map = new_map;
    }
}
/* fn is_nearby_occupied(map: &Vec<Vec<Place>>, i:usize, j: usize) -> usize{
    let mut count = 0;
    let i:i128 = i128::try_from(i).unwrap();
    let j:i128 = i128::try_from(j).unwrap();
    let len = i128::try_from(map.len()).unwrap()-1;
    let len_row = i128::try_from(map[0].len()).unwrap()-1;

    for index in i-1..=i+1{
        for jindex in j-1..=j+1{
            if i-1>=0 && i+1<len{
                if j-1>=0 && j+1<len_row{
                    if map[usize::try_from(index).unwrap()][usize::try_from(jindex).unwrap()] == Place::Occupied {
                        count+=1;
                    }
                }
            }
        }
    }
    count
} */


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day11_test() {
        let input = String::from(
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL");
        assert_eq!(37,occupied_seats_sim(&input));
    }
    #[test]
    fn day11_test_2() {
        let input = String::from(
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL");
        assert_eq!(26,occupied_seats_sim_2(&input));
    }
}