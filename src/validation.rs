pub fn is_snake_case(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() || c.is_ascii_digit() => (),
        _ => return false,
    }
    for c in chars {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return false;
        }
    }
    true
}

pub fn should_snake_case(s: &str) -> anyhow::Result<()> {
    if is_snake_case(s) {
        Ok(())
    } else {
        anyhow::bail!("'{}'はスネークケースで指定してください。", s);
    }
}
