use crate::components::{
    kv::PostgresKV,
    web_socket::{WSServer, WebSocketReceivers},
};
use actix::Addr;
use actix_web::web::Data;


use crate::components::{
    document::ws_receiver::{make_document_ws_receiver, HttpDocumentCloudPersistence},
    folder::ws_receiver::{make_folder_ws_receiver, HttpFolderCloudPersistence},
    kv::revision_kv::RevisionKVPersistence,
};


use lib_ws::WSChannel;
use sqlx::PgPool;
use std::sync::Arc;
