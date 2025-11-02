pub mod http;
pub mod nosql;
pub mod sql;

pub use http::check_http;
pub use nosql::check_nosql;
pub use sql::check_sql;
