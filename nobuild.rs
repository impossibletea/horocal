use std::{
    error::Error,
    process::Command,
    iter::FromIterator,
    fs::{
        self,
        DirEntry,
    },
};

const EXEC_PATH: &str = "build/horocal";

fn main() -> Result<(), Box<dyn Error>>
{
    fs::create_dir_all("build")?;

    match rebuild_myself()
    {
        Ok(_)  => eprintln!("INFO: Successfully rebuilt myself"),
        Err(e) => eprintln!("WARNING: Did not rebuild myself: {e}"),
    }

    Command::new("rustc")
    .arg("src/main.rs")
    .arg("-o")
    .arg(EXEC_PATH)
    .arg("-C")
    .arg("opt-level=3")
    .spawn()?;

    eprintln!("INFO: created an executable {EXEC_PATH}");

    Ok(())
}

fn rebuild_myself() -> Result<(), Box<dyn Error>>
{
    let repo: Vec<DirEntry> =
        fs::read_dir("./")
        .and_then(Result::from_iter)?;
    let myself =
        repo.iter()
        .find(|dir| dir.file_name() == "nobuild")
        .ok_or("Did not find `nobuild`, no need for rebuild")?
        .metadata()?
        .modified()?;
    let my_source =
        repo.iter()
        .find(|dir| dir.file_name() == "nobuild.rs")
        .ok_or("Expected to find `nobuild.rs`")?
        .metadata()?
        .modified()?;

    if my_source < myself { return Err("Up to date".into()) }

    Command::new("rustc")
    .arg("nobuild.rs")
    .spawn()?;

    Ok(())
}

