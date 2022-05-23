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

const A_ROW: &str = "あかさたなはまやらわぁがざだばぱゃ";
const I_ROW: &str = "いきしちにひみりぃぎじぢびぴ";
const U_ROW: &str = "うくすつぬふむゆるぅぐずづっぶぷゅゔ";
const E_ROW: &str = "えけせてねへめれぇげぜでべぺ";
const O_ROW: &str = "おこそとのほもよろをぉごぞどぼぽょ";

const A_COLUMN: &str = "あいうえおぁぃぅぇぉ";
const KA_COLUMN: &str = "かきくけこがぎぐげご";
const SA_COLUMN: &str = "さしすせそざじずぜぞ";
const TA_COLUMN: &str = "たちつてとだぢづでどっ";
const NA_COLUMN: &str = "なにぬねの";
const HA_COLUMN: &str = "はひふへほばびぶべぼぱぴぷぺぽ";
const MA_COLUMN: &str = "まみむめも";
const YA_COLUMN: &str = "やゆよゃゅょ";
const RA_COLUMN: &str = "らりるれろ";
const WA_COLUMN: &str = "わを";

const ROWS: &[(&str, Row)] = &[
    (A_ROW, Row::A),
    (I_ROW, Row::I),
    (U_ROW, Row::U),
    (E_ROW, Row::E),
    (O_ROW, Row::O),
];

const COLUMNS: &[(&str, Column)] = &[
    (A_COLUMN, Column::A),
    (KA_COLUMN, Column::Ka),
    (SA_COLUMN, Column::Sa),
    (TA_COLUMN, Column::Ta),
    (NA_COLUMN, Column::Na),
    (HA_COLUMN, Column::Ha),
    (MA_COLUMN, Column::Ma),
    (YA_COLUMN, Column::Ya),
    (RA_COLUMN, Column::Ra),
    (WA_COLUMN, Column::Wa),
];

pub fn row(c: char) -> Option<Row> {
    ROWS.iter().find(|(s, _)| s.contains(c)).map(|(_, r)| *r)
}

pub fn column(c: char) -> Option<Column> {
    COLUMNS
        .iter()
        .find(|(s, _)| s.contains(c))
        .map(|(_, col)| *col)
}
