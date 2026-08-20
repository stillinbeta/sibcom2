use anyhow::Result;

use updater::sourcehut::Sourcehut;

fn main() -> Result<()> {
    let creds = std::env::var("SOURCEHUT_TOKEN")?;

    let client = Sourcehut::new(&creds)?;

    println!("{:?}", client.get_push()?);
    Ok(())
}
