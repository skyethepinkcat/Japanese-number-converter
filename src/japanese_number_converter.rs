//! # Converts a number into japanese
//! This converter takes a number in input and return a result of type `JapaneseNumber`.
//! This type has three properties:
//! - `arabiasu` : the number given asinput
//! - `kanji` : the number in kanji format
//! - `katakana` : the number in katakana format
//! - `romaji` : the number in roman format
//!
//! ## Example
//!
//! ```
//! let result = japanese_number_converter::
//!             JapaneseNumber::convert(100);
//! println!("Result : {}  =>  {}  =>  {}  =>  {}",
//!     result.arabiasuji(),result.kanji(),result.katakana(),result.romaji());
//!     // Result : 100  =>  百  =>  ヒャク   =>  hyaku
//! ```
//!
pub struct JapaneseNumber {
    arabiasuji: usize,
    kanji: String,
    romaji: String,
    katakana: String,
}

impl JapaneseNumber {
    ///  # Example
    ///
    /// ```
    /// let result = japanese_number_converter::
    ///             JapaneseNumber::convert(100);
    /// println!("Result : {}  =>  {}  =>  {}  =>  {}",
    ///     result.arabiasuji(),result.kanji(),result.katakana(),result.romaji());
    ///     // Result : 100  =>  百  =>  ヒャク   =>  hyaku
    /// ```
    ///
    /// # Panics
    ///
    /// The lib support conversion of numbers from 0 upt to 1844_6744_0737_0955_1615.
    /// It basically convert a number which can be contained in a `usize` variable.
    /// When the value cannot be contained in a `usize` variable, it will raise an error.
    /// ## Example :
    /// ```
    ///     let result = japanese_number_converter::
    ///         JapaneseNumber::convert(1844_6744_0737_0955_1616);
    ///     println!("Result : {}  =>  {}  =>  {}  =>  {}",
    ///     result.arabiasuji(),result.kanji(),result.katakana(),result.romaji());
    ///     // error: literal out of range for `usize`
    ///     // note: the literal `1844_6744_0737_0955_1616` does not fit into the type `usize`
    ///     //      whose range is `0..=18446744073709551615`
    /// ```
    ///
    pub fn convert(number: usize) -> JapaneseNumber {
        over_10k(number)
    }

    /// Get the arabic number
    pub fn arabiasuji(&self) -> usize {
        self.arabiasuji
    }
    /// Get the kanji version of the converted number
    pub fn kanji(&self) -> &String {
        &self.kanji
    }
    /// Get the romaji version of the converted number
    pub fn romaji(&self) -> &String {
        &self.romaji
    }
    /// Get the katakana version of the converted number
    pub fn katakana(&self) -> &String {
        &self.katakana
    }
}
const PACKS_ROMAJI: [&str; 4] = ["man", "oku", "chō", "kei"];
const PACKS_KANJI: [&str; 4] = ["万", "億", "兆", "京"];
const PACKS_KATAKANA: [&str; 4] = ["マン", "オク", "チョオ", "ケイ"];

const UNITS_ROMAJI: [&str; 11] = [
    "rei", "ichi", "ni", "san", "yon", "go", "roku", "nana", "hachi", "kyū", "jū",
];
const UNITS_KATAKANA: [&str; 11] = [
    "レ",
    "イチ",
    "ニ",
    "サン",
    "よん",
    "ゴ",
    "ロク",
    "ナナ",
    "ハチ",
    "キュウ",
    "ジュウ",
];
const UNITS_KANJI: [&str; 11] = [
    "零", "一", "ニ", "三", "四", "五", "六", "七", "八", "九", "十",
];

/// Convert numbers less than 11
fn less_than_11(number: usize) -> JapaneseNumber {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();
    if number <= 10 {
        result_romaji += UNITS_ROMAJI[number];
        result_kanji += UNITS_KANJI[number];
        result_katakana += UNITS_KATAKANA[number];
    }

    JapaneseNumber {
        arabiasuji: number,
        kanji: result_kanji,
        romaji: result_romaji,
        katakana: result_katakana,
    }
}

fn less_than_100(number: usize) -> JapaneseNumber {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();

    // Decimals
    if number < 100 {
        let decimal = number / 10;

        // Prevent ichi jū
        if decimal > 0 {
            if decimal > 1 {
                result_romaji += UNITS_ROMAJI[decimal];
                result_romaji += " ";

                result_kanji += UNITS_KANJI[decimal];

                result_katakana += UNITS_KATAKANA[decimal];
                result_katakana += " ";
            }

            result_romaji += UNITS_ROMAJI[10];
            result_romaji += " ";

            result_kanji += UNITS_KANJI[10];
            result_romaji += " ";

            result_katakana += UNITS_KATAKANA[10];
            result_katakana += " ";
        }
        // Units
        let left = number - decimal * 10;
        if left > 0 {
            let res = less_than_11(left);
            result_kanji += &res.kanji;
            result_katakana += &res.katakana;
            result_romaji += &res.romaji;
        }
    }

    JapaneseNumber {
        arabiasuji: number,
        kanji: result_kanji,
        romaji: result_romaji,
        katakana: result_katakana,
    }
}

fn less_than_1000(number: usize) -> JapaneseNumber {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();

    if number < 1000 {
        let hundred = number / 100;
        // Prevent "hyaku jū" when got "10" as input
        if hundred > 0 {
            // Prevent "ichi hyaku"
            if hundred > 1 {
                result_romaji += UNITS_ROMAJI[hundred];
                result_romaji += " ";

                result_kanji += UNITS_KANJI[hundred];

                result_katakana += UNITS_KATAKANA[hundred];
                result_katakana += " ";
            }

            result_romaji += "hyaku";
            result_romaji += " ";

            result_kanji += "百";
            result_romaji += " ";

            result_katakana += "ヒャク";
            result_katakana += " ";
        }
        let left = number - hundred * 100;
        if left > 0 {
            let res = less_than_100(number % 100);
            result_kanji += &res.kanji;
            result_katakana += &res.katakana;
            result_romaji += &res.romaji;
        }
    }

    JapaneseNumber {
        arabiasuji: number,
        kanji: result_kanji,
        romaji: result_romaji,
        katakana: result_katakana,
    }
}

fn less_than_10k(number: usize) -> JapaneseNumber {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();

    if number < 10000 {
        let thousand = number / 1000;
        // Prevent "sen jū" when got "10" as input
        if thousand > 0 {
            // Prevent "ichi sen"
            if thousand > 1 {
                result_romaji += UNITS_ROMAJI[thousand];
                // result_romaji += " ";

                result_kanji += UNITS_KANJI[thousand];

                result_katakana += UNITS_KATAKANA[thousand];
                result_katakana += " ";
            }

            result_romaji += "sen";
            result_romaji += " ";

            result_kanji += "千";
            result_romaji += " ";

            result_katakana += "セン";
            result_katakana += " ";
        }

        let left = number - thousand * 1000;
        if left > 0 {
            let res = less_than_1000(number % 1000);
            result_kanji += &res.kanji;
            result_katakana += &res.katakana;
            result_romaji += &res.romaji;
        }
    }
    JapaneseNumber {
        arabiasuji: number,
        kanji: result_kanji,
        romaji: result_romaji,
        katakana: result_katakana,
    }
}

fn over_10k(number: usize) -> JapaneseNumber {
    let mut result_katakana: Vec<String> = Vec::new();
    let mut result_romaji: Vec<String> = Vec::new();
    let mut result_kanji: Vec<String> = Vec::new();

    if number == 0 {
        return JapaneseNumber {
            arabiasuji: number,
            kanji: UNITS_KANJI[0].to_owned(),
            romaji: UNITS_ROMAJI[0].to_owned(),
            katakana: UNITS_KATAKANA[0].to_owned(),
        };
    }

    let mut num = number;
    let mut sep = 0;

    while num > 0 {
        let result = less_than_10k(num % 10000);
        if sep > 0 {
            if num % 10000 > 0 {
                result_romaji.push(result.romaji + PACKS_ROMAJI[sep - 1]);
                result_katakana.push(result.katakana + PACKS_KATAKANA[sep - 1]);
                result_kanji.push(result.kanji + PACKS_KANJI[sep - 1]);
            }
        } else {
            result_romaji.push(result.romaji);
            result_katakana.push(result.katakana);
            result_kanji.push(result.kanji);
        }
        num = num / 10000;
        sep += 1;
    }

    result_kanji.reverse();
    result_katakana.reverse();
    result_romaji.reverse();

    JapaneseNumber {
        arabiasuji: number,
        kanji: result_kanji.join(""),
        romaji: result_romaji.join(" "),
        katakana: result_katakana.join(" "),
    }
}
