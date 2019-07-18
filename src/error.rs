pub fn report(line: u32, message: &str) {
    eprintln!("[line {}] Error: {}", line, message);
}
