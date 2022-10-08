use std::{env, fs, io, path::Path};
use winres::WindowsResource;

fn main() -> io::Result<()> {
    //create icon
    #[cfg(windows)]
    {
        WindowsResource::new().set_icon("hidden_tb.ico").compile()?;
    }

    copy_icon_to_target();
    Ok(())
}

fn copy_icon_to_target() {
    let mut source = env::var("CARGO_MANIFEST_DIR").unwrap();
    source.push_str("\\hidden_tb.ico");
    let source_directory = Path::new(source.as_str());

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut target_directory = Path::new(out_dir.as_str());
    target_directory = target_directory.parent().unwrap();
    target_directory = target_directory.parent().unwrap();
    target_directory = target_directory.parent().unwrap();
    let target_string = format!(
        "{}/{}",
        target_directory.to_str().unwrap(),
        "\\hidden_tb.ico"
    );

    dbg!(&source_directory);
    dbg!(&target_string);

    fs::copy(source_directory.as_os_str(), target_string).expect("move icon to target failed");
}
