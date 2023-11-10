use std::collections::HashMap;
use std::ffi::{OsString, OsStr, CString};

fn main() {
    println!("Hello, world!");
    let vec = vec![1, 2, 3, 4, 5];
    //let boxed_slice: Box<[i32]> = &vec.into_boxed_slice();

    let arr = [1, 2, 3, 4, 5];
    let boxed_slice: Box<[i32]> = Box::new(arr);
    println!("{:?}", boxed_slice);

    for num in &*boxed_slice {
        println!("{}", num);
    }

    println!("{:?}", &vec[1..4]);

    let mut words = vec!["hello", "world", "abc", "cbc"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("{:?}", words);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry("Red".to_string()).or_insert(10);
    scores
        .entry("Yellow".to_string())
        .and_modify(|v| *v *= 10)
        .or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams = vec!["Blue", "Red", "Yellow"];
    let initial_scores = vec![10];

    let scores: HashMap<_, Option<_>> = teams
        .iter()
        .zip(
            initial_scores
                .iter()
                .map(Option::Some)
                .chain(std::iter::repeat(None)),
        )
        .collect();
    println!("{:?}", scores);

    // String Types
    // Primitive: &str (string slice)
    // In std: String, OsString, OsStr, CString, CStr, Cow<str>
    // OsString OsStr -> in file paths
    let os_string = OsString::from("Helllo from OS!");
    let os_str: &OsStr = os_string.as_ref();
    println!("{}", os_str.to_str().unwrap());
    cstring();

}
extern "C" {
    fn puts(s: *const i8);
}
fn cstring(){
    let c_string = CString::new("Hello from C!")
        .expect("Failed to create CString");
    unsafe {
        puts(c_string.as_ptr());
    }

}

fn cow() {
    use std::borrow::Cow;
    let s: &str = "hello";
    let t: String = String::from("Hello");
    let s_cow = Cow::Borrowed(s);
    let t_cow: Cow<str> = Cow::Owned(t);
}