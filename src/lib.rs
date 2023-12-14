use actix_web::{dev::Server, middleware, web, App, HttpResponse, HttpServer};
use sqlx::PgPool;

pub mod configuration;
mod routes;
pub mod startup;
pub mod telemetry;
