// Converts hiragana to _a enders
pub fn to_a(input: &str) -> String {
    match input {
        "う" => String::from("わ"),
        "く" => String::from("か"),
        "す" => String::from("さ"),
        "つ" => String::from("た"),
        "ぬ" => String::from("な"),
        "ふ" => String::from("は"),
        "む" => String::from("ま"),
        "ゆ" => String::from("や"),
        "る" => String::from("ら"),
        "ぐ" => String::from("が"),
        "ず" => String::from("ざ"),
        "ぶ" => String::from("ば"),
        "ぷ" => String::from("ぱ"),
        "づ" => String::from("だ"),
        _ => panic!("Invalid char: {}:", input),
    }
}

// Converts hiragana to _i enders
pub fn to_i(input: &str) -> String {
    match input {
        "う" => String::from("い"),
        "く" => String::from("き"),
        "す" => String::from("し"),
        "つ" => String::from("ち"),
        "ぬ" => String::from("に"),
        "ふ" => String::from("ひ"),
        "む" => String::from("み"),
        "ゆ" => String::from("い"),
        "る" => String::from("り"),
        "ぐ" => String::from("ぎ"),
        "ず" => String::from("じ"),
        "ぶ" => String::from("び"),
        "ぷ" => String::from("ぴ"),
        "づ" => String::from("ぢ"),
        _ => panic!("Invalid char: {}:", input),
    }
}

// Converts hiragana to _e enders
pub fn to_e(input: &str) -> String {
    match input {
        "う" => String::from("え"),
        "く" => String::from("け"),
        "す" => String::from("せ"),
        "つ" => String::from("て"),
        "ぬ" => String::from("ね"),
        "ふ" => String::from("へ"),
        "む" => String::from("め"),
        "ゆ" => String::from("え"),
        "る" => String::from("れ"),
        "ぐ" => String::from("げ"),
        "ず" => String::from("ぜ"),
        "ぶ" => String::from("べ"),
        "ぷ" => String::from("ぺ"),
        "づ" => String::from("で"),
        _ => panic!("Invalid char: {}:", input),
    }
}
