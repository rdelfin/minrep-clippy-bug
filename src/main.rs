use rusqlite::{named_params, Connection};

fn main() -> anyhow::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute("CREATE table (id INTEGER NOT NULL, name TEXT NOT NULL)", ())?;
    conn.execute("INSERT INTO table (id, name) VALUES (1, \"John\"", ())?;
    conn.execute("INSERT INTO table (id, name) VALUES (2, \"Jane\"", ())?;

    let count: Option<u32> = {
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM table WHERE id = :id")?;
        let c = stmt
            .query_map(
                named_params! {
                    ":id": 1,
                },
                |r| r.get(0),
            )?
            .next()
            .transpose()?;
        c
    };
    println!("Count: {count:?}");

    Ok(())
}
