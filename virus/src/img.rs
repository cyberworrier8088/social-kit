pub fn gif() -> Result<(), Box<dyn std::error::Error>> {
    let gif = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("giphy.gif");

    if !gif.exists() {
        return Ok(());
    }

    #[cfg(target_os = "windows")]
    {
        let path = gif.display().to_string();
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()?;
    }

    Ok(())
}
