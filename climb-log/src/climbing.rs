use std::{default, fmt};

#[derive(Debug, Clone)]
pub struct Route {
    pub name: String,
    pub grade: Grade,
    pub style: Vec<Style>,
    pub length: u16,
    pub pitches: u8,
    pub location: String,
}
impl Route {
    pub fn new(name: String, grade: Grade, style: Vec<Style>, length: u16, pitches: u8, location: String) -> Route {
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
    pub date: Date,
    pub partner: String,
    pub attempts: u8,
    pub send_type: SendType,
    pub notes: String,
}
impl std::fmt::Display for Send {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} on {} with {} on {}", self.send_type, self.route, self.partner, self.date)
    }
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
            Grade::Yosemite(grade) => write!(f, "{}", grade),
            Grade::Font(grade) => write!(f, "{}", grade),
            Grade::Hueco(grade) => write!(f, "{}", grade),
            Grade::French(grade) => write!(f, "{}", grade),
            Grade::Uiaa(grade) => write!(f, "{}", grade),
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
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
        }
    }
}
impl std::convert::From<Hueco> for Yosemite { //Note, since this is between bouldering and big wall, it's not a solid conversion
    fn from(hueco: Hueco) -> Yosemite {
        match hueco {
            Hueco::VB => Yosemite::FiveFive,
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
            Uiaa::II => Yosemite::FiveTwo,
            Uiaa::III => Yosemite::FiveThree,
            Uiaa::IVMinus => Yosemite::FiveFour,
            Uiaa::IV => Yosemite::FiveFour,
            Uiaa::IVPlus => Yosemite::FiveFive,
            Uiaa::VMinus => Yosemite::FiveSix,
            Uiaa::V => Yosemite::FiveSeven,
            Uiaa::VPlus => Yosemite::FiveEight,
            Uiaa::VIMinus => Yosemite::FiveNine,
            Uiaa::VI => Yosemite::FiveTenA,
            Uiaa::VIPlus => Yosemite::FiveTenB,
            Uiaa::VIIMinus => Yosemite::FiveTenC,
            Uiaa::VII => Yosemite::FiveTenD,
            Uiaa::VIIPlus => Yosemite::FiveElevenA,
            Uiaa::VIIIMinus => Yosemite::FiveElevenC,
            Uiaa::VIII => Yosemite::FiveElevenD,
            Uiaa::VIIIPlus => Yosemite::FiveTwelveA,
            Uiaa::IXMinus => Yosemite::FiveTwelveC,
            Uiaa::IX => Yosemite::FiveTwelveD,
            Uiaa::IXPlus => Yosemite::FiveThirteenB,
            Uiaa::XMinus => Yosemite::FiveThirteenC,
            Uiaa::X => Yosemite::FiveThirteenD,
            Uiaa::XPlus => Yosemite::FiveFourteenA,
            Uiaa::XIMinus => Yosemite::FiveFourteenB,
            Uiaa::XI => Yosemite::FiveFourteenD,
            Uiaa::XIPlus => Yosemite::FiveFifteenA,
            Uiaa::XIIMinus => Yosemite::FiveFifteenB,
            Uiaa::XII => Yosemite::FiveFifteenC,
            Uiaa::XIIPlus => Yosemite::FiveFifteenD,
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
            _ => Yosemite::FiveNine, //default to 5.9
        }
    }
}
impl default::Default for Yosemite {
    fn default() -> Yosemite {
        Yosemite::FiveNine
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

#[derive(Debug, Copy, Clone)]
pub enum Font {
    OneMinus,
    One,
    OnePlus,
    TwoMinus,
    Two,
    TwoPlus,
    ThreeMinus,
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


#[derive(Debug, Copy, Clone)]
pub enum Hueco { //V Scale
    VB,
    V0Minus,
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
}
impl std::fmt::Display for Hueco {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hueco::VB => write!(f, "VB"),
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
        }
    }
}


#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum Uiaa {
    I,
    II,
    III,
    IVMinus,
    IV,
    IVPlus,
    VMinus,
    V,
    VPlus,
    VIMinus,
    VI,
    VIPlus,
    VIIMinus,
    VII,
    VIIPlus,
    VIIIMinus,
    VIII,
    VIIIPlus,
    IXMinus,
    IX,
    IXPlus,
    XMinus,
    X,
    XPlus,
    XIMinus,
    XI,
    XIPlus,
    XIIMinus,
    XII,
    XIIPlus,
}
impl std::fmt::Display for Uiaa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Uiaa::I => write!(f, "I"),
            Uiaa::II => write!(f, "II"),
            Uiaa::III => write!(f, "III"),
            Uiaa::IVMinus => write!(f, "IV-"),
            Uiaa::IV => write!(f, "IV"),
            Uiaa::IVPlus => write!(f, "IV+"),
            Uiaa::VMinus => write!(f, "V-"),
            Uiaa::V => write!(f, "V"),
            Uiaa::VPlus => write!(f, "V+"),
            Uiaa::VIMinus => write!(f, "VI-"),
            Uiaa::VI => write!(f, "VI"),
            Uiaa::VIPlus => write!(f, "VI+"),
            Uiaa::VIIMinus => write!(f, "VII-"),
            Uiaa::VII => write!(f, "VII"),
            Uiaa::VIIPlus => write!(f, "VII+"),
            Uiaa::VIIIMinus => write!(f, "VIII-"),
            Uiaa::VIII => write!(f, "VIII"),
            Uiaa::VIIIPlus => write!(f, "VIII+"),
            Uiaa::IXMinus => write!(f, "IX-"),
            Uiaa::IX => write!(f, "IX"),
            Uiaa::IXPlus => write!(f, "IX+"),
            Uiaa::XMinus => write!(f, "X-"),
            Uiaa::X => write!(f, "X"),
            Uiaa::XPlus => write!(f, "X+"),
            Uiaa::XIMinus => write!(f, "XI-"),
            Uiaa::XI => write!(f, "XI"),
            Uiaa::XIPlus => write!(f, "XI+"),
            Uiaa::XIIMinus => write!(f, "XII-"),
            Uiaa::XII => write!(f, "XII"),
            Uiaa::XIIPlus => write!(f, "XII+"),
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, )]
pub enum SendType {
    Redpoint, //Send after previous attempts/tops, no rests, lead or boulder (free) only
    Flash, //Completion on first attempt, with beta
    Onsight, //Completion on first attempt, without beta
    Pinkpoint, //Redpoint but on toprope or similar
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
impl std::default::Default for SendType {
    fn default() -> SendType {
        SendType::Attempt
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

#[derive(Debug, Copy, Clone, Default)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}
impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self.month, self.day, self.year)
    }
}
