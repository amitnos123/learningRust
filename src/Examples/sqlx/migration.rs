fn main() {
    let url = "postgress://dbuser:mysecretpassword@localhost:5432/bookstore";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!(".migrations").run(&pool).await?;

    // My default sqlx will set migrations directory in root directory of project /migrations
    // The files inside the directory are named as the following:
    // <version>_<description>.sql

    Ok(())
}