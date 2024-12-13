mod editor;
mod utils;

use editor::text_screen::TextScreen;
use cursive::Cursive;
use cursive::CursiveExt;
use cursive::theme::{Theme, BorderStyle, PaletteColor, Color, BaseColor};
use std::env;

fn main() {
    let mut siv = Cursive::default();
    
    let mut theme = Theme::default();
    theme.shadow = false;
    theme.borders = BorderStyle::Simple;
    theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Light(BaseColor::White);
    theme.palette[PaletteColor::Secondary] = Color::Light(BaseColor::Blue);
    theme.palette[PaletteColor::Tertiary] = Color::Light(BaseColor::Green);
    theme.palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::Green);
    theme.palette[PaletteColor::TitleSecondary] = Color::Light(BaseColor::Blue);
    
    siv.set_theme(theme);

    let args: Vec<String> = env::args().collect();
    let mut text_screen = TextScreen::default();

    if args.len() > 1 {
        let filename = &args[1];
        text_screen.load_file(filename);
    }

    siv.add_layer(text_screen);
    siv.run();
}

