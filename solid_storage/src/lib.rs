use std::io::{Read, Seek, SeekFrom, Write};
use zbox::{init_env, OpenOptions, RepoOpener};
use zbox::Error;

pub fn init() {
    println!("init storage adapter");
    init_env();
}

pub fn read_file(path: &str) -> Result<String, Error> {
    // create and open a repository
    let mut repo = RepoOpener::new().create(true).open("file://./solid-fs", "").unwrap();

    // create and open a file for writing
    let mut file = OpenOptions::new().create(true).open(&mut repo, path).unwrap();
    let mut content = String::new();

    // read file content using std::io::Read trait
    file.seek(SeekFrom::Start(0)).unwrap();
    file.read_to_string(&mut content).unwrap();

    // return file content as String or empty
    Ok(content)
}

pub fn write_file(path: &str, content: &str) -> Result<(), Error> {
    // create and open a repository
    let mut repo = RepoOpener::new().create(true).open("file://./solid-fs", "").unwrap();

    // create and open a file for writing
    let mut file = OpenOptions::new().create(true).open(&mut repo, path).unwrap();

     // use std::io::Write trait to write data into it
     file.write_all(content.as_bytes()).unwrap();

    // finish writting to make a permanent content version
    file.finish().unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
