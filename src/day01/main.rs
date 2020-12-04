use std::fs;


fn find_pair(inputs: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    for i in 0..inputs.len()-1 {
        for j in i+1..inputs.len() {
            if inputs[i] + inputs[j] == sum {
                return Some((inputs[i], inputs[j]));
            }
        }
    }
    None
}


fn find_trio(inputs: &Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    for i in 0..inputs.len()-2 {
        for j in i+1..inputs.len()-1 {
            for k in j+1..inputs.len() {
                if inputs[i] + inputs[j] + inputs[k] == sum {
                    return Some((inputs[i], inputs[j], inputs[k]));
                }
            }
        }
    }
    None
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Unable to read input.txt");

    let inputs: Vec<i32> = contents.split_ascii_whitespace().map(|v| v.parse::<i32>().unwrap()).collect();
    let pair = find_pair(&inputs, 2020);
    match pair {
        Some((x, y)) => {
            println!("{}", x * y);
        },
        _ => {}
    };

    let trio = find_trio(&inputs, 2020);
    match trio {
        Some((x, y, z)) => {
            println!("{}", x * y * z);
        },
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pair() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let pair = find_pair(&inputs, 2020);
        assert!(pair.is_some());
        assert_eq!(pair.unwrap().0 + pair.unwrap().1, 2020);
    }

    #[test]
    fn test_find_trio() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let trio = find_trio(&inputs, 2020);
        assert!(trio.is_some());
        assert_eq!(trio.unwrap().0 + trio.unwrap().1 + trio.unwrap().2, 2020);
    }
}
