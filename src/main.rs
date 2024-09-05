use std::{
    env,
    error::Error,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

mod cal;
use cal::{ Year, Sign };

fn main() -> Result<(), Box<dyn Error>>
{
    let sign =
    {
        let result =
            env::var("HCAL_SIGN")
            .map_err(|_| "Failed to read HCAL_SIGN".to_string())
            .and_then(Sign::from_string);
        match result
        {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{e}");
                return Err("Could not read your sign".into())
            }
        }
    };
    let now =
        SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    let year = Year::new(now, sign);
    println!("{year}");

    Ok(())
}
