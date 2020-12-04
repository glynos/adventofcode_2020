use std::fs;


struct PasswordPolicy {
    index_01: usize,
    index_02: usize,
    char: u8,
}


fn parse_password(line: &str) -> (PasswordPolicy, String) {
    let elements = line.split(":").collect::<Vec<&str>>();
    let policy = elements[0].split_ascii_whitespace().collect::<Vec<&str>>();
    let indices = policy[0].split('-').collect::<Vec<&str>>();
    let index_01 = indices[0].parse::<usize>().unwrap();
    let index_02 = indices[1].parse::<usize>().unwrap();
    (PasswordPolicy{ index_01, index_02, char: policy[1].as_bytes()[0] }, elements[1].trim().to_string())
}


fn is_valid_01(policy: &PasswordPolicy, password: &String) -> bool {
    let char_count = password.bytes()
        .filter(|char| policy.char == *char)
        .count()
    ;
    (policy.index_01 <= char_count) && (char_count <= policy.index_02)
}


fn is_valid_02(policy: &PasswordPolicy, password: &String) -> bool {
    let ascii = password.as_bytes();
    (ascii[policy.index_01 - 1] == policy.char) ^ (ascii[policy.index_02 - 1] == policy.char)
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Unable to read input.txt");

    let invalid_passwords_01 = contents
        .lines()
        .map(parse_password)
        .filter(|(policy, password)| { is_valid_01(policy, password) })
        .count();
    println!("{}", invalid_passwords_01);

    let invalid_passwords_02 = contents
        .lines()
        .map(parse_password)
        .filter(|(policy, password)| { is_valid_02(policy, password) })
        .count();
    println!("{}", invalid_passwords_02);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_password_01() {
        let input = "1-3 a: abcde";
        let result = parse_password(input);
        let (policy, password) = result;
        assert_eq!(policy.index_01, 1);
        assert_eq!(policy.index_02, 3);
        assert_eq!(policy.char, 'a');
        assert_eq!(password, "abcde");
    }
}
