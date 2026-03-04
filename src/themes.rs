use egui::{Color32, Stroke, Theme, ThemePreference, Visuals};

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppTheme {
    EguiDefault,
    #[default]
    Breeze,
    Solarized,
}

impl AppTheme {
    pub fn apply(self, ctx: &egui::Context, theme_preference: ThemePreference) {
        ctx.set_theme(theme_preference);
        ctx.send_viewport_cmd(egui::ViewportCommand::SetTheme(match theme_preference {
            ThemePreference::System => egui::SystemTheme::SystemDefault,
            ThemePreference::Light => egui::SystemTheme::Light,
            ThemePreference::Dark => egui::SystemTheme::Dark,
        }));

        match self {
            Self::EguiDefault => {
                ctx.set_visuals_of(Theme::Light, Visuals::light());
                ctx.set_visuals_of(Theme::Dark, Visuals::dark());
            }
            Self::Breeze => {
                ctx.set_visuals_of(Theme::Light, breeze_light_theme());
                ctx.set_visuals_of(Theme::Dark, breeze_dark_theme());
            }
            Self::Solarized => {
                ctx.set_visuals_of(Theme::Light, solarized_light_theme());
                ctx.set_visuals_of(Theme::Dark, solarized_dark_theme());
            }
        }
    }
}

fn darker(color: Color32, amount: u8) -> Color32 {
    Color32::from_rgba_premultiplied(
        color.r().saturating_sub(amount),
        color.g().saturating_sub(amount),
        color.b().saturating_sub(amount),
        color.a(),
    )
}

fn lighter(color: Color32, amount: u8) -> Color32 {
    Color32::from_rgba_premultiplied(
        color.r().saturating_add(amount),
        color.g().saturating_add(amount),
        color.b().saturating_add(amount),
        color.a(),
    )
}

fn breeze_light_theme() -> Visuals {
    let mut visuals = Visuals::light();

    let window_bg = Color32::from_rgb(252, 252, 252);
    let panel_bg = Color32::from_rgb(239, 240, 241);
    let active_blue = Color32::from_rgb(61, 174, 233);
    let text = Color32::from_rgb(35, 38, 41);
    let stroke = Color32::from_rgb(189, 195, 199);
    let subtle = Color32::from_rgb(223, 225, 227);

    visuals.window_fill = window_bg;
    visuals.panel_fill = panel_bg;
    visuals.faint_bg_color = subtle;
    visuals.extreme_bg_color = Color32::from_rgb(255, 255, 255);
    visuals.code_bg_color = Color32::from_rgb(247, 247, 247);
    visuals.hyperlink_color = active_blue;
    visuals.selection.bg_fill = active_blue;
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.noninteractive.fg_stroke.color = text;
    visuals.widgets.noninteractive.weak_bg_fill = darker(panel_bg, 10);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, stroke);
    visuals.widgets.noninteractive.bg_fill = panel_bg;
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(200, 202, 204);
    visuals.widgets.inactive.weak_bg_fill = lighter(visuals.widgets.inactive.bg_fill, 10);
    visuals.widgets.inactive.fg_stroke.color = text;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, stroke);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(226, 241, 251);
    visuals.widgets.hovered.weak_bg_fill = darker(visuals.widgets.hovered.bg_fill, 10);
    visuals.widgets.hovered.fg_stroke.color = text;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, active_blue);
    visuals.widgets.active.bg_fill = Color32::from_rgb(208, 234, 249);
    visuals.widgets.active.weak_bg_fill = darker(visuals.widgets.active.bg_fill, 10);
    visuals.widgets.active.fg_stroke.color = text;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, active_blue);
    visuals.widgets.open.bg_fill = Color32::from_rgb(231, 244, 252);
    visuals.widgets.open.weak_bg_fill = darker(visuals.widgets.open.bg_fill, 10);
    visuals.widgets.open.fg_stroke.color = text;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, active_blue);

    visuals
}

fn breeze_dark_theme() -> Visuals {
    let mut visuals = Visuals::dark();

    let window_bg = Color32::from_rgb(31, 34, 38);
    let panel_bg = Color32::from_rgb(40, 44, 49);
    let active_blue = Color32::from_rgb(61, 174, 233);
    let text = Color32::from_rgb(239, 240, 241);
    let stroke = Color32::from_rgb(88, 95, 103);

    visuals.window_fill = window_bg;
    visuals.panel_fill = panel_bg;
    visuals.faint_bg_color = Color32::from_rgb(52, 58, 64);
    visuals.extreme_bg_color = Color32::from_rgb(25, 28, 32);
    visuals.code_bg_color = Color32::from_rgb(36, 40, 45);
    visuals.hyperlink_color = active_blue;
    visuals.selection.bg_fill = active_blue;
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.noninteractive.fg_stroke.color = text;
    visuals.widgets.noninteractive.weak_bg_fill = lighter(panel_bg, 10);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, stroke);
    visuals.widgets.noninteractive.bg_fill = panel_bg;
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(84, 89, 95);
    visuals.widgets.inactive.weak_bg_fill = darker(visuals.widgets.inactive.bg_fill, 10);
    visuals.widgets.inactive.bg_fill = visuals.widgets.inactive.weak_bg_fill;
    visuals.widgets.inactive.fg_stroke.color = text;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, stroke);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(56, 67, 77);
    visuals.widgets.hovered.weak_bg_fill = lighter(visuals.widgets.hovered.bg_fill, 10);
    visuals.widgets.hovered.bg_fill = visuals.widgets.hovered.weak_bg_fill;
    visuals.widgets.hovered.fg_stroke.color = text;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, active_blue);
    visuals.widgets.active.bg_fill = Color32::from_rgb(63, 76, 88);
    visuals.widgets.active.weak_bg_fill = lighter(visuals.widgets.active.bg_fill, 10);
    visuals.widgets.active.bg_fill = visuals.widgets.active.weak_bg_fill;
    visuals.widgets.active.fg_stroke.color = text;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, active_blue);
    visuals.widgets.open.bg_fill = Color32::from_rgb(58, 71, 82);
    visuals.widgets.open.weak_bg_fill = lighter(visuals.widgets.open.bg_fill, 10);
    visuals.widgets.open.bg_fill = visuals.widgets.open.weak_bg_fill;
    visuals.widgets.open.fg_stroke.color = text;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, active_blue);

    visuals
}

fn solarized_light_theme() -> Visuals {
    let mut visuals = Visuals::light();

    let base3 = Color32::from_rgb(253, 246, 227);
    let base2 = Color32::from_rgb(238, 232, 213);
    let base1 = Color32::from_rgb(147, 161, 161);
    let base01 = Color32::from_rgb(88, 110, 117);
    let base02 = Color32::from_rgb(7, 54, 66);
    let blue = Color32::from_rgb(38, 139, 210);
    let orange = Color32::from_rgb(203, 75, 22);

    visuals.window_fill = base3;
    visuals.panel_fill = base2;
    visuals.faint_bg_color = Color32::from_rgb(246, 240, 220);
    visuals.extreme_bg_color = Color32::from_rgb(255, 251, 240);
    visuals.code_bg_color = Color32::from_rgb(245, 239, 218);
    visuals.hyperlink_color = blue;
    visuals.selection.bg_fill = blue;
    visuals.selection.stroke = Stroke::new(1.0, base3);
    visuals.widgets.noninteractive.fg_stroke.color = base02;
    visuals.widgets.noninteractive.weak_bg_fill = darker(base2, 10);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, base1);
    visuals.widgets.noninteractive.bg_fill = base2;
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(212, 206, 186);
    visuals.widgets.inactive.weak_bg_fill = lighter(visuals.widgets.inactive.bg_fill, 15);
    visuals.widgets.inactive.fg_stroke.color = base02;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, base1);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(232, 238, 241);
    visuals.widgets.hovered.weak_bg_fill = darker(visuals.widgets.hovered.bg_fill, 10);
    visuals.widgets.hovered.fg_stroke.color = base02;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, blue);
    visuals.widgets.active.bg_fill = Color32::from_rgb(225, 234, 238);
    visuals.widgets.active.weak_bg_fill = darker(visuals.widgets.active.bg_fill, 10);
    visuals.widgets.active.fg_stroke.color = base02;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, orange);
    visuals.widgets.open.bg_fill = Color32::from_rgb(230, 236, 236);
    visuals.widgets.open.weak_bg_fill = darker(visuals.widgets.open.bg_fill, 10);
    visuals.widgets.open.fg_stroke.color = base02;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, blue);
    visuals.override_text_color = Some(base01);

    visuals
}

fn solarized_dark_theme() -> Visuals {
    let mut visuals = Visuals::dark();

    let base03 = Color32::from_rgb(0, 43, 54);
    let base02 = Color32::from_rgb(7, 54, 66);
    let base01 = Color32::from_rgb(88, 110, 117);
    let base0 = Color32::from_rgb(131, 148, 150);
    let base2 = Color32::from_rgb(238, 232, 213);
    let yellow = Color32::from_rgb(181, 137, 0);
    let blue = Color32::from_rgb(38, 139, 210);

    visuals.window_fill = base03;
    visuals.panel_fill = base02;
    visuals.faint_bg_color = Color32::from_rgb(13, 62, 74);
    visuals.extreme_bg_color = Color32::from_rgb(0, 36, 46);
    visuals.code_bg_color = Color32::from_rgb(11, 50, 61);
    visuals.hyperlink_color = blue;
    visuals.selection.bg_fill = blue;
    visuals.selection.stroke = Stroke::new(1.0, base2);
    visuals.widgets.noninteractive.fg_stroke.color = base0;
    visuals.widgets.noninteractive.weak_bg_fill = lighter(base02, 10);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, base01);
    visuals.widgets.noninteractive.bg_fill = base02;
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(19, 66, 78);
    visuals.widgets.inactive.weak_bg_fill = lighter(visuals.widgets.inactive.bg_fill, 10);
    visuals.widgets.inactive.bg_fill = visuals.widgets.inactive.weak_bg_fill;
    visuals.widgets.inactive.fg_stroke.color = base0;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, base01);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(21, 76, 90);
    visuals.widgets.hovered.weak_bg_fill = lighter(visuals.widgets.hovered.bg_fill, 10);
    visuals.widgets.hovered.bg_fill = visuals.widgets.hovered.weak_bg_fill;
    visuals.widgets.hovered.fg_stroke.color = base2;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, blue);
    visuals.widgets.active.bg_fill = Color32::from_rgb(28, 87, 102);
    visuals.widgets.active.weak_bg_fill = lighter(visuals.widgets.active.bg_fill, 10);
    visuals.widgets.active.bg_fill = visuals.widgets.active.weak_bg_fill;
    visuals.widgets.active.fg_stroke.color = base2;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, yellow);
    visuals.widgets.open.bg_fill = Color32::from_rgb(24, 82, 96);
    visuals.widgets.open.weak_bg_fill = lighter(visuals.widgets.open.bg_fill, 10);
    visuals.widgets.open.bg_fill = visuals.widgets.open.weak_bg_fill;
    visuals.widgets.open.fg_stroke.color = base2;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, blue);

    visuals
}
