use std::io::{Read, Seek, SeekFrom, Write};
use zbox::{init_env, OpenOptions, RepoOpener};
use anyhow::Result;

pub fn init() {
    println!("init storage adapter");
    init_env();
}

pub fn read_file(path: &str) -> Result<String> {
    // create and open a repository
    let mut repo = RepoOpener::new()
        .create(true)
        .open("file://./solid-fs", "")?;

    // create and open a file for writing
    let mut file = OpenOptions::new()
        .create(true)
        .open(&mut repo, path)?;

    let mut content = String::new();

    // read file content using std::io::Read trait
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut content)?;

    // return file content as String or empty
    Ok(content)
}

pub fn write_file(path: &str, content: &str) -> Result<()> {
    // create and open a repository
    let mut repo = RepoOpener::new()
        .create(true)
        .open("file://./solid-fs", "")?;

    // create and open a file for writing
    let mut file = OpenOptions::new()
        .create(true)
        .open(&mut repo, path)?;

     // use std::io::Write trait to write data into it
    file.write_all(content.as_bytes())?;

    // finish writting to make a permanent content version
    file.finish()?;

    Ok(())
}
