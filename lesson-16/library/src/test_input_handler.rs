#[cfg(test)]

#[test]
fn test_handle_vec_input() {
    // Test cases for handle_vec_input function
    use uuid::Uuid;
    use crate::input_handler::handle_vec_input;
    let path = std::env::current_dir().unwrap();
    let cwd = path.to_str().unwrap();

    assert!(handle_vec_input(vec![".quit".to_string()]).is_err());
    assert!(handle_vec_input(vec!["Hello world!".to_string()]).is_ok());
    assert!(handle_vec_input(vec![".auth".to_string(), Uuid::new_v4().to_string()]).is_ok());
    assert!(handle_vec_input(vec![".file".to_string(), format!("{}/data/dummy.txt", cwd)]).is_ok());
    assert!(handle_vec_input(vec![".image".to_string(), format!("{}/data/image.png", cwd)]).is_ok());
}
