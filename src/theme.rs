use gio::prelude::SettingsExtManual;

pub fn get() -> iced::Theme {
    let settings = gio::Settings::new("org.gnome.desktop.interface");
    let theme: String = settings.get("color-scheme");

    dbg!(&theme);

    match &theme as &str {
        "prefer-light" | "default" => iced::Theme::GruvboxLight,
        "prefer-dark" => iced::Theme::GruvboxDark,
        _ => iced::Theme::default(),
    }
}

