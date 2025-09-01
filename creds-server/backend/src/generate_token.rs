use anyhow::Result;

mod jwt;

use jwt::Claims;

fn main() -> Result<()> {
    let claims = Claims::new("n.saurabh@uu.nl".into(), "lecturer".into());
    println!("{}", jwt::encode(&claims)?);
    Ok(())
}
