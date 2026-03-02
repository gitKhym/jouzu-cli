// Converts hiragana to _a enders
pub fn to_a(input: &char) -> char {
    match input {
        'う' => 'わ',
        'く' => 'か',
        'す' => 'さ',
        'つ' => 'た',
        'ぬ' => 'な',
        'ふ' => 'は',
        'む' => 'ま',
        'ゆ' => 'や',
        'る' => 'ら',
        'ぐ' => 'が',
        'ず' => 'ざ',
        'ぶ' => 'ば',
        'ぷ' => 'ぱ',
        'づ' => 'だ',
        _ => panic!("Invalid char: {}:", input),
    }
}

// Converts hiragana to _i enders
pub fn to_i(input: &char) -> char {
    match input {
        'う' => 'い',
        'く' => 'き',
        'す' => 'し',
        'つ' => 'ち',
        'ぬ' => 'に',
        'ふ' => 'ひ',
        'む' => 'み',
        'ゆ' => 'い',
        'る' => 'り',
        'ぐ' => 'ぎ',
        'ず' => 'じ',
        'ぶ' => 'び',
        'ぷ' => 'ぴ',
        'づ' => 'ぢ',
        _ => panic!("Invalid char: {}:", input),
    }
}

// Converts hiragana to _e enders
pub fn to_e(input: &char) -> char {
    match input {
        'う' => 'え',
        'く' => 'け',
        'す' => 'せ',
        'つ' => 'て',
        'ぬ' => 'ね',
        'ふ' => 'へ',
        'む' => 'め',
        'ゆ' => 'え',
        'る' => 'れ',
        'ぐ' => 'げ',
        'ず' => 'ぜ',
        'ぶ' => 'べ',
        'ぷ' => 'ぺ',
        'づ' => 'で',
        _ => panic!("Invalid char: {}:", input),
    }
}

pub fn is_voiced(s: &str) -> bool {
    if let Some(last_char) = s.chars().last() {
        let u_row = ['う', 'く', 'ぐ', 'す', 'つ', 'ぬ', 'ぶ', 'む', 'る'];
        if u_row.contains(&last_char) {
            return matches!(last_char, 'ぐ' | 'ぶ' | 'ぬ');
        }
    }
    false
}

pub fn last(input: &str) -> char {
    input.chars().last().unwrap()
}
