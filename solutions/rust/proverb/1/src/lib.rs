pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut proverb = String::new();
    for i in 0..list.len() - 1 {
        let line = format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
        proverb.push_str(&line);
    }
    let last_line = format!("And all for the want of a {}.", list[0]);
    proverb.push_str(&last_line);
    proverb
}