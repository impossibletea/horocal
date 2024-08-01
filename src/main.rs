use std::{
    error::Error,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

mod cal;
use cal::Year;

fn main() -> Result<(), Box<dyn Error>>
{
    let now =
        SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    let year = Year::new(now);
    println!("{year}");

    Ok(())
}
