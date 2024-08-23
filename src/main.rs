use std::io::Result;

mod app;
mod test;
mod tui;

fn main() -> Result<()> {
    let app = app::App::new();
    let mut tui = tui::Tui::init(app);
    while !tui.get_app().get_exit() {
        tui.render();
        let _ = tui.handle_key_events();
    }
    let _ = tui.restore();
    Ok(())
}
