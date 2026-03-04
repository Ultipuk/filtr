use super::*;

impl FilterApp {
    pub(super) fn show_top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button(tr("menu-file"), |ui| {
                    if ui.button(tr("menu-save-params")).clicked() {
                        self.save_compute_parameters(ctx);
                        ui.close();
                    }
                    if ui.button(tr("menu-load-params")).clicked() {
                        self.load_compute_parameters(ctx);
                        ui.close();
                    }
                });
                ui.menu_button(tr("menu-settings"), |ui| {
                    show_scale_submenu(self, ui);
                    show_theme_submenu(self, ctx, ui);
                    show_language_submenu(self, ctx, ui);
                });
                ui.menu_button(tr("menu-view"), |ui| {
                    ui.checkbox(&mut self.left_panel_visible, tr("view-left-panel"));
                    ui.checkbox(&mut self.right_panel_visible, tr("view-right-panel"));
                });
                ui.menu_button(tr("menu-info"), |ui| {
                    if ui.button(tr("menu-about")).clicked() {
                        self.info_modal_visible = true;
                    }
                });
            });
        });
    }
}

fn show_scale_submenu(app: &mut FilterApp, ui: &mut egui::Ui) {
    ui.menu_button(tr("menu-scale"), |ui| {
        ui.radio_value(&mut app.ui_scale, AppScale::Tiny, "100%");
        ui.radio_value(&mut app.ui_scale, AppScale::Small, "125%");
        ui.radio_value(&mut app.ui_scale, AppScale::Normal, "150%");
        ui.radio_value(&mut app.ui_scale, AppScale::Big, "175%");
        ui.radio_value(&mut app.ui_scale, AppScale::Extreme, "200%");
    });
}

fn show_theme_submenu(app: &mut FilterApp, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button(tr("menu-theme"), |ui| {
        if ui
            .radio_value(
                &mut app.theme_preference,
                egui::ThemePreference::System,
                tr("theme-system"),
            )
            .changed()
        {
            app.apply_theme(ctx);
        }
        if ui
            .radio_value(
                &mut app.theme_preference,
                egui::ThemePreference::Light,
                tr("theme-light"),
            )
            .changed()
        {
            app.apply_theme(ctx);
        }
        if ui
            .radio_value(
                &mut app.theme_preference,
                egui::ThemePreference::Dark,
                tr("theme-dark"),
            )
            .changed()
        {
            app.apply_theme(ctx);
        }

        ui.separator();

        if ui
            .radio_value(&mut app.theme, AppTheme::EguiDefault, tr("theme-egui"))
            .changed()
        {
            app.apply_theme(ctx);
        }

        if ui
            .radio_value(&mut app.theme, AppTheme::Breeze, tr("theme-breeze"))
            .changed()
        {
            app.apply_theme(ctx);
        }

        if ui
            .radio_value(&mut app.theme, AppTheme::Solarized, tr("theme-solarized"))
            .changed()
        {
            app.apply_theme(ctx);
        }
    });
}

fn show_language_submenu(app: &mut FilterApp, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button(tr("menu-language"), |ui| {
        let mut changed = false;
        changed |= ui
            .radio_value(&mut app.ui_language, UiLanguage::System, tr("lang-system"))
            .changed();
        changed |= ui
            .radio_value(
                &mut app.ui_language,
                UiLanguage::English,
                tr("lang-english"),
            )
            .changed();
        changed |= ui
            .radio_value(
                &mut app.ui_language,
                UiLanguage::Russian,
                tr("lang-russian"),
            )
            .changed();

        if changed {
            set_language(app.ui_language);
            ctx.request_discard("language changed");
            ctx.request_repaint();
        }
    });
}
