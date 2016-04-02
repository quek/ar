pub fn table_name(str: &str) -> String {
    let mut str = str.to_lowercase();
    str.push('s');
    str
}
