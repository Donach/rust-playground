/*

    // Print -> flushed after newline and/or manually
*/

fn main() {
    use std::io;
    println!("Hello, world!");
    let mut input = String::new();

    let mut stdin = io::stdin();
    //stdin.read_line(&mut input).expect("Failed to read line ");
    println!("You entered: {}", input);

    // FromStr
    // fn parse<T: FromStr>(self) -> io::Result<T>;


    // File
    // No need to close manuallly
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;

    let mut file = File::open("/home/donach/Repositories/rust-learn/lesson-08/src/hello.txt").expect("Failed to create file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");
    println!("File content: {}", content);


    // Builder pattern
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("/home/donach/Repositories/rust-learn/lesson-08/src/hello.txt")
        .expect("Failed to create file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");
    println!("File #2 content: {}", content);

    main_buffered();

}

// Fastest
fn main_buffered() {
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, BufRead, BufWriter, prelude::*};
    let file = File::open("/home/donach/Repositories/rust-learn/lesson-08/src/hello.txt").expect("Failed to create file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.expect("Failed to read line"));
    }

    // Writer
    let file = File::create("/home/donach/Repositories/rust-learn/lesson-08/src/hellob.txt").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    // b"Hello World!" -> &[u8]
    // r"\n" -> ignore newline
    // r#""""""""# -> ignore quotation
    writer.write_all(br#"Hello """"World!"#).expect("Failed to write to file");

}



fn main2() {
    use std::fs::File;
    use std::io::ErrorKind;

    match File::open("nonexistent.txt") {
        Ok(_) => println!("File found"),
        Err(error) => { match error.kind() {
            ErrorKind::NotFound => println!("File not found"),
            ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("Other error"),
        }
        }
    }
}

// Paths

use std::path::{Path, PathBuf};

use prelude::IoWrite;
fn main3() {
    let path = Path::new("/home/donach/Repositories/rust-learn/lesson-08/src/hello.txt");
    let mut path_buff = PathBuf::from("/path/to");
    path_buff.push("hello.txt");
    println!("Path: {}", path.display());

    let parent = Path::new("/path/to");
    let child  = parent.join ("hello.txt");
    // PathBuf -> AsRef<Path>
    // &PathBuf -> &Path
    // &String -> &str

    //File::open(&child);

    if let Some(extention) = child.extension() {
        //println!("Extention: {}", extention);
        
    }
}

// FS
fn main4() {
    use std::fs;
    use std::fs::File;

    fs::create_dir("/home/donach/Repositories/rust-learn/lesson-08/src/hello").expect("Failed to create directory");

    // Reads only top-level objects of dir
    for entry in fs::read_dir("/home/donach/Repositories/rust-learn/lesson-08/src/hello").expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        println!("Entry: {}", entry.path().display());
    }

    fs::remove_dir_all("/home/donach/Repositories/rust-learn/lesson-08/src/hello").expect("Failed to remove directory");

    let metadata = fs::metadata("/home/donach/Repositories/rust-learn/lesson-08/src/hello.txt").expect("Failed to read metadata");

    println!("Is file: {}", metadata.is_file());
    println!("Is dir: {}", metadata.is_dir());
    println!("File size: {}", metadata.len());
    let mut permissions = metadata.permissions();

    println!("Read-only? {}", permissions.readonly());

    permissions.set_readonly(true);
    fs::set_permissions("/home/donach/Repositories/rust-learn/lesson-08/src/hello.txt", permissions).expect("Failed to set permissions");
}


fn main5() {
    use std::io::{Seek, SeekFrom, Cursor, Read};

    let mut cursor = Cursor::new(Vec::new());
    cursor.write_all(&[1, 2, 3, 4, 5]).unwrap();

    // Seek to the third byte
    cursor.seek(SeekFrom::Start(2)).unwrap();

    // Read a byte from this position
    let mut buffer = [0u8; 1];
    cursor.read(&mut buffer).unwrap();

    println!("Read byte: {}", buffer[0]);
}