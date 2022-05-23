#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Row {
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
pub enum Column {
    A,
    I,
    U,
    E,
    O,
}

#[rustfmt::skip]
pub fn row(c: char) -> Option<Row> {
    match c {
        'あ' | 'い' | 'う' | 'え' | 'お' | 'ぁ' | 'ぃ' | 'ぅ' | 'ぇ' | 'ぉ' => Some(Row::A),
        'か' | 'き' | 'く' | 'け' | 'こ' | 'が' | 'ぎ' | 'ぐ' | 'げ' | 'ご' => Some(Row::Ka),
        'さ' | 'し' | 'す' | 'せ' | 'そ' | 'ざ' | 'じ' | 'ず' | 'ぜ' | 'ぞ' => Some(Row::Sa),
        'た' | 'ち' | 'つ' | 'て' | 'と' | 'だ' | 'ぢ' | 'づ' | 'で' | 'ど' | 'っ' => Some(Row::Ta),
        'な' | 'に' | 'ぬ' | 'ね' | 'の' => Some(Row::Na),
        'は' | 'ひ' | 'ふ' | 'へ' | 'ほ' | 'ば' | 'び' | 'ぶ' | 'べ' | 'ぼ' | 'ぱ' | 'ぴ' | 'ぷ' | 'ぺ' | 'ぽ' => Some(Row::Ha),
        'ま' | 'み' | 'む' | 'め' | 'も' => Some(Row::Ma),
        'や' | 'ゆ' | 'よ' | 'ゃ' | 'ゅ' | 'ょ' => Some(Row::Ya),
        'ら' | 'り' | 'る' | 'れ' | 'ろ' => Some(Row::Ra),
        'わ' | 'を' => Some(Row::Wa),
        _ => None,
    }
}

#[rustfmt::skip]
pub fn column(c: char) -> Option<Column> {
    match c {
        'あ' | 'か' | 'さ' | 'た' | 'な' | 'は' | 'ま' | 'や' | 'ら' | 'わ' | 'ぁ' | 'が' | 'ざ' | 'だ' | 'ば' | 'ぱ' | 'ゃ' => Some(Column::A),
        'い' | 'き' | 'し' | 'ち' | 'に' | 'ひ' | 'み' | 'り' | 'ぃ' | 'ぎ' | 'じ' | 'ぢ' | 'び' | 'ぴ' => Some(Column::I),
        'う' | 'く' | 'す' | 'つ' | 'ぬ' | 'ふ' | 'む' | 'ゆ' | 'る' | 'ぅ' | 'ぐ' | 'ず' | 'づ' | 'っ' | 'ぶ' | 'ぷ' | 'ゅ' | 'ゔ' => Some(Column::U),
        'え' | 'け' | 'せ' | 'て' | 'ね' | 'へ' | 'め' | 'れ' | 'ぇ' | 'げ' | 'ぜ' | 'で' | 'べ' | 'ぺ' => Some(Column::E),
        'お' | 'こ' | 'そ' | 'と' | 'の' | 'ほ' | 'も' | 'よ' | 'ろ' | 'を' | 'ぉ' | 'ご' | 'ぞ' | 'ど' | 'ぼ' | 'ぽ' | 'ょ' => Some(Column::O),
        _ => None,
    }
}
