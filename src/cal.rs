use std::fmt;

mod horo;
use self::horo::{
    Horo,
    Mood,
};

pub struct Year
{
    ad:    u16,
    today: u16,
    weekd: u8,
    days:  Vec<Mood>,
    sign:  Sign,
}

#[derive(Copy, Clone)]
pub enum Sign
{
    Aries,     Taurus,   Gemini,
    Cancer,    Leo,      Virgo,
    Libra,     Scorpio,  Sagittarius,
    Capricorn, Aquarius, Pisces,
}

impl Sign
{
    pub fn from_string(input: String) -> Result<Self, String>
    {
        match input.as_str()
        {
            "aries"       => Ok(Self::Aries),
            "taurus"      => Ok(Self::Taurus),
            "gemini"      => Ok(Self::Gemini),
            "cancer"      => Ok(Self::Cancer),
            "leo"         => Ok(Self::Leo),
            "virgo"       => Ok(Self::Virgo),
            "libra"       => Ok(Self::Libra),
            "scorpio"     => Ok(Self::Scorpio),
            "sagittarius" => Ok(Self::Sagittarius),
            "capricorn"   => Ok(Self::Capricorn),
            "aquarius"    => Ok(Self::Aquarius),
            "pisces"      => Ok(Self::Pisces),
            _             => Err(
format!(
"\
Could not match \"{}\"
Possible values:
    aries
    taurus
    gemini
    cancer
    leo
    virgo
    libra
    scorpio
    sagittarius
    capricorn
    aquarius
    pisces\
",
input
)
            )
        }
    }
}

impl fmt::Display for Sign
{
    fn fmt(&self,
           f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        let symbol = match self
        {
            Self::Aries       => '\u{2648}',
            Self::Taurus      => '\u{2649}',
            Self::Gemini      => '\u{264a}',
            Self::Cancer      => '\u{264b}',
            Self::Leo         => '\u{264c}',
            Self::Virgo       => '\u{264d}',
            Self::Libra       => '\u{264e}',
            Self::Scorpio     => '\u{264f}',
            Self::Sagittarius => '\u{2650}',
            Self::Capricorn   => '\u{2651}',
            Self::Aquarius    => '\u{2652}',
            Self::Pisces      => '\u{2653}',
        };
        write!(f, "{symbol}")
    }
}

impl Year
{
    pub fn new(unix_timestamp: u64,
               sign:           Sign) -> Self
    {
        const DAY_SECS:    u64 = 24 * 60 * 60;
        const YEAR_LENGTH: f32 = 365. + 1./4. - 1./100. + 1./400.;
        const YEAR_SECS:   u64 = (DAY_SECS as f32 * YEAR_LENGTH) as u64;

        let ad = (unix_timestamp / YEAR_SECS + 1970) as u16;
        let today = (unix_timestamp % YEAR_SECS / DAY_SECS) as u16;
        let weekd = (ad as u64 * YEAR_SECS / DAY_SECS % 7) as u8;
        let length = if is_leap(ad) { 366 } else { 365 };
        let rng = Horo::new(ad as u64 + sign as u64);
        let days = (0..length).zip(rng).map(|(_, m)| m).collect();

        Self
        {
            ad,
            today,
            weekd,
            days,
            sign,
        }
    }

    const fn current_month(&self) -> u8
    {
        let mut month = 0;
        let mut countdown = self.today;
        loop
        {
            let length = month_length(month, is_leap(self.ad));
            if countdown < length { break }
            month += 1;
            countdown -= length;
        }
        month
    }

    const fn day_offset(&self) -> u16
    {
        let mut month = 0;
        let mut count = 0;
        while month < self.current_month()
        {
            let length = month_length(month, is_leap(self.ad));
            month += 1;
            count += length;
        }
        count
    }
}

impl fmt::Display for Year
{
    fn fmt(&self,
           f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        let month = match self.current_month()
        {
            0 => "January",  1 => "February",  2 => "March",
            3 => "April",    4 => "May",       5 => "June",
            6 => "July",     7 => "August",    8 => "September",
            9 => "October", 10 => "November", 11 => "December",
            _ => "How"
        };
        let header = format!("{}{} {}",
                             self.sign, month, self.ad);
        writeln!(f, "{:^20}", header)?;
        writeln!(f, "Su Mo Tu We Th Fr Sa")?;
        let mut weekday = (self.day_offset() + self.weekd as u16) % 7;
        for _ in 0..weekday
        {
            write!(f, "   ")?;
        }
        for day in 0..month_length(self.current_month(), is_leap(self.ad))
        {
            let today = self.day_offset() + day;
            let mood = &self.days[today as usize];
            let sq = if self.today == today { "\u{1b}[7m" } else { "" };
            write!(f, "{}{}{:>2}\u{1b}[0m ",
                   mood, sq, day+1)?;
            weekday += 1;
            weekday %= 7;
            if weekday == 0 { writeln!(f)? }
        }
        Ok(())
    }
}

const fn is_leap(year: u16) -> bool
{
    if year %   4 != 0 { return false }
    if year % 100 != 0 { return true }
    if year % 400 != 0 { return false }
    true
}

const fn month_length(month: u8, leap: bool) -> u16
{
    let february = if leap { 29 } else { 28 };
    match month
    {
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        3 | 5 | 8 | 10             => 30,
        1 => february,
        _ => 366,
    }
}

