use std::fmt;

#[derive(Debug, Clone)]
pub struct Route {
    pub name: String,
    pub grade: Grade,
    pub style: Vec<Style>,
    pub length: i32,
    pub pitches: i32,
    pub location: String,
}
impl Route {
    pub fn new(name: String, grade: Grade, style: Vec<Style>, length: i32, pitches: i32, location: String) -> Route {
        Route {
            name,
            grade,
            style,
            length,
            pitches,
            location,
        }
    }

    pub fn default() -> Route {
        Route {
            name: String::from("Unnamed Route"),
            grade: Grade::Yosemite(Yosemite::FiveNine),
            style: vec![Style::Sport],
            length: 0,
            pitches: 1,
            location: String::from("Unknown Location"),
        }
    }
}
impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}, {} ft, {} pitches, at {}", self.name, self.grade, self.length, self.pitches, self.location)
    }
}

#[derive(Debug, Clone)]
pub struct Send {
    pub route: Route,
    pub date: sea_orm::prelude::Date,
    pub partner: String,
    pub attempts: i32,
    pub s_type: SendType,
    pub notes: String,
}
impl std::fmt::Display for Send {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} on {} with {} on {}", self.s_type, self.route, self.partner, self.date)
    }
}

trait TallGradeSysConvert {
    fn to_yosemite(&self) -> Yosemite;
    fn to_french(&self) -> French;
    fn to_uiaa(&self) -> Uiaa;
}
trait BoulderGradeSysConvert {
    fn to_font(&self) -> Font;
    fn to_hueco(&self) -> Hueco;
}

#[derive(Debug, Clone)]
pub enum Style {
    Boulder,
    TopRope,
    Sport,
    Trad,
    Ice,
    Alpine,
    Aid,
    Speed,
    FreeSolo,
    DeepWater,
}
impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Style::Boulder => write!(f, "Boulder"),
            Style::TopRope => write!(f, "Top Rope"),
            Style::Sport => write!(f, "Sport"),
            Style::Trad => write!(f, "Trad"),
            Style::Ice => write!(f, "Ice"),
            Style::Alpine => write!(f, "Alpine"),
            Style::Aid => write!(f, "Aid"),
            Style::Speed => write!(f, "Speed"),
            Style::FreeSolo => write!(f, "Free Solo"),
            Style::DeepWater => write!(f, "Deep Water"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq,)]
pub enum TallGradeSys {
    Yosemite,
    French,
    Uiaa,
}
impl std::fmt::Display for TallGradeSys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TallGradeSys::French => write!(f, "French"),
            TallGradeSys::Uiaa => write!(f, "UIAA"),
            TallGradeSys::Yosemite => write!(f, "Yosemite"),
        }
    }
}
impl TallGradeSys {
    pub fn iter() -> impl Iterator<Item = TallGradeSys> {
        [
            TallGradeSys::Yosemite,
            TallGradeSys::French,
            TallGradeSys::Uiaa,
        ].iter().copied()
    }
}

#[derive(Debug, Copy, Clone, PartialEq,)]
pub enum BoulderGradeSys {
    Hueco,
    Font,
}
impl std::fmt::Display for BoulderGradeSys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoulderGradeSys::Font => write!(f, "Font"),
            BoulderGradeSys::Hueco => write!(f, "Hueco (V-Grade)"),
        }
    }
}
impl BoulderGradeSys {
    pub fn iter() -> impl Iterator<Item = BoulderGradeSys> {
        [
            BoulderGradeSys::Hueco,
            BoulderGradeSys::Font,
        ].iter().copied()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Grade {
    Yosemite(Yosemite), //5.7, 5.8, 5.9, etc
    Font(Font), //4-, 4, 4+, 5-, 5, 5+, then 6A-, 6A, 6A+, 6B-, 6B+, 6C-, 6C, 6C+, 7A-, etc.
    Hueco(Hueco), //VB, V0, V1, etc
    French(French), //4a, 4b, 4c, 5a, 5b, 5c, etc.
    Uiaa(Uiaa), //I, II, III, IV, V, VI, etc. with + and - as needed
    //add others as needed, but these are most common
}
impl std::fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grade::Yosemite(grade) => write!(f, "{grade}"),
            Grade::Font(grade) => write!(f, "{grade}"),
            Grade::Hueco(grade) => write!(f, "{grade}"),
            Grade::French(grade) => write!(f, "{grade}"),
            Grade::Uiaa(grade) => write!(f, "{grade}"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct FullGrade {
    pub yosemite: Yosemite,
    pub font: Font,
    pub hueco: Hueco,
    pub french: French,
    pub uiaa: Uiaa,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
pub enum Yosemite {
    One,
    Two,
    Three,
    Four,
    FiveZero,
    FiveOne,
    FiveTwo,
    FiveThree,
    FiveFour,
    FiveFive,
    FiveSix,
    FiveSeven,
    FiveEight,
    #[default]
    FiveNine,
    FiveTenA,
    FiveTenB,
    FiveTenC,
    FiveTenD,
    FiveElevenA,
    FiveElevenB,
    FiveElevenC,
    FiveElevenD,
    FiveTwelveA,
    FiveTwelveB,
    FiveTwelveC,
    FiveTwelveD,
    FiveThirteenA,
    FiveThirteenB,
    FiveThirteenC,
    FiveThirteenD,
    FiveFourteenA,
    FiveFourteenB,
    FiveFourteenC,
    FiveFourteenD,
    FiveFifteenA,
    FiveFifteenB,
    FiveFifteenC,
    FiveFifteenD,
    //add more as needed
    None,
}
impl std::fmt::Display for Yosemite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Yosemite::One => write!(f, "1"),
            Yosemite::Two => write!(f, "2"),
            Yosemite::Three => write!(f, "3"),
            Yosemite::Four => write!(f, "4"),
            Yosemite::FiveZero => write!(f, "5.0"),
            Yosemite::FiveOne => write!(f, "5.1"),
            Yosemite::FiveTwo => write!(f, "5.2"),
            Yosemite::FiveThree => write!(f, "5.3"),
            Yosemite::FiveFour => write!(f, "5.4"),
            Yosemite::FiveFive => write!(f, "5.5"),
            Yosemite::FiveSix => write!(f, "5.6"),
            Yosemite::FiveSeven => write!(f, "5.7"),
            Yosemite::FiveEight => write!(f, "5.8"),
            Yosemite::FiveNine => write!(f, "5.9"),
            Yosemite::FiveTenA => write!(f, "5.10a"),
            Yosemite::FiveTenB => write!(f, "5.10b"),
            Yosemite::FiveTenC => write!(f, "5.10c"),
            Yosemite::FiveTenD => write!(f, "5.10d"),
            Yosemite::FiveElevenA => write!(f, "5.11a"),
            Yosemite::FiveElevenB => write!(f, "5.11b"),
            Yosemite::FiveElevenC => write!(f, "5.11c"),
            Yosemite::FiveElevenD => write!(f, "5.11d"),
            Yosemite::FiveTwelveA => write!(f, "5.12a"),
            Yosemite::FiveTwelveB => write!(f, "5.12b"),
            Yosemite::FiveTwelveC => write!(f, "5.12c"),
            Yosemite::FiveTwelveD => write!(f, "5.12d"),
            Yosemite::FiveThirteenA => write!(f, "5.13a"),
            Yosemite::FiveThirteenB => write!(f, "5.13b"),
            Yosemite::FiveThirteenC => write!(f, "5.13c"),
            Yosemite::FiveThirteenD => write!(f, "5.13d"),
            Yosemite::FiveFourteenA => write!(f, "5.14a"),
            Yosemite::FiveFourteenB => write!(f, "5.14b"),
            Yosemite::FiveFourteenC => write!(f, "5.14c"),
            Yosemite::FiveFourteenD => write!(f, "5.14d"),
            Yosemite::FiveFifteenA => write!(f, "5.15a"),
            Yosemite::FiveFifteenB => write!(f, "5.15b"),
            Yosemite::FiveFifteenC => write!(f, "5.15c"),
            Yosemite::FiveFifteenD => write!(f, "5.15d"),
            Yosemite::None => write!(f, "None"),
        }
    }
}
impl std::convert::From<Hueco> for Yosemite { //Note, since this is between bouldering and big wall, it's not a solid conversion
    fn from(hueco: Hueco) -> Yosemite {
        match hueco {
            Hueco::Vb => Yosemite::FiveFive,
            Hueco::V0Minus => Yosemite::FiveEight,
            Hueco::V0 => Yosemite::FiveNine,
            Hueco::V0Plus => Yosemite::FiveTenA,
            Hueco::V1Minus => Yosemite::FiveTenB,
            Hueco::V1 => Yosemite::FiveTenC,
            Hueco::V1Plus => Yosemite::FiveTenD,
            Hueco::V2Minus => Yosemite::FiveElevenA,
            Hueco::V2 => Yosemite::FiveElevenA,
            Hueco::V2Plus => Yosemite::FiveElevenB,
            Hueco::V3Minus => Yosemite::FiveElevenC,
            Hueco::V3 => Yosemite::FiveElevenC,
            Hueco::V3Plus => Yosemite::FiveElevenD,
            Hueco::V4Minus => Yosemite::FiveTwelveA,
            Hueco::V4 => Yosemite::FiveTwelveA,
            Hueco::V4Plus => Yosemite::FiveTwelveB,
            Hueco::V5Minus => Yosemite::FiveTwelveB,
            Hueco::V5 => Yosemite::FiveTwelveB,
            Hueco::V5Plus => Yosemite::FiveTwelveB,
            Hueco::V6Minus => Yosemite::FiveTwelveC,
            Hueco::V6 => Yosemite::FiveTwelveD,
            Hueco::V6Plus => Yosemite::FiveTwelveD,
            Hueco::V7Minus => Yosemite::FiveThirteenA,
            Hueco::V7 => Yosemite::FiveThirteenA,
            Hueco::V7Plus => Yosemite::FiveThirteenB,
            Hueco::V8Minus => Yosemite::FiveThirteenB,
            Hueco::V8 => Yosemite::FiveThirteenC,
            Hueco::V8Plus => Yosemite::FiveThirteenC,
            Hueco::V9Minus => Yosemite::FiveThirteenC,
            Hueco::V9 => Yosemite::FiveThirteenD,
            Hueco::V9Plus => Yosemite::FiveThirteenD,
            Hueco::V10Minus => Yosemite::FiveFourteenA,
            Hueco::V10 => Yosemite::FiveFourteenA,
            Hueco::None => Yosemite::None,
            _ => Yosemite::FiveNine, //default to 5.9
        }
    }
}
impl std::convert::From<French> for Yosemite { //Note, conversions are always rough between systems
    fn from(french: French) -> Yosemite {
        match french {
            French::One => Yosemite::FiveTwo,
            French::Two => Yosemite::FiveThree,
            French::Three => Yosemite::FiveFour,
            French::FourA => Yosemite::FiveFive,
            French::FourB => Yosemite::FiveFive,
            French::FourC => Yosemite::FiveFive,
            French::FiveA => Yosemite::FiveSix,
            French::FiveB => Yosemite::FiveSeven,
            French::FiveC => Yosemite::FiveEight,
            French::SixAMinus => Yosemite::FiveNine,
            French::SixA => Yosemite::FiveTenA,
            French::SixAPlus => Yosemite::FiveTenB,
            French::SixBMinus => Yosemite::FiveTenC,
            French::SixB => Yosemite::FiveTenC,
            French::SixBPlus => Yosemite::FiveTenD,
            French::SixCMinus => Yosemite::FiveElevenA,
            French::SixC => Yosemite::FiveElevenA,
            French::SixCPlus => Yosemite::FiveElevenB,
            French::SevenAMinus => Yosemite::FiveElevenC,
            French::SevenA => Yosemite::FiveElevenC,
            French::SevenAPlus => Yosemite::FiveElevenD,
            French::SevenBMinus => Yosemite::FiveTwelveA,
            French::SevenB => Yosemite::FiveTwelveA,
            French::SevenBPlus => Yosemite::FiveTwelveB,
            French::SevenCMinus => Yosemite::FiveTwelveC,
            French::SevenC => Yosemite::FiveTwelveC,
            French::SevenCPlus => Yosemite::FiveTwelveD,
            French::EightAMinus => Yosemite::FiveThirteenA,
            French::EightA => Yosemite::FiveThirteenB,
            French::EightAPlus => Yosemite::FiveThirteenC,
            French::EightBMinus => Yosemite::FiveThirteenD,
            French::EightB => Yosemite::FiveThirteenD,
            French::EightBPlus => Yosemite::FiveFourteenA,
            French::EightCMinus => Yosemite::FiveFourteenB,
            French::EightC => Yosemite::FiveFourteenB,
            French::EightCPlus => Yosemite::FiveFourteenC,
            French::NineAMinus => Yosemite::FiveFourteenD,
            French::NineA => Yosemite::FiveFourteenD,
            French::NineAPlus => Yosemite::FiveFifteenA,
            French::NineBMinus => Yosemite::FiveFifteenB,
            French::NineB => Yosemite::FiveFifteenB,
            French::NineBPlus => Yosemite::FiveFifteenC,
            French::NineCMinus => Yosemite::FiveFifteenD,
            French::NineC => Yosemite::FiveFifteenD,
            French::NineCPlus => Yosemite::FiveFifteenD,
            _ => Yosemite::FiveNine, //default to 5.9
        }
    }
}
impl std::convert::From<Uiaa> for Yosemite {
    fn from(uiaa: Uiaa) -> Yosemite {
        match uiaa {
            Uiaa::I => Yosemite::FiveOne,
            Uiaa::Ii => Yosemite::FiveTwo,
            Uiaa::Iii => Yosemite::FiveThree,
            Uiaa::IvMinus => Yosemite::FiveFour,
            Uiaa::Iv => Yosemite::FiveFour,
            Uiaa::IvPlus => Yosemite::FiveFive,
            Uiaa::VMinus => Yosemite::FiveSix,
            Uiaa::V => Yosemite::FiveSeven,
            Uiaa::VPlus => Yosemite::FiveEight,
            Uiaa::ViMinus => Yosemite::FiveNine,
            Uiaa::Vi => Yosemite::FiveTenA,
            Uiaa::ViPlus => Yosemite::FiveTenB,
            Uiaa::ViiMinus => Yosemite::FiveTenC,
            Uiaa::Vii => Yosemite::FiveTenD,
            Uiaa::ViiPlus => Yosemite::FiveElevenA,
            Uiaa::ViiiMinus => Yosemite::FiveElevenC,
            Uiaa::Viii => Yosemite::FiveElevenD,
            Uiaa::ViiiPlus => Yosemite::FiveTwelveA,
            Uiaa::IxMinus => Yosemite::FiveTwelveC,
            Uiaa::Ix => Yosemite::FiveTwelveD,
            Uiaa::IxPlus => Yosemite::FiveThirteenB,
            Uiaa::XMinus => Yosemite::FiveThirteenC,
            Uiaa::X => Yosemite::FiveThirteenD,
            Uiaa::XPlus => Yosemite::FiveFourteenA,
            Uiaa::XiMinus => Yosemite::FiveFourteenB,
            Uiaa::Xi => Yosemite::FiveFourteenD,
            Uiaa::XiPlus => Yosemite::FiveFifteenA,
            Uiaa::XiiMinus => Yosemite::FiveFifteenB,
            Uiaa::Xii => Yosemite::FiveFifteenC,
            Uiaa::XiiPlus => Yosemite::FiveFifteenD,
            _ => Yosemite::FiveNine, //default to 5.9
        }
    }
}
impl std::convert::From<Font> for Yosemite { //Note, bouldering grades are not a direct conversion to big wall
    fn from(font: Font) -> Yosemite {
        match font {
            Font::OneMinus => Yosemite::FiveFour,
            Font::One => Yosemite::FiveFive,
            Font::OnePlus => Yosemite::FiveSix,
            Font::TwoMinus => Yosemite::FiveSix,
            Font::Two => Yosemite::FiveSeven,
            Font::TwoPlus => Yosemite::FiveEight,
            Font::ThreeMinus => Yosemite::FiveEight,
            Font::Three => Yosemite::FiveNine,
            Font::ThreePlus => Yosemite::FiveTenA,
            Font::FourMinus => Yosemite::FiveTenB,
            Font::Four => Yosemite::FiveTenC,
            Font::FourPlus => Yosemite::FiveTenD,
            Font::FiveMinus => Yosemite::FiveElevenA,
            Font::Five => Yosemite::FiveElevenA,
            Font::FivePlus => Yosemite::FiveElevenB,
            Font::SixAMinus => Yosemite::FiveElevenC,
            Font::SixA => Yosemite::FiveElevenC,
            Font::SixAPlus => Yosemite::FiveElevenD,
            Font::SixBMinus => Yosemite::FiveTwelveA,
            Font::SixB => Yosemite::FiveTwelveA,
            Font::SixBPlus => Yosemite::FiveTwelveB,
            Font::SixCMinus => Yosemite::FiveTwelveB,
            Font::SixC => Yosemite::FiveTwelveB,
            Font::SixCPlus => Yosemite::FiveTwelveB,
            Font::SevenAMinus => Yosemite::FiveTwelveC,
            Font::SevenA => Yosemite::FiveTwelveD,
            Font::SevenAPlus => Yosemite::FiveTwelveD,
            Font::SevenBMinus => Yosemite::FiveThirteenA,
            Font::SevenB => Yosemite::FiveThirteenA,
            Font::SevenBPlus => Yosemite::FiveThirteenB,
            Font::SevenCMinus => Yosemite::FiveThirteenB,
            Font::SevenC => Yosemite::FiveThirteenC,
            Font::SevenCPlus => Yosemite::FiveThirteenC,
            Font::EightAMinus => Yosemite::FiveThirteenC,
            Font::EightA => Yosemite::FiveThirteenD,
            Font::EightAPlus => Yosemite::FiveThirteenD,
            Font::EightBMinus => Yosemite::FiveFourteenA,
            Font::EightB => Yosemite::FiveFourteenA,
            Font::EightBPlus => Yosemite::FiveFourteenB,
            Font::EightCMinus => Yosemite::FiveFourteenB,
            Font::EightC => Yosemite::FiveFourteenC,
            Font::EightCPlus => Yosemite::FiveFourteenD,
            Font::NineAMinus => Yosemite::FiveFifteenA,
            Font::NineA => Yosemite::FiveFifteenB,
            Font::NineAPlus => Yosemite::FiveFifteenC,
            Font::NineBMinus => Yosemite::FiveFifteenD,
            Font::NineB => Yosemite::FiveFifteenD,
            Font::NineBPlus => Yosemite::FiveFifteenD,
            _ => Yosemite::FiveNine, //default to 5.9
        }
    }
}
impl std::convert::From<String> for Yosemite {
    fn from(grade: String) -> Yosemite {
        match grade.as_str() {
            "1" => Yosemite::One,
            "2" => Yosemite::Two,
            "3" => Yosemite::Three,
            "4" => Yosemite::Four,
            "5.1" => Yosemite::FiveOne,
            "5.2" => Yosemite::FiveTwo,
            "5.3" => Yosemite::FiveThree,
            "5.4" => Yosemite::FiveFour,
            "5.5" => Yosemite::FiveFive,
            "5.6" => Yosemite::FiveSix,
            "5.7" => Yosemite::FiveSeven,
            "5.8" => Yosemite::FiveEight,
            "5.9" => Yosemite::FiveNine,
            "5.10a" => Yosemite::FiveTenA,
            "5.10b" => Yosemite::FiveTenB,
            "5.10c" => Yosemite::FiveTenC,
            "5.10d" => Yosemite::FiveTenD,
            "5.11a" => Yosemite::FiveElevenA,
            "5.11b" => Yosemite::FiveElevenB,
            "5.11c" => Yosemite::FiveElevenC,
            "5.11d" => Yosemite::FiveElevenD,
            "5.12a" => Yosemite::FiveTwelveA,
            "5.12b" => Yosemite::FiveTwelveB,
            "5.12c" => Yosemite::FiveTwelveC,
            "5.12d" => Yosemite::FiveTwelveD,
            "5.13a" => Yosemite::FiveThirteenA,
            "5.13b" => Yosemite::FiveThirteenB,
            "5.13c" => Yosemite::FiveThirteenC,
            "5.13d" => Yosemite::FiveThirteenD,
            "5.14a" => Yosemite::FiveFourteenA,
            "5.14b" => Yosemite::FiveFourteenB,
            "5.14c" => Yosemite::FiveFourteenC,
            "5.14d" => Yosemite::FiveFourteenD,
            "5.15a" => Yosemite::FiveFifteenA,
            "5.15b" => Yosemite::FiveFifteenB,
            "5.15c" => Yosemite::FiveFifteenC,
            "5.15d" => Yosemite::FiveFifteenD,
            "None" => Yosemite::None,
            _ => Yosemite::FiveNine, //default to 5.9
        }
    }
}
impl std::convert::From<i32> for Yosemite {
    fn from(grade: i32) -> Yosemite {
        match grade {
            0 => Yosemite::One,
            1 => Yosemite::Two,
            2 => Yosemite::Three,
            3 => Yosemite::Four,
            4 => Yosemite::FiveZero,
            5 => Yosemite::FiveOne,
            6 => Yosemite::FiveTwo,
            7 => Yosemite::FiveThree,
            8 => Yosemite::FiveFour,
            9 => Yosemite::FiveFive,
            10 => Yosemite::FiveSix,
            11 => Yosemite::FiveSeven,
            12 => Yosemite::FiveEight,
            13 => Yosemite::FiveNine,
            14 => Yosemite::FiveTenA,
            15 => Yosemite::FiveTenB,
            16 => Yosemite::FiveTenC,
            17 => Yosemite::FiveTenD,
            18 => Yosemite::FiveElevenA,
            19 => Yosemite::FiveElevenB,
            20 => Yosemite::FiveElevenC,
            21 => Yosemite::FiveElevenD,
            22 => Yosemite::FiveTwelveA,
            23 => Yosemite::FiveTwelveB,
            24 => Yosemite::FiveTwelveC,
            25 => Yosemite::FiveTwelveD,
            26 => Yosemite::FiveThirteenA,
            27 => Yosemite::FiveThirteenB,
            28 => Yosemite::FiveThirteenC,
            29 => Yosemite::FiveThirteenD,
            30 => Yosemite::FiveFourteenA,
            31 => Yosemite::FiveFourteenB,
            32 => Yosemite::FiveFourteenC,
            33 => Yosemite::FiveFourteenD,
            34 => Yosemite::FiveFifteenA,
            35 => Yosemite::FiveFifteenB,
            36 => Yosemite::FiveFifteenC,
            37 => Yosemite::FiveFifteenD,
            _ => Yosemite::None, //default to 5.9
        }
    }
}
impl Yosemite {
    pub fn iter() -> impl Iterator<Item = Yosemite> {
        [
            Yosemite::One,
            Yosemite::Two,
            Yosemite::Three,
            Yosemite::Four,
            Yosemite::FiveZero,
            Yosemite::FiveOne,
            Yosemite::FiveTwo,
            Yosemite::FiveThree,
            Yosemite::FiveFour,
            Yosemite::FiveFive,
            Yosemite::FiveSix,
            Yosemite::FiveSeven,
            Yosemite::FiveEight,
            Yosemite::FiveNine,
            Yosemite::FiveTenA,
            Yosemite::FiveTenB,
            Yosemite::FiveTenC,
            Yosemite::FiveTenD,
            Yosemite::FiveElevenA,
            Yosemite::FiveElevenB,
            Yosemite::FiveElevenC,
            Yosemite::FiveElevenD,
            Yosemite::FiveTwelveA,
            Yosemite::FiveTwelveB,
            Yosemite::FiveTwelveC,
            Yosemite::FiveTwelveD,
            Yosemite::FiveThirteenA,
            Yosemite::FiveThirteenB,
            Yosemite::FiveThirteenC,
            Yosemite::FiveThirteenD,
            Yosemite::FiveFourteenA,
            Yosemite::FiveFourteenB,
            Yosemite::FiveFourteenC,
            Yosemite::FiveFourteenD,
            Yosemite::FiveFifteenA,
            Yosemite::FiveFifteenB,
            Yosemite::FiveFifteenC,
            Yosemite::FiveFifteenD,
        ].iter().copied()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
pub enum Font {
    OneMinus,
    One,
    OnePlus,
    TwoMinus,
    Two,
    TwoPlus,
    ThreeMinus,
    #[default]
    Three,
    ThreePlus,
    FourMinus,
    Four,
    FourPlus,
    FiveMinus,
    Five,
    FivePlus,
    SixAMinus,
    SixA,
    SixAPlus,
    SixBMinus,
    SixB,
    SixBPlus,
    SixCMinus,
    SixC,
    SixCPlus,
    SevenAMinus,
    SevenA,
    SevenAPlus,
    SevenBMinus,
    SevenB,
    SevenBPlus,
    SevenCMinus,
    SevenC,
    SevenCPlus,
    EightAMinus,
    EightA,
    EightAPlus,
    EightBMinus,
    EightB,
    EightBPlus,
    EightCMinus,
    EightC,
    EightCPlus,
    NineAMinus,
    NineA,
    NineAPlus,
    NineBMinus,
    NineB,
    NineBPlus,
    NineCMinus,
    NineC,
    NineCPlus,
    //add more as needed
}
impl std::fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Font::OneMinus => write!(f, "1-"),
            Font::One => write!(f, "1"),
            Font::OnePlus => write!(f, "1+"),
            Font::TwoMinus => write!(f, "2-"),
            Font::Two => write!(f, "2"),
            Font::TwoPlus => write!(f, "2+"),
            Font::ThreeMinus => write!(f, "3-"),
            Font::Three => write!(f, "3"),
            Font::ThreePlus => write!(f, "3+"),
            Font::FourMinus => write!(f, "4-"),
            Font::Four => write!(f, "4"),
            Font::FourPlus => write!(f, "4+"),
            Font::FiveMinus => write!(f, "5-"),
            Font::Five => write!(f, "5"),
            Font::FivePlus => write!(f, "5+"),
            Font::SixAMinus => write!(f, "6A-"),
            Font::SixA => write!(f, "6A"),
            Font::SixAPlus => write!(f, "6A+"),
            Font::SixBMinus => write!(f, "6B-"),
            Font::SixB => write!(f, "6B"),
            Font::SixBPlus => write!(f, "6B+"),
            Font::SixCMinus => write!(f, "6C-"),
            Font::SixC => write!(f, "6C"),
            Font::SixCPlus => write!(f, "6C+"),
            Font::SevenAMinus => write!(f, "7A-"),
            Font::SevenA => write!(f, "7A"),
            Font::SevenAPlus => write!(f, "7A+"),
            Font::SevenBMinus => write!(f, "7B-"),
            Font::SevenB => write!(f, "7B"),
            Font::SevenBPlus => write!(f, "7B+"),
            Font::SevenCMinus => write!(f, "7C-"),
            Font::SevenC => write!(f, "7C"),
            Font::SevenCPlus => write!(f, "7C+"),
            Font::EightAMinus => write!(f, "8A-"),
            Font::EightA => write!(f, "8A"),
            Font::EightAPlus => write!(f, "8A+"),
            Font::EightBMinus => write!(f, "8B-"),
            Font::EightB => write!(f, "8B"),
            Font::EightBPlus => write!(f, "8B+"),
            Font::EightCMinus => write!(f, "8C-"),
            Font::EightC => write!(f, "8C"),
            Font::EightCPlus => write!(f, "8C+"),
            Font::NineAMinus => write!(f, "9A-"),
            Font::NineA => write!(f, "9A"),
            Font::NineAPlus => write!(f, "9A+"),
            Font::NineBMinus => write!(f, "9B-"),
            Font::NineB => write!(f, "9B"),
            Font::NineBPlus => write!(f, "9B+"),
            Font::NineCMinus => write!(f, "9C-"),
            Font::NineC => write!(f, "9C"),
            Font::NineCPlus => write!(f, "9C+"),
        }
    }
}
impl std::convert::From<Hueco> for Font {
    fn from(hueco: Hueco) -> Font {
        match hueco {
            Hueco::Vb => Font::Five,
            Hueco::V0Minus => Font::FivePlus,
            Hueco::V0 => Font::SixAMinus,
            Hueco::V0Plus => Font::SixA,
            Hueco::V1Minus => Font::SixAPlus,
            Hueco::V1 => Font::SixBMinus,
            Hueco::V1Plus => Font::SixB,
            Hueco::V2Minus => Font::SixBPlus,
            Hueco::V2 => Font::SixCMinus,
            Hueco::V2Plus => Font::SixC,
            Hueco::V3Minus => Font::SixCPlus,
            Hueco::V3 => Font::SevenAMinus,
            Hueco::V3Plus => Font::SevenA,
            Hueco::V4Minus => Font::SevenAPlus,
            Hueco::V4 => Font::SevenBMinus,
            Hueco::V4Plus => Font::SevenB,
            Hueco::V5Minus => Font::SevenBPlus,
            Hueco::V5 => Font::SevenCMinus,
            Hueco::V5Plus => Font::SevenC,
            Hueco::V6Minus => Font::SevenCPlus,
            Hueco::V6 => Font::EightAMinus,
            Hueco::V6Plus => Font::EightA,
            Hueco::V7Minus => Font::EightAPlus,
            Hueco::V7 => Font::EightBMinus,
            Hueco::V7Plus => Font::EightB,
            Hueco::V8Minus => Font::EightBPlus,
            Hueco::V8 => Font::EightCMinus,
            Hueco::V8Plus => Font::EightC,
            Hueco::V9Minus => Font::EightCPlus,
            Hueco::V9 => Font::NineAMinus,
            Hueco::V9Plus => Font::NineA,
            Hueco::V10Minus => Font::NineAPlus,
            Hueco::V10 => Font::NineBMinus,
            Hueco::None => Font::NineB,
            _ => Font::Five, //default to 5
        }
    }
}
impl Font {
    pub fn iter() -> impl Iterator<Item = Font> {
        [
            Font::OneMinus,
            Font::One,
            Font::OnePlus,
            Font::TwoMinus,
            Font::Two,
            Font::TwoPlus,
            Font::ThreeMinus,
            Font::Three,
            Font::ThreePlus,
            Font::FourMinus,
            Font::Four,
            Font::FourPlus,
            Font::FiveMinus,
            Font::Five,
            Font::FivePlus,
            Font::SixAMinus,
            Font::SixA,
            Font::SixAPlus,
            Font::SixBMinus,
            Font::SixB,
            Font::SixBPlus,
            Font::SixCMinus,
            Font::SixC,
            Font::SixCPlus,
            Font::SevenAMinus,
            Font::SevenA,
            Font::SevenAPlus,
            Font::SevenBMinus,
            Font::SevenB,
            Font::SevenBPlus,
            Font::SevenCMinus,
            Font::SevenC,
            Font::SevenCPlus,
            Font::EightAMinus,
            Font::EightA,
            Font::EightAPlus,
            Font::EightBMinus,
            Font::EightB,
            Font::EightBPlus,
            Font::EightCMinus,
            Font::EightC,
            Font::EightCPlus,
            Font::NineAMinus,
            Font::NineA,
            Font::NineAPlus,
            Font::NineBMinus,
            Font::NineB,
            Font::NineBPlus,
            Font::NineCMinus,
            Font::NineC,
            Font::NineCPlus,
        ].iter().copied()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
pub enum Hueco { //V Scale
    Vb,
    V0Minus,
    #[default]
    V0,
    V0Plus,
    V1Minus,
    V1,
    V1Plus,
    V2Minus,
    V2,
    V2Plus,
    V3Minus,
    V3,
    V3Plus,
    V4Minus,
    V4,
    V4Plus,
    V5Minus,
    V5,
    V5Plus,
    V6Minus,
    V6,
    V6Plus,
    V7Minus,
    V7,
    V7Plus,
    V8Minus,
    V8,
    V8Plus,
    V9Minus,
    V9,
    V9Plus,
    V10Minus,
    V10,
    V10Plus,
    V11Minus,
    V11,
    V11Plus,
    V12Minus,
    V12,
    V12Plus,
    V13Minus,
    V13,
    V13Plus,
    V14Minus,
    V14,
    V14Plus,
    V15Minus,
    V15,
    V15Plus,
    V16Minus,
    V16,
    V16Plus,
    V17Minus,
    V17,
    V17Plus,
    //add more as needed
    None,
}
impl std::fmt::Display for Hueco {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hueco::Vb => write!(f, "VB"),
            Hueco::V0Minus => write!(f, "V0-"),
            Hueco::V0 => write!(f, "V0"),
            Hueco::V0Plus => write!(f, "V0+"),
            Hueco::V1Minus => write!(f, "V1-"),
            Hueco::V1 => write!(f, "V1"),
            Hueco::V1Plus => write!(f, "V1+"),
            Hueco::V2Minus => write!(f, "V2-"),
            Hueco::V2 => write!(f, "V2"),
            Hueco::V2Plus => write!(f, "V2+"),
            Hueco::V3Minus => write!(f, "V3-"),
            Hueco::V3 => write!(f, "V3"),
            Hueco::V3Plus => write!(f, "V3+"),
            Hueco::V4Minus => write!(f, "V4-"),
            Hueco::V4 => write!(f, "V4"),
            Hueco::V4Plus => write!(f, "V4+"),
            Hueco::V5Minus => write!(f, "V5-"),
            Hueco::V5 => write!(f, "V5"),
            Hueco::V5Plus => write!(f, "V5+"),
            Hueco::V6Minus => write!(f, "V6-"),
            Hueco::V6 => write!(f, "V6"),
            Hueco::V6Plus => write!(f, "V6+"),
            Hueco::V7Minus => write!(f, "V7-"),
            Hueco::V7 => write!(f, "V7"),
            Hueco::V7Plus => write!(f, "V7+"),
            Hueco::V8Minus => write!(f, "V8-"),
            Hueco::V8 => write!(f, "V8"),
            Hueco::V8Plus => write!(f, "V8+"),
            Hueco::V9Minus => write!(f, "V9-"),
            Hueco::V9 => write!(f, "V9"),
            Hueco::V9Plus => write!(f, "V9+"),
            Hueco::V10Minus => write!(f, "V10-"),
            Hueco::V10 => write!(f, "V10"),
            Hueco::V10Plus => write!(f, "V10+"),
            Hueco::V11Minus => write!(f, "V11-"),
            Hueco::V11 => write!(f, "V11"),
            Hueco::V11Plus => write!(f, "V11+"),
            Hueco::V12Minus => write!(f, "V12-"),
            Hueco::V12 => write!(f, "V12"),
            Hueco::V12Plus => write!(f, "V12+"),
            Hueco::V13Minus => write!(f, "V13-"),
            Hueco::V13 => write!(f, "V13"),
            Hueco::V13Plus => write!(f, "V13+"),
            Hueco::V14Minus => write!(f, "V14-"),
            Hueco::V14 => write!(f, "V14"),
            Hueco::V14Plus => write!(f, "V14+"),
            Hueco::V15Minus => write!(f, "V15-"),
            Hueco::V15 => write!(f, "V15"),
            Hueco::V15Plus => write!(f, "V15+"),
            Hueco::V16Minus => write!(f, "V16-"),
            Hueco::V16 => write!(f, "V16"),
            Hueco::V16Plus => write!(f, "V16+"),
            Hueco::V17Minus => write!(f, "V17-"),
            Hueco::V17 => write!(f, "V17"),
            Hueco::V17Plus => write!(f, "V17+"),
            Hueco::None => write!(f, "None"),
        }
    }
}
impl std::convert::From<i32> for Hueco {
    fn from(grade: i32) -> Hueco {
        match grade {
            -1 => Hueco::Vb,
            0 => Hueco::V0Minus,
            1 => Hueco::V0,
            2 => Hueco::V0Plus,
            3 => Hueco::V1Minus,
            4 => Hueco::V1,
            5 => Hueco::V1Plus,
            6 => Hueco::V2Minus,
            7 => Hueco::V2,
            8 => Hueco::V2Plus,
            9 => Hueco::V3Minus,
            10 => Hueco::V3,
            11 => Hueco::V3Plus,
            12 => Hueco::V4Minus,
            13 => Hueco::V4,
            14 => Hueco::V4Plus,
            15 => Hueco::V5Minus,
            16 => Hueco::V5,
            17 => Hueco::V5Plus,
            18 => Hueco::V6Minus,
            19 => Hueco::V6,
            20 => Hueco::V6Plus,
            21 => Hueco::V7Minus,
            22 => Hueco::V7,
            23 => Hueco::V7Plus,
            24 => Hueco::V8Minus,
            25 => Hueco::V8,
            26 => Hueco::V8Plus,
            27 => Hueco::V9Minus,
            28 => Hueco::V9,
            29 => Hueco::V9Plus,
            30 => Hueco::V10Minus,
            31 => Hueco::V10,
            32 => Hueco::V10Plus,
            33 => Hueco::V11Minus,
            34 => Hueco::V11,
            35 => Hueco::V11Plus,
            36 => Hueco::V12Minus,
            37 => Hueco::V12,
            38 => Hueco::V12Plus,
            39 => Hueco::V13Minus,
            40 => Hueco::V13,
            41 => Hueco::V13Plus,
            42 => Hueco::V14Minus,
            43 => Hueco::V14,
            44 => Hueco::V14Plus,
            45 => Hueco::V15Minus,
            46 => Hueco::V15,
            47 => Hueco::V15Plus,
            48 => Hueco::V16Minus,
            49 => Hueco::V16,
            50 => Hueco::V16Plus,
            51 => Hueco::V17Minus,
            52 => Hueco::V17,
            53 => Hueco::V17Plus,
            _ => Hueco::None, //default to no grade
        }
    }

}
impl std::convert::From<String> for Hueco {
    fn from(grade: String) -> Hueco {
        match grade.as_str() {
            "VB" => Hueco::Vb,
            "V0-" => Hueco::V0Minus,
            "V0" => Hueco::V0,
            "V0+" => Hueco::V0Plus,
            "V1-" => Hueco::V1Minus,
            "V1" => Hueco::V1,
            "V1+" => Hueco::V1Plus,
            "V2-" => Hueco::V2Minus,
            "V2" => Hueco::V2,
            "V2+" => Hueco::V2Plus,
            "V3-" => Hueco::V3Minus,
            "V3" => Hueco::V3,
            "V3+" => Hueco::V3Plus,
            "V4-" => Hueco::V4Minus,
            "V4" => Hueco::V4,
            "V4+" => Hueco::V4Plus,
            "V5-" => Hueco::V5Minus,
            "V5" => Hueco::V5,
            "V5+" => Hueco::V5Plus,
            "V6-" => Hueco::V6Minus,
            "V6" => Hueco::V6,
            "V6+" => Hueco::V6Plus,
            "V7-" => Hueco::V7Minus,
            "V7" => Hueco::V7,
            "V7+" => Hueco::V7Plus,
            "V8-" => Hueco::V8Minus,
            "V8" => Hueco::V8,
            "V8+" => Hueco::V8Plus,
            "V9-" => Hueco::V9Minus,
            "V9" => Hueco::V9,
            "V9+" => Hueco::V9Plus,
            "V10-" => Hueco::V10Minus,
            "V10" => Hueco::V10,
            "V10+" => Hueco::V10Plus,
            "V11-" => Hueco::V11Minus,
            "V11" => Hueco::V11,
            "V11+" => Hueco::V11Plus,
            "V12-" => Hueco::V12Minus,
            "V12" => Hueco::V12,
            "V12+" => Hueco::V12Plus,
            "V13-" => Hueco::V13Minus,
            "V13" => Hueco::V13,
            "V13+" => Hueco::V13Plus,
            "V14-" => Hueco::V14Minus,
            "V14" => Hueco::V14,
            "V14+" => Hueco::V14Plus,
            "V15-" => Hueco::V15Minus,
            "V15" => Hueco::V15,
            "V15+" => Hueco::V15Plus,
            "V16-" => Hueco::V16Minus,
            "V16" => Hueco::V16,
            "V16+" => Hueco::V16Plus,
            "V17-" => Hueco::V17Minus,
            "V17" => Hueco::V17,
            "V17+" => Hueco::V17Plus,
            "None" => Hueco::None,
            _ => Hueco::V0, //default to V0
        }
    }
}
impl std::convert::From<Font> for Hueco {
    fn from(font: Font) -> Hueco {
        match font {
            Font::OneMinus => Hueco::V0Minus,
            Font::One => Hueco::V0,
            Font::OnePlus => Hueco::V0Plus,
            Font::TwoMinus => Hueco::V1Minus,
            Font::Two => Hueco::V1,
            Font::TwoPlus => Hueco::V1Plus,
            Font::ThreeMinus => Hueco::V2Minus,
            Font::Three => Hueco::V2,
            Font::ThreePlus => Hueco::V2Plus,
            Font::FourMinus => Hueco::V3Minus,
            Font::Four => Hueco::V3,
            Font::FourPlus => Hueco::V3Plus,
            Font::FiveMinus => Hueco::V4Minus,
            Font::Five => Hueco::V4,
            Font::FivePlus => Hueco::V4Plus,
            Font::SixAMinus => Hueco::V5Minus,
            Font::SixA => Hueco::V5,
            Font::SixAPlus => Hueco::V5Plus,
            Font::SixBMinus => Hueco::V6Minus,
            Font::SixB => Hueco::V6,
            Font::SixBPlus => Hueco::V6Plus,
            Font::SixCMinus => Hueco::V7Minus,
            Font::SixC => Hueco::V7,
            Font::SixCPlus => Hueco::V7Plus,
            Font::SevenAMinus => Hueco::V8Minus,
            Font::SevenA => Hueco::V8,
            Font::SevenAPlus => Hueco::V8Plus,
            Font::SevenBMinus => Hueco::V9Minus,
            Font::SevenB => Hueco::V9,
            Font::SevenBPlus => Hueco::V9Plus,
            Font::SevenCMinus => Hueco::V10Minus,
            Font::SevenC => Hueco::V10,
            Font::SevenCPlus => Hueco::V10Plus,
            Font::EightAMinus => Hueco::V11Minus,
            Font::EightA => Hueco::V11,
            Font::EightAPlus => Hueco::V11Plus,
            Font::EightBMinus => Hueco::V12Minus,
            Font::EightB => Hueco::V12,
            Font::EightBPlus => Hueco::V12Plus,
            Font::EightCMinus => Hueco::V13Minus,
            Font::EightC => Hueco::V13,
            Font::EightCPlus => Hueco::V13Plus,
            Font::NineAMinus => Hueco::V14Minus,
            Font::NineA => Hueco::V14,
            Font::NineAPlus => Hueco::V14Plus,
            Font::NineBMinus => Hueco::V15Minus,
            Font::NineB => Hueco::V15,
            Font::NineBPlus => Hueco::V15Plus,
            Font::NineCMinus => Hueco::V16Minus,
            Font::NineC => Hueco::V16,
            Font::NineCPlus => Hueco::V16Plus,
            _ => Hueco::V0, //default to V0
        }
    }
}
impl Hueco {
    pub fn iter() -> impl Iterator<Item = Hueco> {
        [
            Hueco::Vb,
            Hueco::V0Minus,
            Hueco::V0,
            Hueco::V0Plus,
            Hueco::V1Minus,
            Hueco::V1,
            Hueco::V1Plus,
            Hueco::V2Minus,
            Hueco::V2,
            Hueco::V2Plus,
            Hueco::V3Minus,
            Hueco::V3,
            Hueco::V3Plus,
            Hueco::V4Minus,
            Hueco::V4,
            Hueco::V4Plus,
            Hueco::V5Minus,
            Hueco::V5,
            Hueco::V5Plus,
            Hueco::V6Minus,
            Hueco::V6,
            Hueco::V6Plus,
            Hueco::V7Minus,
            Hueco::V7,
            Hueco::V7Plus,
            Hueco::V8Minus,
            Hueco::V8,
            Hueco::V8Plus,
            Hueco::V9Minus,
            Hueco::V9,
            Hueco::V9Plus,
            Hueco::V10Minus,
            Hueco::V10,
            Hueco::V10Plus,
            Hueco::V11Minus,
            Hueco::V11,
            Hueco::V11Plus,
            Hueco::V12Minus,
            Hueco::V12,
            Hueco::V12Plus,
            Hueco::V13Minus,
            Hueco::V13,
            Hueco::V13Plus,
            Hueco::V14Minus,
            Hueco::V14,
            Hueco::V14Plus,
            Hueco::V15Minus,
            Hueco::V15,
            Hueco::V15Plus,
            Hueco::V16Minus,
            Hueco::V16,
            Hueco::V16Plus,
            Hueco::V17Minus,
            Hueco::V17,
            Hueco::V17Plus,
        ].iter().copied()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
pub enum French { //5a, 6a+-, etc.
    One,
    Two,
    Three,
    FourA,
    FourB,
    FourC,
    FiveA,
    FiveB,
    FiveC,
    #[default]
    SixAMinus,
    SixA,
    SixAPlus,
    SixBMinus,
    SixB,
    SixBPlus,
    SixCMinus,
    SixC,
    SixCPlus,
    SevenAMinus,
    SevenA,
    SevenAPlus,
    SevenBMinus,
    SevenB,
    SevenBPlus,
    SevenCMinus,
    SevenC,
    SevenCPlus,
    EightAMinus,
    EightA,
    EightAPlus,
    EightBMinus,
    EightB,
    EightBPlus,
    EightCMinus,
    EightC,
    EightCPlus,
    NineAMinus,
    NineA,
    NineAPlus,
    NineBMinus,
    NineB,
    NineBPlus,
    NineCMinus,
    NineC,
    NineCPlus,
    //add more as needed
}
impl std::fmt::Display for French {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            French::One => write!(f, "1"),
            French::Two => write!(f, "2"),
            French::Three => write!(f, "3"),
            French::FourA => write!(f, "4a"),
            French::FourB => write!(f, "4b"),
            French::FourC => write!(f, "4c"),
            French::FiveA => write!(f, "5a"),
            French::FiveB => write!(f, "5b"),
            French::FiveC => write!(f, "5c"),
            French::SixAMinus => write!(f, "6a-"),
            French::SixA => write!(f, "6a"),
            French::SixAPlus => write!(f, "6a+"),
            French::SixBMinus => write!(f, "6b-"),
            French::SixB => write!(f, "6b"),
            French::SixBPlus => write!(f, "6b+"),
            French::SixCMinus => write!(f, "6c-"),
            French::SixC => write!(f, "6c"),
            French::SixCPlus => write!(f, "6c+"),
            French::SevenAMinus => write!(f, "7a-"),
            French::SevenA => write!(f, "7a"),
            French::SevenAPlus => write!(f, "7a+"),
            French::SevenBMinus => write!(f, "7b-"),
            French::SevenB => write!(f, "7b"),
            French::SevenBPlus => write!(f, "7b+"),
            French::SevenCMinus => write!(f, "7c-"),
            French::SevenC => write!(f, "7c"),
            French::SevenCPlus => write!(f, "7c+"),
            French::EightAMinus => write!(f, "8a-"),
            French::EightA => write!(f, "8a"),
            French::EightAPlus => write!(f, "8a+"),
            French::EightBMinus => write!(f, "8b-"),
            French::EightB => write!(f, "8b"),
            French::EightBPlus => write!(f, "8b+"),
            French::EightCMinus => write!(f, "8c-"),
            French::EightC => write!(f, "8c"),
            French::EightCPlus => write!(f, "8c+"),
            French::NineAMinus => write!(f, "9a-"),
            French::NineA => write!(f, "9a"),
            French::NineAPlus => write!(f, "9a+"),
            French::NineBMinus => write!(f, "9b-"),
            French::NineB => write!(f, "9b"),
            French::NineBPlus => write!(f, "9b+"),
            French::NineCMinus => write!(f, "9c-"),
            French::NineC => write!(f, "9c"),
            French::NineCPlus => write!(f, "9c+"),
        }
    }
}
impl std::convert::From<Yosemite> for French {
    fn from(yosemite: Yosemite) -> French {
        match yosemite {
            Yosemite::One => French::One,
            Yosemite::Two => French::One,
            Yosemite::Three => French::One,
            Yosemite::Four => French::One,
            Yosemite::FiveZero => French::One,
            Yosemite::FiveOne => French::One,
            Yosemite::FiveTwo => French::One,
            Yosemite::FiveThree => French::Two,
            Yosemite::FiveFour => French::Three,
            Yosemite::FiveFive => French::FourB,
            Yosemite::FiveSix => French::FiveA,
            Yosemite::FiveSeven => French::FiveB,
            Yosemite::FiveEight => French::FiveC,
            Yosemite::FiveNine => French::SixAMinus,
            Yosemite::FiveTenA => French::SixA,
            Yosemite::FiveTenB => French::SixAPlus,
            Yosemite::FiveTenC => French::SixBMinus,
            Yosemite::FiveTenD => French::SixBPlus,
            Yosemite::FiveElevenA => French::SixCMinus,
            Yosemite::FiveElevenB => French::SixCPlus,
            Yosemite::FiveElevenC => French::SevenA,
            Yosemite::FiveElevenD => French::SevenAPlus,
            Yosemite::FiveTwelveA => French::SevenB,
            Yosemite::FiveTwelveB => French::SevenBPlus,
            Yosemite::FiveTwelveC => French::SevenCMinus,
            Yosemite::FiveTwelveD => French::SevenCPlus,
            Yosemite::FiveThirteenA => French::EightAMinus,
            Yosemite::FiveThirteenB => French::EightA,
            Yosemite::FiveThirteenC => French::EightAPlus,
            Yosemite::FiveThirteenD => French::EightBMinus,
            Yosemite::FiveFourteenA => French::EightBPlus,
            Yosemite::FiveFourteenB => French::EightC,
            Yosemite::FiveFourteenC => French::EightCPlus,
            Yosemite::FiveFourteenD => French::NineAMinus,
            Yosemite::FiveFifteenA => French::NineAPlus,
            Yosemite::FiveFifteenB => French::NineB,
            Yosemite::FiveFifteenC => French::NineBPlus,
            Yosemite::FiveFifteenD => French::NineC,
            _ => French::SixC,
        }
    }

}
impl std::convert::From<Uiaa> for French {
    fn from(uiaa: Uiaa) -> French {
        match uiaa {
            Uiaa::I => French::One,
            Uiaa::Ii => French::Two,
            Uiaa::Iii => French::Three,
            Uiaa::IvMinus => French::FourA,
            Uiaa::Iv => French::FourB,
            Uiaa::IvPlus => French::FourC,
            Uiaa::VMinus => French::FiveA,
            Uiaa::V => French::FiveB,
            Uiaa::VPlus => French::FiveC,
            Uiaa::ViMinus => French::SixAMinus,
            Uiaa::Vi => French::SixA,
            Uiaa::ViPlus => French::SixAPlus,
            Uiaa::ViiMinus => French::SixBMinus,
            Uiaa::Vii => French::SixB,
            Uiaa::ViiPlus => French::SixBPlus,
            Uiaa::ViiiMinus => French::SixCMinus,
            Uiaa::Viii => French::SixC,
            Uiaa::ViiiPlus => French::SixCPlus,
            Uiaa::IxMinus => French::SevenAMinus,
            Uiaa::Ix => French::SevenA,
            Uiaa::IxPlus => French::SevenAPlus,
            Uiaa::XMinus => French::SevenBMinus,
            Uiaa::X => French::SevenB,
            Uiaa::XPlus => French::SevenBPlus,
            Uiaa::XiMinus => French::SevenCMinus,
            Uiaa::Xi => French::SevenC,
            Uiaa::XiPlus => French::SevenCPlus,
            Uiaa::XiiMinus => French::EightAMinus,
            Uiaa::Xii => French::EightA,
            Uiaa::XiiPlus => French::EightAPlus,
            _ => French::SixC,
        }
    }
}
impl French {
    pub fn iter() -> impl Iterator<Item = French> {
        [
            French::One,
            French::Two,
            French::Three,
            French::FourA,
            French::FourB,
            French::FourC,
            French::FiveA,
            French::FiveB,
            French::FiveC,
            French::SixAMinus,
            French::SixA,
            French::SixAPlus,
            French::SixBMinus,
            French::SixB,
            French::SixBPlus,
            French::SixCMinus,
            French::SixC,
            French::SixCPlus,
            French::SevenAMinus,
            French::SevenA,
            French::SevenAPlus,
            French::SevenBMinus,
            French::SevenB,
            French::SevenBPlus,
            French::SevenCMinus,
            French::SevenC,
            French::SevenCPlus,
            French::EightAMinus,
            French::EightA,
            French::EightAPlus,
            French::EightBMinus,
            French::EightB,
            French::EightBPlus,
            French::EightCMinus,
            French::EightC,
            French::EightCPlus,
            French::NineAMinus,
            French::NineA,
            French::NineAPlus,
            French::NineBMinus,
            French::NineB,
            French::NineBPlus,
            French::NineCMinus,
            French::NineC,
            French::NineCPlus,
        ].iter().copied()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
pub enum Uiaa {
    I,
    Ii,
    Iii,
    IvMinus,
    Iv,
    IvPlus,
    VMinus,
    V,
    VPlus,
    #[default]
    ViMinus,
    Vi,
    ViPlus,
    ViiMinus,
    Vii,
    ViiPlus,
    ViiiMinus,
    Viii,
    ViiiPlus,
    IxMinus,
    Ix,
    IxPlus,
    XMinus,
    X,
    XPlus,
    XiMinus,
    Xi,
    XiPlus,
    XiiMinus,
    Xii,
    XiiPlus,
}
impl std::fmt::Display for Uiaa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Uiaa::I => write!(f, "I"),
            Uiaa::Ii => write!(f, "II"),
            Uiaa::Iii => write!(f, "III"),
            Uiaa::IvMinus => write!(f, "IV-"),
            Uiaa::Iv => write!(f, "IV"),
            Uiaa::IvPlus => write!(f, "IV+"),
            Uiaa::VMinus => write!(f, "V-"),
            Uiaa::V => write!(f, "V"),
            Uiaa::VPlus => write!(f, "V+"),
            Uiaa::ViMinus => write!(f, "VI-"),
            Uiaa::Vi => write!(f, "VI"),
            Uiaa::ViPlus => write!(f, "VI+"),
            Uiaa::ViiMinus => write!(f, "VII-"),
            Uiaa::Vii => write!(f, "VII"),
            Uiaa::ViiPlus => write!(f, "VII+"),
            Uiaa::ViiiMinus => write!(f, "VIII-"),
            Uiaa::Viii => write!(f, "VIII"),
            Uiaa::ViiiPlus => write!(f, "VIII+"),
            Uiaa::IxMinus => write!(f, "IX-"),
            Uiaa::Ix => write!(f, "IX"),
            Uiaa::IxPlus => write!(f, "IX+"),
            Uiaa::XMinus => write!(f, "X-"),
            Uiaa::X => write!(f, "X"),
            Uiaa::XPlus => write!(f, "X+"),
            Uiaa::XiMinus => write!(f, "XI-"),
            Uiaa::Xi => write!(f, "XI"),
            Uiaa::XiPlus => write!(f, "XI+"),
            Uiaa::XiiMinus => write!(f, "XII-"),
            Uiaa::Xii => write!(f, "XII"),
            Uiaa::XiiPlus => write!(f, "XII+"),
        }
    }
}
impl std::convert::From<Yosemite> for Uiaa { // Not currently accurate grading, just testing
    fn from(yosemite: Yosemite) -> Uiaa {
        match yosemite {
            Yosemite::FiveZero => Uiaa::I,
            Yosemite::FiveOne => Uiaa::Ii,
            Yosemite::FiveTwo => Uiaa::Iii,
            Yosemite::FiveThree => Uiaa::IvMinus,
            Yosemite::FiveFour => Uiaa::Iv,
            Yosemite::FiveFive => Uiaa::IvPlus,
            Yosemite::FiveSix => Uiaa::VMinus,
            Yosemite::FiveSeven => Uiaa::V,
            Yosemite::FiveEight => Uiaa::VPlus,
            Yosemite::FiveNine => Uiaa::ViMinus,
            Yosemite::FiveTenA => Uiaa::Vi,
            Yosemite::FiveTenB => Uiaa::ViPlus,
            Yosemite::FiveTenC => Uiaa::ViiMinus,
            Yosemite::FiveTenD => Uiaa::Vii,
            Yosemite::FiveElevenA => Uiaa::ViiPlus,
            Yosemite::FiveElevenB => Uiaa::ViiiMinus,
            Yosemite::FiveElevenC => Uiaa::Viii,
            Yosemite::FiveElevenD => Uiaa::ViiiPlus,
            Yosemite::FiveTwelveA => Uiaa::IxMinus,
            Yosemite::FiveTwelveB => Uiaa::Ix,
            Yosemite::FiveTwelveC => Uiaa::IxPlus,
            Yosemite::FiveTwelveD => Uiaa::XMinus,
            Yosemite::FiveThirteenA => Uiaa::X,
            Yosemite::FiveThirteenB => Uiaa::XPlus,
            Yosemite::FiveThirteenC => Uiaa::XiMinus,
            Yosemite::FiveThirteenD => Uiaa::Xi,
            Yosemite::FiveFourteenA => Uiaa::XiPlus,
            Yosemite::FiveFourteenB => Uiaa::XiiMinus,
            Yosemite::FiveFourteenC => Uiaa::Xii,
            Yosemite::FiveFourteenD => Uiaa::XiiPlus,
            _ => Uiaa::Iv,
        }
    }
}
impl std::convert::From<French> for Uiaa {
    fn from(french: French) -> Uiaa {
        match french {
            French::One => Uiaa::I,
            French::Two => Uiaa::Ii,
            French::Three => Uiaa::Iii,
            French::FourA => Uiaa::IvMinus,
            French::FourB => Uiaa::Iv,
            French::FourC => Uiaa::IvPlus,
            French::FiveA => Uiaa::VMinus,
            French::FiveB => Uiaa::V,
            French::FiveC => Uiaa::VPlus,
            French::SixAMinus => Uiaa::ViMinus,
            French::SixA => Uiaa::Vi,
            French::SixAPlus => Uiaa::ViPlus,
            French::SixBMinus => Uiaa::ViiMinus,
            French::SixB => Uiaa::Vii,
            French::SixBPlus => Uiaa::ViiPlus,
            French::SixCMinus => Uiaa::ViiiMinus,
            French::SixC => Uiaa::Viii,
            French::SixCPlus => Uiaa::ViiiPlus,
            French::SevenAMinus => Uiaa::IxMinus,
            French::SevenA => Uiaa::Ix,
            French::SevenAPlus => Uiaa::IxPlus,
            French::SevenBMinus => Uiaa::XMinus,
            French::SevenB => Uiaa::X,
            French::SevenBPlus => Uiaa::XPlus,
            French::SevenCMinus => Uiaa::XiMinus,
            French::SevenC => Uiaa::Xi,
            French::SevenCPlus => Uiaa::XiPlus,
            French::EightAMinus => Uiaa::XiiMinus,
            French::EightA => Uiaa::Xii,
            French::EightAPlus => Uiaa::XiiPlus,
            _ => Uiaa::Viii,
        }
    }
}
impl Uiaa {
    pub fn iter() -> impl Iterator<Item = Uiaa> {
        [
            Uiaa::I,
            Uiaa::Ii,
            Uiaa::Iii,
            Uiaa::IvMinus,
            Uiaa::Iv,
            Uiaa::IvPlus,
            Uiaa::VMinus,
            Uiaa::V,
            Uiaa::VPlus,
            Uiaa::ViMinus,
            Uiaa::Vi,
            Uiaa::ViPlus,
            Uiaa::ViiMinus,
            Uiaa::Vii,
            Uiaa::ViiPlus,
            Uiaa::ViiiMinus,
            Uiaa::Viii,
            Uiaa::ViiiPlus,
            Uiaa::IxMinus,
            Uiaa::Ix,
            Uiaa::IxPlus,
            Uiaa::XMinus,
            Uiaa::X,
            Uiaa::XPlus,
            Uiaa::XiMinus,
            Uiaa::Xi,
            Uiaa::XiPlus,
            Uiaa::XiiMinus,
            Uiaa::Xii,
            Uiaa::XiiPlus,
        ].iter().copied()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum SendType {
    Redpoint, //Send after previous attempts/tops, no rests, lead or boulder (free) only
    Flash, //Completion on first attempt, with beta
    Onsight, //Completion on first attempt, without beta
    Pinkpoint, //Redpoint but on toprope or similar
    #[default]
    Attempt, //No completion
    Top, //Completed but with takes/falls
    Repeat, //Repeat of a route
    FreeSolo, //No rope or protection, but not on boulders
}
impl std::fmt::Display for SendType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SendType::Redpoint => write!(f, "Redpoint"),
            SendType::Flash => write!(f, "Flash"),
            SendType::Onsight => write!(f, "Onsight"),
            SendType::Pinkpoint => write!(f, "Pinkpoint"),
            SendType::Attempt => write!(f, "Attempt"),
            SendType::Top => write!(f, "Top"),
            SendType::Repeat => write!(f, "Repeat"),
            SendType::FreeSolo => write!(f, "Free Solo"),
        }
    }
}
impl SendType {
    pub fn iter() -> impl Iterator<Item = SendType> {
        [
            SendType::Redpoint,
            SendType::Flash,
            SendType::Onsight,
            SendType::Pinkpoint,
            SendType::Attempt,
            SendType::Top,
            SendType::Repeat,
            SendType::FreeSolo,
        ].iter().copied()
    }

}


impl TallGradeSysConvert for Yosemite {
    fn to_yosemite(&self) -> Yosemite {
        *self
    }

    fn to_french(&self) -> French {
        French::from(*self)
    }

    fn to_uiaa(&self) -> Uiaa {
        Uiaa::from(*self)
    }
}
impl TallGradeSysConvert for French {
    fn to_yosemite(&self) -> Yosemite {
        Yosemite::from(*self)
    }

    fn to_french(&self) -> French {
        *self
    }

    fn to_uiaa(&self) -> Uiaa {
        Uiaa::from(*self)
    }
}
impl TallGradeSysConvert for Uiaa {
    fn to_yosemite(&self) -> Yosemite {
        Yosemite::from(*self)
    }

    fn to_french(&self) -> French {
        French::from(*self)
    }

    fn to_uiaa(&self) -> Uiaa {
        *self
    }
}

impl BoulderGradeSysConvert for Hueco {
    fn to_hueco(&self) -> Hueco {
        *self
    }

    fn to_font(&self) -> Font {
        Font::from(*self)
    }
}
impl BoulderGradeSysConvert for Font {
    fn to_hueco(&self) -> Hueco {
        Hueco::from(*self)
    }

    fn to_font(&self) -> Font {
        *self
    }
}