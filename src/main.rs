use anyhow::Result;
use tectyl::app::App;

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run()?;

    Ok(())
}
