fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    let mut resource = winres::WindowsResource::new();
    resource.set_icon("packaging/app_icons/icon.ico");

    if std::env::var("TARGET")
        .map(|target| target.ends_with("windows-gnu"))
        .unwrap_or(false)
    {
        resource
            .set_windres_path("x86_64-w64-mingw32-windres")
            .set_ar_path("x86_64-w64-mingw32-ar");
    }

    resource
        .compile()
        .expect("failed to embed Windows application icon");

    if std::env::var("TARGET")
        .map(|target| target.ends_with("windows-gnu"))
        .unwrap_or(false)
    {
        let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is not set");
        println!("cargo:rustc-link-arg-bin=telemetry_lmu={out_dir}/resource.o");
    }
}
