use sqlx::{query_builder::QueryBuilder, Execute};

#[derive(Default)]
struct Search {
    id: Option<i64>,
    username: Option<String>,
    min_age: Option<i8>,
    max_age: Option<i8>,
}

fn search_query(search: Search) -> String {
    if let (None, None, None, None) = (search.id, &search.username, search.min_age, search.max_age) {
        return "SELECT * from users".into();
    }

    let mut query = QueryBuilder::new("SELECT * from users");

    query.push(" WHERE");

    if let Some(id) = search.id {
        query.push(" id = ");
        query.push_bind(id);
    }

    if let Some(username) = search.username {
        if search.id.is_some() {
            query.push(" AND");
        }

        query.push(" username = ");
        query.push_bind(username);
    }

    if let Some(min_age) = search.min_age {
        if search.id.is_some() || search.username.is_some() {
            query.push(" AND");
        }

        query.push(" age > ");
        query.push_bind(min_age);

    }


    if let Some(max_age) = search.max_age {
        if search.id.is_some() || search.username.is_some() || search.min_age.is_some() {
            query.push(" AND");
        }

        query.push(" age < ");
        query.push_bind(max_age);

    }

    query.build().sql().into()
}

fn main() {
    dbg!(search_query(Search::default())); // "SELECT * from users"

    dbg!(search_query(Search {
        id: Some(12),
        username: None,
        min_age: None,
        max_age: None,
    })); // "SELECT * from users where id = $1"

    dbg!(search_query(Search {
        id: Some(12),
        username: Some("Bob".into()),
        min_age: None,
        max_age: None,
    })); // "SELECT * from users where id = $1 AND username = $2"
    dbg!(search_query(Search {
        id: Some(12),
        username: Some("Bob".into()),
        min_age: Some(10),
        max_age: Some(70),
    })); // "SELECT * from users where id = $1 AND username = $2 AND age > $3 AND age < $4"
}