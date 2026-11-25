fn main() {
    // Only run on Windows
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        // Path to the .ico file (relative to project root)
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Valoroulette");
        res.set("FileDescription", "Valoroulette Rust");
        res.set("LegalCopyright", "MIT License");
        res.compile().expect("Failed to compile Windows resources");
    }
}