use anyhow::Result;
use goffin::tui;

#[tokio::main]
async fn main() -> Result<()> {
    tui::run().await?;
    Ok(())
}
