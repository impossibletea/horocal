mod cal;
use cal::Year;

fn main()
{
    let unix_timestamp = 1707166800;
    let year = Year::new(unix_timestamp);
    println!("{year}");
}
