use std::fmt;

pub struct Horo { state: u64 }

pub enum Mood
{
    Worst,
    Bad,
    Normal,
    Good,
    Best,
}

impl Horo
{
    pub const fn new(mut seed: u64) -> Self
    {
        // https://en.wikipedia.org/wiki/Xorshift#Initialization
        seed  = seed.wrapping_add(0x9e3779b97f4a7c15);
        seed ^= seed >> 30;
        seed  = seed.wrapping_mul(0xbf58476d1ce4e5b9);
        seed ^= seed >> 27;
        seed  = seed.wrapping_mul(0x94d049bb133111eb);
        seed ^= seed >> 31;

        Self { state: seed }
    }

    pub fn rand(&mut self) -> u64
    {
        let x = &mut self.state;
        *x ^= *x << 13;
        *x ^= *x >> 7;
        *x ^= *x << 17;
        self.state
    }
}

impl Iterator for Horo
{
    type Item = Mood;

    fn next(&mut self) -> Option<Self::Item>
    {
        // https://en.wikipedia.org/wiki/Standard_deviation
        const STD_DEV_1: f32 = 0.341;
        const STD_DEV_2: f32 = 0.136;
        const STD_DEV_3: f32 = 0.021;
        const MIN: f32 = 0.  + STD_DEV_3;
        const LO:  f32 = MIN + STD_DEV_2;
        const MED: f32 = LO  + STD_DEV_1 * 2.;
        const HI:  f32 = MED + STD_DEV_2;

        let x = self.rand() as f32 / u64::MAX as f32;
        let result = match &x
        {
            x if (0. ..MIN).contains(x) => Self::Item::Worst,
            x if (MIN.. LO).contains(x) => Self::Item::Bad,
            x if (LO ..MED).contains(x) => Self::Item::Normal,
            x if (MED.. HI).contains(x) => Self::Item::Good,
            x if (HI ..=1.).contains(x) => Self::Item::Best,
            _ => return None
        };

        Some(result)
    }
}

impl fmt::Display for Mood
{
    fn fmt(&self,
           f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>
    {
        let x = match self
        {
            Self::Worst  => 91,
            Self::Bad    => 31,
            Self::Normal =>  0,
            Self::Good   => 32,
            Self::Best   => 92,
        };
        write!(f, "\u{1b}[{x}m")
    }
}

