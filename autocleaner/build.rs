/// Build script entry point for embedding a Windows application icon.
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("../resources/icon.ico");
    res.compile().unwrap();
}
