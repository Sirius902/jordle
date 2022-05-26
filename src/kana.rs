#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Consonant {
    A,
    Ka,
    Sa,
    Ta,
    Na,
    Ha,
    Ma,
    Ya,
    Ra,
    Wa,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Vowel {
    A,
    I,
    U,
    E,
    O,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum CloseStatus {
    Consonant,
    Vowel,
    Close,
}

#[rustfmt::skip]
pub fn consonant(c: char) -> Option<Consonant> {
    match c {
        'ぁ' ..= 'お' => Some(Consonant::A),
        'か' ..= 'ご' => Some(Consonant::Ka),
        'さ' ..= 'ぞ' => Some(Consonant::Sa),
        'た' ..= 'ど' => Some(Consonant::Ta),
        'な' ..= 'の' => Some(Consonant::Na),
        'は' ..= 'ぽ' => Some(Consonant::Ha),
        'ま' ..= 'も' => Some(Consonant::Ma),
        'ゃ' ..= 'よ' => Some(Consonant::Ya),
        'ら' ..= 'ろ' => Some(Consonant::Ra),
        'わ' | 'を' => Some(Consonant::Wa),
        _ => None,
    }
}

#[rustfmt::skip]
pub fn vowel(c: char) -> Option<Vowel> {
    match c {
        'ぁ' | 'あ' | 'か' | 'さ' | 'た' | 'な' | 'は' | 'ま' | 'や' | 'ら' | 'わ' | 'が' | 'ざ' | 'だ' | 'ば' | 'ぱ' | 'ゃ' => Some(Vowel::A),
        'ぃ' | 'い' | 'き' | 'し' | 'ち' | 'に' | 'ひ' | 'み' | 'り' | 'ぎ' | 'じ' | 'ぢ' | 'び' | 'ぴ' => Some(Vowel::I),
        'ぅ' | 'う' | 'く' | 'す' | 'つ' | 'ぬ' | 'ふ' | 'む' | 'ゆ' | 'る' | 'ゔ' | 'ぐ' | 'ず' | 'づ' | 'っ' | 'ぶ' | 'ぷ' | 'ゅ' => Some(Vowel::U),
        'ぇ' | 'え' | 'け' | 'せ' | 'て' | 'ね' | 'へ' | 'め' | 'れ' | 'げ' | 'ぜ' | 'で' | 'べ' | 'ぺ' => Some(Vowel::E),
        'ぉ' | 'お' | 'こ' | 'そ' | 'と' | 'の' | 'ほ' | 'も' | 'よ' | 'ろ' | 'を' | 'ご' | 'ぞ' | 'ど' | 'ぼ' | 'ぽ' | 'ょ' => Some(Vowel::O),
        _ => None,
    }
}

#[rustfmt::skip]
pub fn close_status(c1: char, c2: char) -> Option<CloseStatus> {
    if let Some((cons1, cons2)) = consonant(c1).zip(consonant(c2)) {
        if let Some((vow1, vow2)) = vowel(c1).zip(vowel(c2)) {
            match (cons1 == cons2, vow1 == vow2) {
                (true, true) => return Some(CloseStatus::Close),
                (true, false) => return Some(CloseStatus::Consonant),
                (false, true) => return Some(CloseStatus::Vowel),
                _ => {},
            }
        }
    }

    let close_vu = |c: char| matches!(c, 'う' | 'ぅ');

    if c1 == 'ゔ' && close_vu(c2) || c2 == 'ゔ' && close_vu(c1) {
        Some(CloseStatus::Close)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::kana::CloseStatus::*;
    use crate::kana::*;

    #[test]
    fn close() {
        assert_eq!(close_status('ゔ', 'う'), Some(Close));
    }
}
