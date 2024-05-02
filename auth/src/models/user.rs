use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
use std::error::Error;

use crate::schema::users::{self};

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn create(conn: &mut PgConnection, user: NewUser) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn find_by_email(conn: &mut PgConnection, email: &str) -> Option<User> {
        users::dsl::users
            .select(User::as_select())
            .filter(users::dsl::email.eq(email))
            .first(conn)
            .ok()
    }

    pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Option<User> {
        users::dsl::users
            .select(User::as_select())
            .filter(users::dsl::username.eq(username))
            .first(conn)
            .ok()
    }

    pub fn get_user(conn: &mut PgConnection, user_id: &i32) -> Option<User> {
        users::dsl::users.find(user_id).first(conn).ok()
    }
    pub fn list_users(conn: &mut PgConnection) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let users = users::dsl::users.load::<(i32, String, String, String)>(conn)?;

        let users: Vec<User> = users
            .into_iter()
            .map(|(user_id, username, email, _)| User {
                id: user_id,
                username,
                email,
                password: String::new(),
            })
            .collect();

        Ok(users)
    }
}

#[tonic::async_trait]
pub trait IUserCache {
    async fn set_exp(
        &self,
        key: &String,
        value: &String,
        seconds: usize,
    ) -> Result<(), Box<dyn Error>>;
    async fn del_val_for_key(&self, keys: &String) -> Result<u64, Box<dyn Error>>;
    async fn get_val(&self, key: &String) -> Result<Option<String>, Box<dyn Error>>;
}
