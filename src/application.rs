use actix::Actor;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{dev::Server, middleware, web, web::Data, App, HttpServer, Scope};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::TcpListener, time::Duration};
use tokio::time::interval;
