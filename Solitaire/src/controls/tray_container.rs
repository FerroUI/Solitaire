//! The source TrayContainer's dark inset tray and gold edge, shared by empty piles.
use ferroui::{
    base::media::{Color, Geometry, IBrush, SolidColorBrush, Stretch},
    controls::{Border, Grid, TextBlock, Viewbox, shapes::Path},
    prelude::*,
};
use std::rc::Rc;
pub fn brush(color: &str) -> Rc<dyn IBrush> {
    SolidColorBrush::with_color(Color::parse(color).expect("constant colour")).into()
}
pub fn tray(label: &str, width: f64, height: f64) -> Ref<Border> {
    let border = Border::new();
    border.set_width(width);
    border.set_height(height);
    border.set_corner_radius(CornerRadius::uniform(8.0));
    border.set_background(Some(brush("#18130E07")));
    border.set_border_brush(Some(brush("#609CCDAB")));
    border.set_border_thickness(Thickness::uniform(2.0));
    // The suits and the recycle mark of the empty piles are vector paths, as the card artwork
    // is: no font of every platform has them (the browser has no system font fallback at all),
    // so the desktop and the browser draw the same marks.
    match mark_paths(label) {
        Some(paths) => {
            // The paths keep their places in the 24 by 24 box, which is scaled as a whole.
            let grid = Grid::new();
            grid.set_width(24.0);
            grid.set_height(24.0);
            for (data, filled) in paths {
                let path = Path::new();
                path.set_data(Geometry::parse(data).expect("constant path data"));
                path.set_stretch(Stretch::None);
                if *filled {
                    path.set_fill(Some(brush(MARK_COLOUR)));
                } else {
                    path.set_stroke(Some(brush(MARK_COLOUR)));
                    path.set_stroke_thickness(2.5);
                }
                grid.children().add(path);
            }
            let viewbox = Viewbox::new();
            viewbox.set_width(44.0);
            viewbox.set_height(44.0);
            viewbox.set_horizontal_alignment(HorizontalAlignment::Center);
            viewbox.set_vertical_alignment(VerticalAlignment::Center);
            viewbox.set_child(grid);
            border.set_child(viewbox);
        }
        None => {
            let text = TextBlock::new();
            text.set_text(Some(label));
            text.set_font_size(36.0);
            text.set_foreground(Some(brush(MARK_COLOUR)));
            text.set_horizontal_alignment(HorizontalAlignment::Center);
            text.set_vertical_alignment(VerticalAlignment::Center);
            border.set_child(text);
        }
    }
    border
}

const MARK_COLOUR: &str = "#709CCDAB";

/// The paths of the mark of an empty pile in a 24 by 24 box (each filled, or stroked when
/// `false`), for the labels of the suits and of the stock; `None` for any other label.
pub fn mark_paths(label: &str) -> Option<&'static [(&'static str, bool)]> {
    Some(match label {
        "\u{2665}" => &[(
            "M12 21 C12 21 3 14.5 3 8.5 C3 5.5 5.2 3.5 7.8 3.5 C9.6 3.5 11.1 4.6 12 6.2 C12.9 4.6 14.4 3.5 16.2 3.5 C18.8 3.5 21 5.5 21 8.5 C21 14.5 12 21 12 21 Z",
            true,
        )],
        "\u{2666}" => &[("M12 2 L20 12 L12 22 L4 12 Z", true)],
        "\u{2660}" => &[(
            "M12 2 C12 2 3 9 3 14 C3 16.8 5.1 18.6 7.5 18.6 C9.2 18.6 10.5 17.8 11.2 16.6 C11 18.6 10.2 20.3 8.5 22 L15.5 22 C13.8 20.3 13 18.6 12.8 16.6 C13.5 17.8 14.8 18.6 16.5 18.6 C18.9 18.6 21 16.8 21 14 C21 9 12 2 12 2 Z",
            true,
        )],
        "\u{2663}" => &[(
            "F1 M12 2.5 A4.2 4.2 0 1 1 12 10.9 A4.2 4.2 0 1 1 12 2.5 Z M7.2 9.2 A4.2 4.2 0 1 1 7.2 17.6 A4.2 4.2 0 1 1 7.2 9.2 Z M16.8 9.2 A4.2 4.2 0 1 1 16.8 17.6 A4.2 4.2 0 1 1 16.8 9.2 Z M11 12 C11 17 10 19.8 8.5 22 L15.5 22 C14 19.8 13 17 13 12 Z",
            true,
        )],
        "\u{21bb}" => &[
            ("M18.5 13.5 A7 7 0 1 1 14.6 5.3", false),
            ("M12.2 2.2 L18.6 5.2 L13.6 9.6 Z", true),
        ],
        _ => return None,
    })
}
