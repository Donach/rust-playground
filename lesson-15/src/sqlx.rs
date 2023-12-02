use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	let database_url = "postgres://username:password@localhost/dbname";
	let pool = PgPoolOptions::new()
    	.max_connections(5)
    	.connect(database_url)
    	.await?;

	println!("Connected to the database.");
	Ok(())
}




use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	let pool = PgPoolOptions::new()
    	.max_connections(5)
    	.connect("postgres://username:password@localhost/dbname")
    	.await?;

	sqlx::query!("INSERT INTO users (name, email) VALUES ($1, $2)", "Alice", "alice@example.com")
    	.execute(&pool)
    	.await?;

	println!("User added.");
	Ok(())
}



use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	let pool = PgPoolOptions::new()
    	.max_connections(5)
    	.connect("postgres://username:password@localhost/dbname")
    	.await?;

	let rows = sqlx::query("SELECT id, name, email FROM users")
    	.map(|row: PgRow| {
        	let id: i32 = row.get("id");
        	let name: String = row.get("name");
        	let email: String = row.get("email");
        	(id, name, email)
    	})
    	.fetch_all(&pool)
    	.await?;

	for row in rows {
  	  	println!("ID: {}, Name: {}, Email: {}", row.0, row.1, row.2);
	}

	Ok(())
}


// sqlx - compile time verification
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	let pool = PgPoolOptions::new()
    	.max_connections(5)
    	.connect("postgres://username:password@localhost/dbname")
    	.await?;

	let user = sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = $1", 1)
    	.fetch_one(&pool)
    	.await?;

	println!("User: {:?}", user);
	Ok(())
}

#[derive(Debug)]
struct User {
	id: i32,
	name: String,
	email: String,
}

// sqlx - transaction
use sqlx::postgres::{PgPoolOptions, Transaction};
use sqlx::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let pool = PgPoolOptions::new()
    	.max_connections(5)
    	.connect("postgres://username:password@localhost/dbname")
    	.await?;

	let mut transaction = pool.begin().await?;

	sqlx::query!("INSERT INTO users (name, email) VALUES ($1, $2)", "Bob", "bob@example.com")
    	.execute(&mut transaction)
    	.await?;

	transaction.commit().await?;
	println!("Transaction committed.");

	Ok(())
}



