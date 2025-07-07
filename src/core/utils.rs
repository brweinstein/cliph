pub fn parse_range(input: &str) -> Option<(f64, f64)> {
    // Ex: "x=-5..5"
    if let Some(eq) = input.split('=').nth(1) {
        let parts: Vec<_> = eq.split("..").collect();
        if parts.len() == 2 {
            let a = parts[0].trim().parse().ok()?;
            let b = parts[1].trim().parse().ok()?;
            return Some((a, b));
        }
    }
    None
}
