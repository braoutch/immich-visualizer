#[cfg_attr(target_arch = "wasm32",
           wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() {
    // slint_build::compile("ui/appwindow.slint").unwrap();

    let config =
    slint_build::CompilerConfiguration::new()
    .with_style("cosmic-dark".into());
slint_build::compile_with_config("ui/appwindow.slint", config).unwrap();
}