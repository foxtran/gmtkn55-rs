pub fn unit_scale(from: &str, to: &str) -> f64 {
    let from_2_hartree = match from {
        "Hartree" => 1.0,
        "kcal/mol" => 1.0 / 627.5095,
        _ => panic!("Unknown unit from: {}", from),
    };
    let hartree_2_to = match to {
        "Hartree" => 1.0,
        "kcal/mol" => 627.5095,
        _ => panic!("Unknown unit from: {}", to),
    };
    from_2_hartree * hartree_2_to
}
