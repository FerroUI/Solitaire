#![cfg(feature = "headless")]
//! The marks of the empty piles (the suits of the foundations, the recycle mark of the stock)
//! are vector paths, drawn the same on every platform: the browser has no system font that
//! has the suit characters, and a text mark showed the missing-glyph box there.
use ferro_solitaire::{
    app::App,
    controls::tray_container::{mark_paths, tray},
    models::CardSuit,
};
use ferroui::{
    base::{PixelRect, PixelSize, media::imaging::RenderTargetBitmap, threading::Dispatcher},
    controls::TextBlock,
    headless::{FerroHeadlessPlatformExtensions, FerroHeadlessPlatformOptions},
    prelude::*,
    skia::SkiaApplicationExtensions,
};

const SIZE: (i32, i32) = (100, 140);

/// The pixels of an empty pile with `label`, rendered through the real Skia backend.
fn render(label: &str) -> Vec<u8> {
    let window = Window::new();
    window.set_width(SIZE.0 as f64);
    window.set_height(SIZE.1 as f64);
    window.set_content(Some(Control::boxed(tray(label, SIZE.0 as f64, SIZE.1 as f64))));
    window.show();
    window.update_layout();
    Dispatcher::ui_thread().run_jobs(None);
    window.update_layout();
    let bitmap = RenderTargetBitmap::new(PixelSize::new(SIZE.0, SIZE.1));
    bitmap.render(&window.clone().upcast());
    let mut pixels = vec![0u8; (SIZE.0 * SIZE.1 * 4) as usize];
    bitmap.copy_pixels(PixelRect::new(0, 0, SIZE.0, SIZE.1), &mut pixels, SIZE.0 * 4);
    window.close();
    pixels
}

/// How many pixels of the central 44 by 44 box (where the mark is) differ from `empty`.
fn mark_coverage(pixels: &[u8], empty: &[u8]) -> usize {
    let (x0, y0) = ((SIZE.0 - 44) / 2, (SIZE.1 - 44) / 2);
    let mut count = 0;
    for y in y0..y0 + 44 {
        for x in x0..x0 + 44 {
            let i = ((y * SIZE.0 + x) * 4) as usize;
            if pixels[i..i + 4] != empty[i..i + 4] {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn foundation_and_stock_marks_are_vector_paths_that_render_filled_shapes() {
    AppBuilder::configure::<App>()
        .use_headless(FerroHeadlessPlatformOptions { use_headless_drawing: false, ..Default::default() })
        .use_skia()
        .setup_without_starting();

    // Every label the piles use has a path, so no pile mark is ever text.
    for suit in CardSuit::ALL {
        assert!(mark_paths(suit.symbol()).is_some(), "the {suit:?} foundation has no vector mark");
        let pile = tray(suit.symbol(), 100.0, 140.0);
        let has_text = pile.get_visual_descendants().any(|visual| visual.cast::<TextBlock>().is_some());
        assert!(!has_text, "the {suit:?} foundation draws its mark as text");
    }
    assert!(mark_paths("\u{21bb}").is_some(), "the stock has no vector mark");

    // The notdef box of a missing glyph is a thin outline; the suits are solid shapes that
    // cover a large part of their box, the centre included.
    let empty = render("");
    for suit in CardSuit::ALL {
        let pixels = render(suit.symbol());
        let coverage = mark_coverage(&pixels, &empty);
        assert!(coverage > 44 * 44 / 4, "the {suit:?} mark covers only {coverage} pixels");
        let centre = (((SIZE.1 / 2 + 2) * SIZE.0 + SIZE.0 / 2) * 4) as usize;
        assert_ne!(pixels[centre..centre + 4], empty[centre..centre + 4], "the {suit:?} mark is hollow");
    }
    let recycle = render("\u{21bb}");
    assert!(mark_coverage(&recycle, &empty) > 44 * 44 / 12, "the stock mark is not drawn");
}
