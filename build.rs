#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn add_icon() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("copper.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn empty() {
    
}

fn main() {
    #[cfg(windows)]
    add_icon();
    #[cfg(not(windows))]
    empty();
}
