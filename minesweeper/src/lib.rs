use std::collections::{BTreeMap};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut mine: Vec<String> = vec![];
    let stars = get_stars(minefield);
    println!("{:?}", stars);
    for (i, star) in stars.iter() {
        let mut count;
        let mut string = String::new();
        for (j, s) in star.iter().enumerate() {
            count = 0;
            if *s == 1000 {
                let first_line= if *i as i32-1 >= 0 {stars.get(&(i-1)).is_some()} else { false };
                let second_line = stars.get(&i).is_some();
                let third_line = stars.get(&(i+1)).is_some();
                if first_line {
                    if j as i32-1 >= 0 && stars.get(&(i-1)).unwrap().get(j-1).is_some() {
                        if *stars.get(&(i -1)).unwrap().get(j-1).unwrap() != 1000 {
                            count += 1;
                        }
                    }
                    if stars.get(&(i-1)).unwrap().get(j).is_some(){
                        if *stars.get(&(i-1)).unwrap().get(j).unwrap() != 1000 {
                            count += 1;
                        }
                    }
                    if stars.get(&(i-1)).unwrap().get(j+1).is_some(){
                        if *stars.get(&(i-1)).unwrap().get(j+1).unwrap() != 1000 {
                            count += 1;
                        }
                    }
                }
                if second_line {
                    if j as i32-1 >= 0 && stars.get(&(i)).unwrap().get(j-1).is_some() {
                        if *stars.get(&(i)).unwrap().get(j-1).unwrap() != 1000 {
                            count += 1;
                        }
                    }
                    if stars.get(&(i)).unwrap().get(j+1).is_some(){
                        if *stars.get(&(i)).unwrap().get(j+1).unwrap() != 1000 {
                            count += 1;
                        }
                    }
                }
                if third_line {
                    if j as i32-1 >= 0 && stars.get(&(i + 1)).unwrap().get(j-1).is_some() {
                        if *stars.get(&(i + 1)).unwrap().get(j-1).unwrap() != 1000 {
                            count += 1;
                        }
                    }
                    if stars.get(&(i + 1)).unwrap().get(j).is_some(){
                        if *stars.get(&(i + 1)).unwrap().get(j).unwrap() != 1000 {
                            count += 1;
                        }
                    }
                    if stars.get(&(i + 1)).unwrap().get(j+1).is_some(){
                        if *stars.get(&(i+1)).unwrap().get(j+1).unwrap() != 1000 {
                            count += 1;
                        }
                    }
                }
            }

            let count_string = if count != 0 {
                count.to_string()
            } else if count == 0 && *s == 1000 {
                " ".to_string()
            } else {
                "*".to_string()
            };
            string += count_string.as_str();
        }
        mine.push(string)
    }

    mine
}

fn get_stars(minefield: &[&str]) -> BTreeMap<usize, Vec<usize>> {
    let mut star_position:BTreeMap<usize, Vec<usize>> =  BTreeMap::new();

    for (i, str) in minefield.iter().enumerate() {
        let mut arr = vec![];
        for (j, mine) in str.chars().enumerate() {

            if mine == '*' {
                arr.push(j);
            } else {
                arr.push(1000);
            }
        }
        star_position.insert(i, arr);
    }

    star_position
}
