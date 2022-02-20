pub fn get_command_vec(command: &String) -> Vec<String> {
    let cmd_iter = command.split_whitespace();
    let vec = cmd_iter.map(|x| x.to_string()).collect::<Vec<String>>();
    return vec;
}
