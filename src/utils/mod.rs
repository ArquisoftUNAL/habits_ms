use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use warp::Filter;

pub fn with_db(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> impl Filter<Extract = (Pool<ConnectionManager<PgConnection>>,), Error = std::convert::Infallible>
       + Clone {
    warp::any().map(move || pool.clone())
}
