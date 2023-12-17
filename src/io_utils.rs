pub fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

pub fn get_user_input_trimmed() -> String {
    get_user_input().trim().to_string()
}
