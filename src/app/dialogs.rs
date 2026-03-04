use super::*;

impl FilterApp {
    pub(super) fn show_dialogs(&mut self, ctx: &egui::Context) {
        if self.info_modal_visible {
            show_info_modal(ctx, &mut self.info_modal_visible);
        }
        if self.notice_message.is_some() {
            show_notice_modal(ctx, &mut self.notice_message);
        }
        if self.reset_modal_visible && show_reset_modal(ctx, &mut self.reset_modal_visible) {
            *self = Self::default();
            self.auto_compute = false;
        }
    }
}

fn show_notice_modal(ctx: &egui::Context, message: &mut Option<String>) {
    let Some(text) = message.clone() else {
        return;
    };
    let mut close = false;
    let modal = egui::Window::new(tr("msg-notice-title"))
        .id(egui::Id::new("notice_modal"))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(text);
            ui.add_space(6.0);
            ui.vertical_centered(|ui| {
                if ui.add(egui::Button::new(tr("button-ok"))).clicked() {
                    close = true;
                }
            });
        });

    if close || modal.is_none() {
        *message = None;
    }
}

fn show_info_modal(ctx: &egui::Context, visible: &mut bool) {
    let modal = egui::Window::new(tr("menu-about"))
        .id(egui::Id::new("info_modal"))
        .collapsible(false)
        .default_width(420.0)
        .min_width(420.0)
        .resizable(false)
        .show(ctx, |ui| {
            let panel_fill = ui.visuals().faint_bg_color;
            egui::Frame::new()
                .fill(panel_fill)
                .corner_radius(egui::CornerRadius::same(8))
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    ui.label(tr("msg-about-description"));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(tr("msg-about-authors")).strong());
                    ui.label(tr("about-author-1"));
                    ui.label(tr("about-author-2"));
                    ui.label(tr("about-author-3"));
                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(2.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "{} {}",
                            tr("msg-about-version"),
                            env!("CARGO_PKG_VERSION")
                        ))
                        .small()
                        .weak(),
                    );
                });

            ui.add_space(8.0);
            ui.vertical_centered(|ui| {
                if ui.add(egui::Button::new(tr("button-ok"))).clicked() {
                    *visible = false;
                }
            });
        });

    if modal.is_none() {
        *visible = false;
    }
}

fn show_reset_modal(ctx: &egui::Context, visible: &mut bool) -> bool {
    let mut confirmed = false;
    let modal = egui::Window::new(tr("msg-reset-confirm-title"))
        .id(egui::Id::new("reset_modal"))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(tr("msg-reset-confirm-body"));
                let row_height = ui.spacing().interact_size.y;
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), row_height),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        let spacing = ui.style().spacing.item_spacing.x;
                        let button_size = egui::vec2(90.0, row_height);
                        let total_width = button_size.x * 2.0 + spacing;
                        let left_pad = (ui.available_width() - total_width).max(0.0) * 0.5;
                        ui.add_space(left_pad);

                        if ui
                            .add_sized(button_size, egui::Button::new(tr("button-cancel")))
                            .clicked()
                        {
                            *visible = false;
                        }
                        if ui
                            .add_sized(button_size, egui::Button::new(tr("button-ok")))
                            .clicked()
                        {
                            confirmed = true;
                            *visible = false;
                        }
                    },
                );
            });
        });

    if modal.is_none() {
        *visible = false;
    }

    confirmed
}
