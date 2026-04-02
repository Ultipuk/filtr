use super::*;

impl FilterApp {
    pub(super) fn show_left_value_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("value_table_panel")
            .resizable(true)
            .show_animated(ctx, self.left_panel_visible, |ui| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        if self.results.is_some() {
                            let (mode, filter_type) = {
                                let results = self.results.as_ref().expect("results checked");
                                (
                                    results.inputs.common_parameters.mode,
                                    results.inputs.common_parameters.filter_type,
                                )
                            };

                            if mode == FilterMode::Design {
                                self.show_table_type_selector(ui, filter_type);
                            }

                            let results = self.results.as_ref().expect("results checked");
                            if crate::widgets::parameter_section_button(ui, tr("button-save-csv"))
                                .clicked()
                            {
                                let csv_content = Self::build_csv(results, mode, self.table_type);
                                self.request_csv_save(ui.ctx(), csv_content);
                            }

                            let results = self.results.as_ref().expect("results checked");
                            match mode {
                                FilterMode::Design => match self.table_type {
                                    ValueTableType::Frequency => {
                                        Self::show_frequency_table(ui, results)
                                    }
                                    ValueTableType::Impulse => {
                                        Self::show_impulse_table(ui, results)
                                    }
                                    ValueTableType::Step => Self::show_step_table(ui, results),
                                },
                                FilterMode::Operation => Self::show_operation_table(ui, results),
                            }
                        } else {
                            ui.centered_and_justified(|ui| {
                                ui.label(tr("msg-no-results"));
                            });
                        }
                    },
                );
            });
    }

    pub(super) fn show_right_parameters_panel(&mut self, ctx: &egui::Context) {
        use crate::widgets::{ParameterSection, parameter_menu, parameter_section_button};

        egui::SidePanel::right("parameters_panel")
            .min_width(300.0)
            .show_animated(ctx, self.right_panel_visible, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                        ParameterSection::new(tr("section-filter")).show(ui, |ui| {
                            self.common_parameters.show_filter_mode(ui);
                            self.common_parameters.show_filter_type(ui);
                            self.common_parameters.show_filter_category(ui);
                        });

                        ParameterSection::new(tr("section-main-params")).show(ui, |ui| {
                            self.common_parameters.show_sampling_rate(ui);

                            self.common_filter_parameters.show(ui);

                            self.cutoff_parameters
                                .show(ui, &self.common_parameters.filter_category);
                        });

                        if self.common_parameters.filter_type == FilterType::Recursive
                            && self.common_parameters.mode == FilterMode::Design
                        {
                            ParameterSection::new(tr("section-recursive-params")).show(ui, |ui| {
                                self.recursive_parameters.show(ui);
                            });
                        }

                        match self.common_parameters.mode {
                            FilterMode::Design => {
                                ParameterSection::new(tr("section-design-params")).show(ui, |ui| {
                                    self.design_parameters.show(ui);
                                });
                            }
                            FilterMode::Operation => {
                                ParameterSection::new(tr("section-operation-params")).show(
                                    ui,
                                    |ui| {
                                        self.operation_parameters
                                            .show(ui, self.common_parameters.filter_type);
                                    },
                                );
                            }
                        }

                        ParameterSection::new(tr("section-compute")).show(ui, |ui| {
                            let _ = crate::widgets::parameter_section_checkbox_row(
                                ui,
                                "",
                                &mut self.auto_compute,
                                tr("checkbox-auto-compute"),
                            );
                            parameter_section_button(ui, tr("button-compute"))
                                .clicked()
                                .then(|| self.compute());

                            if *self != Self::default() {
                                parameter_menu::parameter_menu_button(
                                    ui,
                                    tr("button-reset-params"),
                                )
                                .clicked()
                                .then(|| {
                                    self.reset_modal_visible = true;
                                });
                            }
                        });

                        if self.results.is_some() {
                            let (mode, filter_type) = {
                                let results = self.results.as_ref().expect("results checked");
                                (
                                    results.inputs.common_parameters.mode,
                                    results.inputs.common_parameters.filter_type,
                                )
                            };
                            self.normalize_plot_type(mode, filter_type);

                            ParameterSection::new(tr("section-result")).show(ui, |ui| {
                                let taken = self.results.take();
                                if let Some(results) = taken {
                                    self.show_result_controls(ui, mode, filter_type, &results);
                                    self.results = Some(results);
                                }
                            });
                        }
                    });
                });
            });
    }
}
