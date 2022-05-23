#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Row {
    A,
    I,
    U,
    E,
    O,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Column {
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

#[rustfmt::skip]
pub fn row(c: char) -> Option<Row> {
    match c {
        'あ' | 'か' | 'さ' | 'た' | 'な' | 'は' | 'ま' | 'や' | 'ら' | 'わ' | 'ぁ' | 'が' | 'ざ' | 'だ' | 'ば' | 'ぱ' | 'ゃ' => Some(Row::A),
        'い' | 'き' | 'し' | 'ち' | 'に' | 'ひ' | 'み' | 'り' | 'ぃ' | 'ぎ' | 'じ' | 'ぢ' | 'び' | 'ぴ' => Some(Row::I),
        'う' | 'く' | 'す' | 'つ' | 'ぬ' | 'ふ' | 'む' | 'ゆ' | 'る' | 'ぅ' | 'ぐ' | 'ず' | 'づ' | 'っ' | 'ぶ' | 'ぷ' | 'ゅ' | 'ゔ' => Some(Row::U),
        'え' | 'け' | 'せ' | 'て' | 'ね' | 'へ' | 'め' | 'れ' | 'ぇ' | 'げ' | 'ぜ' | 'で' | 'べ' | 'ぺ' => Some(Row::E),
        'お' | 'こ' | 'そ' | 'と' | 'の' | 'ほ' | 'も' | 'よ' | 'ろ' | 'を' | 'ぉ' | 'ご' | 'ぞ' | 'ど' | 'ぼ' | 'ぽ' | 'ょ' => Some(Row::O),
        _ => None,
    }
}

#[rustfmt::skip]
pub fn column(c: char) -> Option<Column> {
    match c {
        'あ' | 'い' | 'う' | 'え' | 'お' | 'ぁ' | 'ぃ' | 'ぅ' | 'ぇ' | 'ぉ' => Some(Column::A),
        'か' | 'き' | 'く' | 'け' | 'こ' | 'が' | 'ぎ' | 'ぐ' | 'げ' | 'ご' => Some(Column::Ka),
        'さ' | 'し' | 'す' | 'せ' | 'そ' | 'ざ' | 'じ' | 'ず' | 'ぜ' | 'ぞ' => Some(Column::Sa),
        'た' | 'ち' | 'つ' | 'て' | 'と' | 'だ' | 'ぢ' | 'づ' | 'で' | 'ど' | 'っ' => Some(Column::Ta),
        'な' | 'に' | 'ぬ' | 'ね' | 'の' => Some(Column::Na),
        'は' | 'ひ' | 'ふ' | 'へ' | 'ほ' | 'ば' | 'び' | 'ぶ' | 'べ' | 'ぼ' | 'ぱ' | 'ぴ' | 'ぷ' | 'ぺ' | 'ぽ' => Some(Column::Ha),
        'ま' | 'み' | 'む' | 'め' | 'も' => Some(Column::Ma),
        'や' | 'ゆ' | 'よ' | 'ゃ' | 'ゅ' | 'ょ' => Some(Column::Ya),
        'ら' | 'り' | 'る' | 'れ' | 'ろ' => Some(Column::Ra),
        'わ' | 'を' => Some(Column::Wa),
        _ => None,
    }
}
