use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use sqlx::PgPool;

pub mod configuration;
mod routes;
pub mod startup;
