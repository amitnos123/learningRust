#[derive(FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String
}

fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

fn update(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = %1, author = $2 WHERE isbn = $3";
    
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

fn read(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    sqlx::query(query);

    // Single row
    // If no rows are found, throw an error
    let row = query.fetch_one(pool).await?;

    //-------------------------------------//

    // Return single row in option.
    // If no rows are found, return None
    let maybe_row = query.fetch_optional(pool).await?;

    //-------------------------------------//

    // Return all the rows as vector.
    let rows = query.fetch_all(pool).await?;

    let book = Book{
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn")
    };

    let books = rows.iter().map(|row| {
        Book{
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn")
        }
    }).collect();

    //-------------------------------------//

    // Return all the rows as stream like type.
    // Async approch
    let rows = query.fetch(pool).await?;

    let mut books = vec![];

    // Require futures crate
    while let Some(row) = rows.try_next().await? {
        books.push(Book{
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn")
        })
    }

    //-------------------------------------//

    let q = "SELECT title, author, isbn FROM book";

    // Require the struct to derive(FromRow)
    let query = sqlx::query_as::<_, Book>(q);

    let books = query.fetch_all.await?;

    Ok(book)
}

fn insert_book(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    // Starting transaction
    let mut txn = conn.begin().await?;

    let author_q = r"
        INSERT INTOP author (name) VALUES ($1) RETURNING id
    ";

    let book_q = r"
        INSERT INTO BOOK (title, author_id, isbn)
        VALUES ($1, $2, $3)
    ";

    let author_id: (i64,) = sqlx::query_as(author_q)
        .bind(&book.author)
        .fetch_one(&mut thx)
        .await?;

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    txn.commit().await?;
    txn.rollback().await?;

    Ok(())
}

fn main() {
    let url = "postgress://dbuser:mysecretpassword@localhost:5432/bookstore";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let book = Book {
        title: "Salem's Lot".to_string(),
        author: "Stephen King".to_string(),
        isbn: "978-0-385-00751-1".to_string(),
    };

    create(&book, &pool);

    Ok(())
}