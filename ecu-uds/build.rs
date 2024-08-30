fn main() {
    let features = [
        std::env::var("CARGO_FEATURE_std2006").is_ok(),
        std::env::var("CARGO_FEATURE_std2013").is_ok(),
        std::env::var("CARGO_FEATURE_std2020").is_ok(),
    ];

    let crate_name = std::env::var("CARGO_PKG_NAME")
        .unwrap_or("ecu-uds".into());

    match features.iter()
        .filter(|&&en| en)
        .count() {
        1 => {},
        _ => panic!(
            "***`{}`*** at most one of the features `std2006` `std2013` or `std2016` can be enabled at a time.",
            crate_name
        )
    }
}