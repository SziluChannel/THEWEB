use diesel::{pg::PgConnection, prelude::*};

fn establish_connection() -> PgConnection {
    let database_url = "postgres://appuser:appuser@localhost/theweb";
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to url: {}", database_url))
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
