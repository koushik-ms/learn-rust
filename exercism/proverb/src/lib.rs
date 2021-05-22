pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => "".to_string(),
        _ => bp(list)
    }
}

fn bp(list: &[&str]) -> String {
    let ans = (1..list.len())
        .map( |i| format!("For want of a {} the {} was lost.\n",list[i-1], list[i]))
        .collect::<Vec<_>>()
        .concat();
    format!("{}And all for the want of a {}.", ans, list[0])
}
