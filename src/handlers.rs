use super::{
    models::{User, NewUser},
    schema::users::dsl::*,
    Pool,
};
use diesel::{
    QueryDsl, RunQueryDsl,
    dsl::{delete, insert_into},
};
use serde::{Serialize, Deserialize};
use actix_web::{web, Error, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// Handler for GET /users
pub(crate) async fn get_users(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    fn execute_db(pool: web::Data<Pool>) -> anyhow::Result<Vec<User>> {
        let conn = pool.get()?;
        let item = users.load::<User>(&conn)?;
        Ok(item)
    }

    Ok(
        web::block(move || execute_db(pool))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

pub(crate) async fn get_user_by_id(pool: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    fn execute_db(pool: web::Data<Pool>, user_id: i32) -> anyhow::Result<User> {
        let conn = pool.get()?;
        let item = users.find(user_id).get_result::<User>(&conn)?;
        Ok(item)
    }

    Ok(
        web::block(move || execute_db(pool, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

// Handler for POST /users
pub(crate) async fn add_user(pool: web::Data<Pool>, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    fn execute_db(pool: web::Data<Pool>, item: web::Json<InputUser>) -> anyhow::Result<usize> {
        let conn = pool.get()?;
        let new_user = NewUser {
            first_name: &item.first_name,
            last_name: &item.last_name,
            email: &item.email,
            created_at: chrono::Local::now().naive_local(),
        };
        let item = insert_into(users).values(&new_user).execute(&conn)?;
        Ok(item)
    }

    Ok(
        web::block(move || execute_db(pool, item))
            .await
            .map(|count| HttpResponse::Created().json(count))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

// Handler for DELETE /users/{id}
pub(crate) async fn delete_user(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    fn execute_db(pool: web::Data<Pool>, user_id: i32) -> anyhow::Result<usize> {
        let conn = pool.get()?;
        let count = delete(users.find(user_id)).execute(&conn)?;
        Ok(count)
    }

    Ok(
        web::block(move || execute_db(db, user_id.into_inner()))
            .await
            .map(|count| HttpResponse::Ok().json(count))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}
