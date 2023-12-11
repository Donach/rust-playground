#[cfg(test)]
#[tokio::test]
async fn test_db_client() {
    use crate::db_client::*;
    use uuid::Uuid;
    let path = std::env::current_dir().unwrap();
    let path = path.parent().unwrap();
    let cwd = path.to_str().unwrap();
    println!("cwd: {}", cwd);
    std::env::set_var("DATABASE_URL", format!("sqlite:{}/local.db", cwd));

    let db_pool = setup_database_pool().await;
    assert!(db_pool.is_ok());

    let db_pool = db_pool.unwrap();
    let uid = Uuid::new_v4();
    let msg = crate::MessageType::Text("Hello world".to_string());

    let result_auth = auth_client(&db_pool, uid).await;
    assert!(result_auth.is_ok());

    let result_savemsg = save_message(&db_pool, uid.to_string(), &msg).await;
    assert!(result_savemsg.is_ok());
}
