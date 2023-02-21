use diesel::{table, AsChangeset, Identifiable, Insertable, Queryable};
use diesel_derives_extra::{Model, NewModel};

#[test]
fn simple_model() {
    table! {
        jobs (id) {
            id -> Int4,
            payload -> Varchar,
        }
    }

    #[derive(Debug, Queryable, Identifiable, AsChangeset, Model)]
    #[diesel(table_name = jobs)]
    struct Job {
        id: i32,
        payload: String,
    }

    #[derive(Debug, Insertable, NewModel)]
    #[diesel(table_name = jobs)]
    #[model(Job)]
    struct NewJob {
        payload: String,
    }
}

#[test]
fn with_lifetime() {
    table! {
        jobs (id) {
            id -> Int4,
            payload -> Varchar,
        }
    }

    #[derive(Debug, Queryable, Identifiable, AsChangeset, Model)]
    #[diesel(table_name = jobs)]
    struct Job {
        id: i32,
        payload: String,
    }

    #[derive(Debug, Insertable, NewModel)]
    #[diesel(table_name = jobs)]
    #[model(Job)]
    struct NewJob<'a> {
        payload: &'a str,
    }
}

#[test]
fn new_without_model() {
    table! {
        jobs (id) {
            id -> Int4,
            payload -> Varchar,
        }
    }

    #[derive(Debug, Queryable, Identifiable)]
    #[diesel(table_name = jobs)]
    #[allow(dead_code)]
    struct Job {
        id: i32,
        payload: String,
    }

    #[derive(Debug, Insertable, NewModel)]
    #[diesel(table_name = jobs)]
    #[model(Job)]
    struct NewJob {
        payload: String,
    }
}
