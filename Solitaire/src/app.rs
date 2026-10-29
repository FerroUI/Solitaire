//! App and desktop lifetime, following the framework's application port pattern.
use crate::controls::tray_container::brush;
use crate::{models::*, views::casino_view::CasinoView};
use ferroui::{base::styling::Styles, markup_xaml_loader::FerroRuntimeXamlLoader, prelude::*};

#[repr(C)]
pub struct App {
    base: Application,
}
ferro_class!(App: Application);
ferro_impl_classes!(App: FerroObjectImpl);
impl NewApplication for App {
    fn new_application() -> Ref<Self> {
        instantiate(Self {
            base: Application::construct(),
        })
    }
}
impl ApplicationImpl for App {
    fn initialize(this: &Self) {
        Self::parent_initialize(this);
        this.set_name(Some("Ferro Solitaire".into()));
        ferroui::base::platform::register_assets(
            "FerroSolitaire",
            &[
                (
                    "/Assets/QuentellCFBold-Regular.ttf",
                    include_bytes!("assets/QuentellCFBold-Regular.ttf"),
                ),
                (
                    "/Assets/Nunito-Bold-subset.ttf",
                    include_bytes!("assets/Nunito-Bold-subset.ttf"),
                ),
            ],
        );
        ferroui::base::media::FontManager::current().add_font_collection(std::rc::Rc::new(
            ferroui::base::media::fonts::EmbeddedFontCollection::new(
                ferroui::base::utilities::Uri::absolute("fonts:Solitaire").expect("font key"),
                ferroui::base::utilities::Uri::absolute("ferres://FerroSolitaire/Assets")
                    .expect("font source"),
            ),
        ));
        this.styles().add(FluentTheme::new().as_style());
        let styles: Ref<Styles> =
            FerroRuntimeXamlLoader::parse_as(include_str!("styles/Button.xaml"), None)
                .expect("valid button styles");
        this.styles().add(styles);
        let animations: Ref<Styles> =
            FerroRuntimeXamlLoader::parse_as(include_str!("styles/Animations.xaml"), None)
                .expect("valid animation styles");
        this.styles().add(animations);
    }
    fn on_framework_initialization_completed(this: &Self) {
        if let Some(lifetime) = this.application_lifetime() {
            if let Some(desktop) = lifetime.as_classic_desktop_style_application_lifetime() {
                desktop.set_main_window(Some(create_main_window()));
            } else if let Some(single_view) = lifetime.as_single_view_application_lifetime() {
                single_view.set_main_view(Some(create_main_view()));
            }
        }
        Self::parent_on_framework_initialization_completed(this);
    }
}

pub fn create_main_window() -> Ref<Window> {
    let window = Window::new();
    window.set_title(Some("Ferro Solitaire".into()));
    window.set_width(1200.0);
    window.set_height(860.0);
    window.set_min_width(640.0);
    window.set_min_height(480.0);
    window.set_background(Some(brush("#116336")));
    let casino = CasinoView::new(PersistentState::default_path());
    if let Some(kind) = std::env::args().skip(1).find_map(|s| match s.as_str() {
        "--klondike" => Some(GameKind::Klondike),
        "--freecell" => Some(GameKind::FreeCell),
        "--spider" => Some(GameKind::Spider),
        _ => None,
    }) {
        casino.open_game(kind);
    }
    window.set_content(Some(Control::boxed(casino.root.clone())));
    casino.attach_to_window(&window);
    if let Some(ms) = std::env::var("FERROUI_SMOKE_EXIT_MS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
    {
        let window = window.clone();
        ferroui::base::threading::DispatcherTimer::run_once(
            move || window.close(),
            std::time::Duration::from_millis(ms),
            ferroui::base::threading::DispatcherPriority::NORMAL,
        );
    }
    window
}
/// The view of a single-view host (the browser): the casino on the green table.
pub fn create_main_view() -> Ref<Control> {
    let casino = CasinoView::new(PersistentState::default_path());
    let table = Border::new();
    table.set_background(Some(brush("#116336")));
    table.set_child(&casino.root);
    casino.attach_to_view();
    table.upcast()
}
#[cfg(feature = "desktop")]
pub fn run() -> std::process::ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    std::process::ExitCode::from(
        AppBuilder::configure::<App>()
            .use_platform_detect()
            .start_with_classic_desktop_lifetime(&args) as u8,
    )
}
