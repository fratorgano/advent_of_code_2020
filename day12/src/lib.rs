// I loved this day, I feel like my code is fast and well-writted 

#[derive(Debug)]
pub struct Instruction{
    pub direction: char,
    pub value: isize
}

pub fn parse_input(input: &String) -> Vec<Instruction>{
    let mut vec_instructions:Vec<Instruction> = vec!(); 
    for line in input.lines(){
        let direction = line.chars().next().unwrap();
        let value = line.strip_prefix(|c:char| ! c.is_numeric()).unwrap().parse().unwrap();
        vec_instructions.push( Instruction{direction, value});
    }
    vec_instructions
}

pub fn calc_manhattan_distance(from_x:isize, from_y:isize, facing:isize, instructions: &Vec<Instruction>) -> isize{
    let mut angle = facing;
    let mut x = from_x;
    let mut y = from_y;
    for ins in instructions{
        match ins.direction{
            'N' => y+=ins.value,
            'S' => y-=ins.value,
            'E' => x+=ins.value,
            'W' => x-=ins.value,
            'L' => {angle-=ins.value; angle = (angle+360)%360},
            'R' => {angle+=ins.value; angle = (angle+360)%360},
            'F' => match angle{
                0 => y+=ins.value,
                90 => x+=ins.value,
                180 => y-=ins.value,
                270 => x-=ins.value,
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }
    x.abs()+y.abs()
}

pub fn calc_manhattan_distance_waypoint(waypoint_x:isize, waypoint_y:isize, ship_x:isize, ship_y:isize, instructions: &Vec<Instruction>) -> isize{
    let mut sh_x = ship_x;
    let mut sh_y = ship_y;
    let mut wp_x = waypoint_x;
    let mut wp_y = waypoint_y;
    for ins in instructions{
        match ins.direction{
            'N' => wp_y+=ins.value,
            'S' => wp_y-=ins.value,
            'E' => wp_x+=ins.value,
            'W' => wp_x-=ins.value,
            'L' => {match ins.value%360{
                0 => (),
                90 => {let app = wp_x; wp_x = -wp_y; wp_y = app},
                180 => {wp_x=-wp_x;wp_y=-wp_y},
                270 => {let app = -wp_x; wp_x = wp_y; wp_y = app},
                _ => unreachable!(),
            }},
            'R' => {match ins.value%360{
                0 => (),
                90 => {let app = -wp_x; wp_x = wp_y; wp_y = app},
                180 => {wp_x=-wp_x;wp_y=-wp_y},
                270 => {let app = wp_x; wp_x = -wp_y; wp_y = app},
                _ => unreachable!(),
            }},
            'F' => {sh_x+=wp_x*ins.value; sh_y+=wp_y*ins.value},
            _ => unreachable!()
        }
        /* println!("Ship: {} {}", sh_x,sh_y);
        println!("Waypoint: {} {}", wp_x,wp_y);
        println!(""); */
    }
    sh_x.abs()+sh_y.abs()
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn day12_test() {
        let input = String::from("F10
N3
F7
R90
F11");
        let ins = parse_input(&input);
        assert_eq!(25,calc_manhattan_distance(0,0,90,&ins));
    }
    #[test]
    fn day12_test_2() {
        let input = String::from("F10
N3
F7
R90
F11");
        let ins = parse_input(&input);
        assert_eq!(286,calc_manhattan_distance_waypoint(10,1,0,0,&ins));
    }
}