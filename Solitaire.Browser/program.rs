//! The browser host of Ferro Solitaire.
//!
//! The host page (`wwwroot/main.js`) creates the WebAssembly module, hands it to the script side of
//! the platform and calls [`run_main`] with the address of the page; the game starts in the element
//! `out` of the page as a single view (the casino on its green table) and the function returns,
//! leaving the page to drive it.
//!
//! The browser has no system fonts: the embedded Inter font is the default family, beside the fonts
//! the game embeds itself. Saved games live in the memory file system of the module for the time
//! the page is open.
//!
//! The query string selects the renderer: `?RenderingMode=Software2D` draws without WebGL.

#![cfg_attr(target_os = "emscripten", no_main)]

#[cfg(target_os = "emscripten")]
use wasm_bindgen::prelude::*;

#[cfg(target_os = "emscripten")]
#[wasm_bindgen(js_name = runMain)]
pub fn run_main(href: &str) {
    use ferroui_base::logging::LogEventLevel;
    use ferroui_base::media::FontManagerOptions;
    use ferroui_browser::{BrowserAppBuilder, BrowserPlatformOptions, BrowserRenderingMode};
    use ferroui_controls::AppBuilder;
    use ferroui_fonts_inter::AppBuilderExtension;
    use std::rc::Rc;

    let mut options = BrowserPlatformOptions::default();
    if href.contains("RenderingMode=Software2D") {
        options.rendering_mode = vec![BrowserRenderingMode::Software2D];
    }
    AppBuilder::configure::<ferro_solitaire::app::App>()
        .with_inter_font()
        .with(Rc::new(FontManagerOptions {
            default_family_name: Some("fonts:Inter#Inter".to_owned()),
            ..Default::default()
        }))
        .log_to_text_writer(std::io::stdout(), LogEventLevel::Warning, &[])
        .start_browser_app("out", Some(options));
}

#[cfg(not(target_os = "emscripten"))]
fn main() {
    println!(
        "Build the browser site with: FERROUI_BROWSER_WORKSPACE=$PWD FerroUI/scripts/build-browser.sh ferro-solitaire-browser"
    );
}
