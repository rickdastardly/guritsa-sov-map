// Embeds the icon + version info into the Windows .exe (no-op elsewhere).
fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Gurista Sov Map");
        res.set("FileDescription", "Gurista Sov Map - EVE Online sovereignty map");
        res.set("CompanyName", "gurista.co.uk");
        res.set("LegalCopyright", "EVE Online and the EVE logo are the registered trademarks of CCP hf.");
        if let Err(e) = res.compile() {
            println!("cargo:warning=could not embed icon: {e}");
        }
    }
    println!("cargo:rerun-if-changed=assets/icon.ico");
}
