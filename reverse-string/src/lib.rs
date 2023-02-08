pub fn reverse(input: &str) -> String {
    let mut inpt: Vec<String> = input.chars().map(|x|x.to_string()).collect();
    inpt.reverse();

    inpt.join("")
}
