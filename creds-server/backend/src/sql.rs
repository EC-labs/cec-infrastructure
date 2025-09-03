use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::{rusqlite::params, SqliteConnectionManager};

pub fn init_sql(pool: Pool<SqliteConnectionManager>) -> Result<()> {
    let res = pool.get()?
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS user (
                client_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                email TEXT UNIQUE NOT NULL,
                group_id INTEGER,
                role TEXT NOT NULL
            );
            INSERT INTO user(email, role) VALUES ('d.landau@uu.nl', 'admin') ON CONFLICT DO NOTHING;
            ",
        )?;
    Ok(())
}
