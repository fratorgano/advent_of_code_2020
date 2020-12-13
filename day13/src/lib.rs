use std::collections::HashMap;

pub fn find_best_bus(time:usize, buses: &Vec<&str>) -> usize{
    let mut min = usize::MAX;
    let mut min_id = 0;

    for bus in buses{
        let bus_time = bus.parse::<usize>();
        if bus_time.is_ok() {
            let id =  bus_time.unwrap();
            let wait_time = id - (time%id);
            //println!("{} = {} - ({}%{}={})",wait_time,id,time,id,time%id,);
            if wait_time<min{
                min = wait_time;
                min_id = id;
                //println!("{} {}",min,min_id);
            }
        }
    }
    min*min_id
}

pub fn find_best_timestamp(buses:&Vec<&str>) -> usize{
    

    // ids[busNumber] = position in the array
    let mut ids = HashMap::new();

    for (i,bus) in buses.iter().enumerate(){
        let bus_value = bus.parse::<usize>();
        if bus_value.is_ok(){
            ids.insert(bus_value.unwrap(), i);
        }
    }

    let mut min_val = 0;
    let mut running_product = 1;
    
    // Assume that a solution exists: 
    // exists T(timestamp) such that (T + v)%k == 0 for each k(bus_number),v(position in the array)
    // (T+v) means the timestamp + added time based on position on the array (1 minute for each position) so +1 for each position
    // (T+v)%k if this equals 0 it means that we found a value for T that works for the couple(k,v)

    for (k,v) in ids{
        //println!("Considering k = {}, v = {}",k,v);
        while (min_val+v)%k !=0{
            //println!("Incrementing T({}) by {}",min_val,running_product);
            min_val+=running_product
        }
        //println!("Found ({}+{})%{}=={}, proceding with new values, now I'll increment by a multiple of {}",min_val,v,k,(min_val+v)%k,k);
        running_product*=k;
    }
    min_val
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn day12_test() {
        let time = 939;
        let buses = vec!["7","13","x","x","59","x","31","19"];
        assert_eq!(295, find_best_bus(time, &buses));
    }
    #[test]
    fn day12_test_2() {
        let buses = vec!["17","x","13","19"];
        assert_eq!(3417, find_best_timestamp(&buses));
    }
}