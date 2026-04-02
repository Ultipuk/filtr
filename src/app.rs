use crate::bindings::{Results, norfil2};
use crate::i18n::{UiLanguage, set_language, tr, tr_args};
use crate::models::{
    FilterMode, FilterType, common::CommonParameters, common_filter::CommonFilterParameters,
    cutoff::CutoffParameters, design::DesignParameters, operation::OperationParameters,
    recursive::RecursiveParameters,
};
use crate::parameter_section_row;
use crate::themes::AppTheme;
use egui_plot::{AxisHints, Legend, Line, LineStyle, Plot as EguiPlot, PlotPoints};
use fluent_templates::fluent_bundle::FluentValue;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::Mutex;

mod dialogs;
mod export;
mod panels;
mod tables;
mod top_bar;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq)]
enum AppScale {
    Tiny,
    Small,
    Normal,
    Big,
    Extreme,
}

impl AppScale {
    fn value(&self) -> f32 {
        match self {
            Self::Tiny => 1.0,
            Self::Small => 1.25,
            Self::Normal => 1.5,
            Self::Big => 1.75,
            Self::Extreme => 2.0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Default)]
enum ValueTableType {
    #[default]
    Frequency,
    Impulse,
    Step,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Default)]
enum PlotType {
    #[default]
    FrequencyResponse,
    PhaseResponse,
    ImpulseResponse,
    StepResponse,
    Operation,
}

impl PlotType {
    fn mode(self) -> FilterMode {
        match self {
            Self::FrequencyResponse
            | Self::PhaseResponse
            | Self::ImpulseResponse
            | Self::StepResponse => FilterMode::Design,
            Self::Operation => FilterMode::Operation,
        }
    }

    fn label(self) -> String {
        match self {
            Self::FrequencyResponse => tr("plot-type-frequency"),
            Self::PhaseResponse => tr("plot-type-phase"),
            Self::ImpulseResponse => tr("plot-type-impulse"),
            Self::StepResponse => tr("plot-type-step"),
            Self::Operation => tr("plot-type-operation"),
        }
    }
}

#[derive(Clone, Copy)]
enum OperationSeries {
    Input,
    Output,
    Corrected,
    Reference,
}

impl OperationSeries {
    fn name(self) -> &'static str {
        match self {
            Self::Input => "x(t)",
            Self::Output => "y(t)",
            Self::Corrected => "z(t)",
            Self::Reference => "v(t)",
        }
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
enum LineId {
    DesignFrequency,
    DesignPhase,
    DesignImpulse,
    DesignStep,
    OperationInput,
    OperationOutput,
    OperationCorrected,
    OperationReference,
}

impl LineId {
    fn label(self) -> String {
        match self {
            Self::DesignFrequency => tr("line-design-frequency"),
            Self::DesignPhase => tr("line-design-phase"),
            Self::DesignImpulse => tr("line-design-impulse"),
            Self::DesignStep => tr("line-design-step"),
            Self::OperationInput => tr("line-operation-input"),
            Self::OperationOutput => tr("line-operation-output"),
            Self::OperationCorrected => tr("line-operation-corrected"),
            Self::OperationReference => tr("line-operation-reference"),
        }
    }

    fn default_color_index(self) -> usize {
        match self {
            Self::DesignFrequency => 0,
            Self::DesignPhase => 1,
            Self::DesignImpulse => 2,
            Self::DesignStep => 3,
            Self::OperationInput => 0,
            Self::OperationOutput => 1,
            Self::OperationCorrected => 2,
            Self::OperationReference => 3,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
enum LineDrawStyle {
    Solid,
    Dotted,
    Dashed,
}

impl LineDrawStyle {
    fn label(self) -> String {
        match self {
            Self::Solid => tr("option-line-solid"),
            Self::Dotted => tr("option-line-dotted"),
            Self::Dashed => tr("option-line-dashed"),
        }
    }

    fn to_egui(self, dotted_spacing: f32, dashed_length: f32) -> LineStyle {
        match self {
            Self::Solid => LineStyle::Solid,
            Self::Dotted => LineStyle::Dotted {
                spacing: dotted_spacing,
            },
            Self::Dashed => LineStyle::Dashed {
                length: dashed_length,
            },
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
struct LineVisualSettings {
    color_index: usize,
    style: LineDrawStyle,
    #[serde(default = "default_dotted_spacing")]
    dotted_spacing: f32,
    #[serde(default = "default_dashed_length")]
    dashed_length: f32,
}

impl LineVisualSettings {
    fn default_for(line: LineId) -> Self {
        Self {
            color_index: line.default_color_index(),
            style: LineDrawStyle::Solid,
            dotted_spacing: default_dotted_spacing(),
            dashed_length: default_dashed_length(),
        }
    }
}

const fn default_dotted_spacing() -> f32 {
    10.0
}

const fn default_dashed_length() -> f32 {
    15.0
}

#[derive(Clone, PartialEq)]
struct ParameterSnapshot {
    common_parameters: CommonParameters,
    common_filter_parameters: CommonFilterParameters,
    cutoff_parameters: CutoffParameters,
    design_parameters: DesignParameters,
    operation_parameters: OperationParameters,
    recursive_parameters: RecursiveParameters,
}

impl ParameterSnapshot {
    fn from_app(app: &FilterApp) -> Self {
        Self {
            common_parameters: app.common_parameters.clone(),
            common_filter_parameters: app.common_filter_parameters.clone(),
            cutoff_parameters: app.cutoff_parameters.clone(),
            design_parameters: app.design_parameters.clone(),
            operation_parameters: app.operation_parameters.clone(),
            recursive_parameters: app.recursive_parameters.clone(),
        }
    }

    fn matches_app(&self, app: &FilterApp) -> bool {
        self.common_parameters == app.common_parameters
            && self.common_filter_parameters == app.common_filter_parameters
            && self.cutoff_parameters == app.cutoff_parameters
            && self.design_parameters == app.design_parameters
            && self.operation_parameters == app.operation_parameters
            && self.recursive_parameters == app.recursive_parameters
    }
}

#[derive(Clone, Copy)]
enum SaveNoticeKind {
    Csv,
    Plot,
}

#[derive(Clone, Copy)]
enum PlotScaleAction {
    Downscale,
    Fit,
    Upscale,
}

static SAVE_NOTICES: Lazy<Mutex<Vec<(SaveNoticeKind, String)>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct FilterApp {
    pub common_parameters: CommonParameters,
    pub common_filter_parameters: CommonFilterParameters,
    pub cutoff_parameters: CutoffParameters,
    pub design_parameters: DesignParameters,
    pub operation_parameters: OperationParameters,
    pub recursive_parameters: RecursiveParameters,
    #[serde(skip)]
    pub results: Option<Results>,

    #[serde(skip)]
    left_panel_visible: bool,
    #[serde(skip)]
    right_panel_visible: bool,

    #[serde(skip)]
    info_modal_visible: bool,
    #[serde(skip)]
    reset_modal_visible: bool,
    #[serde(skip)]
    compute_error: Option<String>,
    #[serde(skip)]
    notice_message: Option<String>,
    auto_compute: bool,
    auto_compute_delay_ms: u32,

    ui_scale: AppScale,
    ui_language: UiLanguage,
    theme: AppTheme,
    theme_preference: egui::ThemePreference,
    table_type: ValueTableType,
    plot_type: PlotType,
    plot_line_width: f32,
    plot_show_legend: bool,
    line_visuals: BTreeMap<LineId, LineVisualSettings>,
    show_input_signal: bool,
    show_output_signal: bool,
    show_corrected_signal: bool,
    show_reference_signal: bool,
    #[serde(skip)]
    plot_rect: Option<egui::Rect>,
    #[serde(skip)]
    plot_save_pending: bool,
    #[serde(skip)]
    plot_needs_reset: bool,
    #[serde(skip)]
    plot_scale_action: Option<PlotScaleAction>,
    #[serde(skip)]
    last_seen_snapshot: Option<ParameterSnapshot>,
    #[serde(skip)]
    last_computed_snapshot: Option<ParameterSnapshot>,
    #[serde(skip)]
    input_change_time: f64,
    #[serde(skip)]
    recompute_pending: bool,
}

impl Default for FilterApp {
    fn default() -> Self {
        Self {
            common_parameters: CommonParameters::default(),
            common_filter_parameters: CommonFilterParameters::default(),
            cutoff_parameters: CutoffParameters::default(),
            design_parameters: DesignParameters::default(),
            operation_parameters: OperationParameters::default(),
            recursive_parameters: RecursiveParameters::default(),
            results: None,

            left_panel_visible: true,
            right_panel_visible: true,

            info_modal_visible: false,
            reset_modal_visible: false,
            compute_error: None,
            notice_message: None,
            auto_compute: false,
            auto_compute_delay_ms: 0,

            ui_scale: AppScale::Normal,
            ui_language: UiLanguage::System,
            theme: AppTheme::Breeze,
            theme_preference: egui::ThemePreference::System,
            table_type: ValueTableType::Frequency,
            plot_type: PlotType::FrequencyResponse,
            plot_line_width: 2.0,
            plot_show_legend: true,
            line_visuals: BTreeMap::new(),
            show_input_signal: true,
            show_output_signal: true,
            show_corrected_signal: true,
            show_reference_signal: true,
            plot_rect: None,
            plot_save_pending: false,
            plot_needs_reset: true,
            plot_scale_action: None,
            last_seen_snapshot: None,
            last_computed_snapshot: None,
            input_change_time: 0.0,
            recompute_pending: false,
        }
    }
}

impl PartialEq for FilterApp {
    fn eq(&self, other: &Self) -> bool {
        self.common_parameters == other.common_parameters
            && self.design_parameters == other.design_parameters
            && self.recursive_parameters == other.recursive_parameters
            && self.common_filter_parameters == other.common_filter_parameters
            && self.cutoff_parameters == other.cutoff_parameters
            && self.operation_parameters == other.operation_parameters
    }
}

impl FilterApp {
    const COLOR_PALETTE: [egui::Color32; 5] = [
        egui::Color32::from_rgb(0, 114, 178),
        egui::Color32::from_rgb(230, 159, 0),
        egui::Color32::from_rgb(0, 158, 115),
        egui::Color32::from_rgb(213, 94, 0),
        egui::Color32::from_rgb(86, 180, 233),
    ];

    fn line_color(&self, id: LineId) -> egui::Color32 {
        let settings = self
            .line_visuals
            .get(&id)
            .copied()
            .unwrap_or_else(|| LineVisualSettings::default_for(id));
        let idx = settings
            .color_index
            .min(Self::COLOR_PALETTE.len().saturating_sub(1));
        Self::COLOR_PALETTE[idx]
    }

    fn color_palette() -> Vec<(String, egui::Color32)> {
        vec![
            (tr("color-blue"), Self::COLOR_PALETTE[0]),
            (tr("color-orange"), Self::COLOR_PALETTE[1]),
            (tr("color-green"), Self::COLOR_PALETTE[2]),
            (tr("color-red"), Self::COLOR_PALETTE[3]),
            (tr("color-cyan"), Self::COLOR_PALETTE[4]),
        ]
    }

    fn line_style(&self, id: LineId) -> LineStyle {
        let settings = self
            .line_visuals
            .get(&id)
            .copied()
            .unwrap_or_else(|| LineVisualSettings::default_for(id));
        settings
            .style
            .to_egui(settings.dotted_spacing, settings.dashed_length)
    }

    fn operation_line_ids() -> [LineId; 4] {
        [
            LineId::OperationInput,
            LineId::OperationOutput,
            LineId::OperationCorrected,
            LineId::OperationReference,
        ]
    }

    fn line_visible(&self, id: LineId) -> bool {
        match id {
            LineId::OperationInput => self.show_input_signal,
            LineId::OperationOutput => self.show_output_signal,
            LineId::OperationCorrected => self.show_corrected_signal,
            LineId::OperationReference => self.show_reference_signal,
            _ => true,
        }
    }

    fn line_visible_mut(&mut self, id: LineId) -> Option<&mut bool> {
        match id {
            LineId::OperationInput => Some(&mut self.show_input_signal),
            LineId::OperationOutput => Some(&mut self.show_output_signal),
            LineId::OperationCorrected => Some(&mut self.show_corrected_signal),
            LineId::OperationReference => Some(&mut self.show_reference_signal),
            _ => None,
        }
    }

    fn active_line_ids(&self, mode: FilterMode, filter_type: FilterType) -> Vec<LineId> {
        match mode {
            FilterMode::Design => {
                let id = match self.plot_type {
                    PlotType::FrequencyResponse => LineId::DesignFrequency,
                    PlotType::PhaseResponse => LineId::DesignPhase,
                    PlotType::ImpulseResponse => LineId::DesignImpulse,
                    PlotType::StepResponse => LineId::DesignStep,
                    PlotType::Operation => {
                        if filter_type == FilterType::NonRecursive {
                            LineId::DesignImpulse
                        } else {
                            LineId::DesignStep
                        }
                    }
                };
                vec![id]
            }
            FilterMode::Operation => {
                let mut ids = Vec::new();
                for id in Self::operation_line_ids() {
                    if self.line_visible(id) {
                        ids.push(id);
                    }
                }
                ids
            }
        }
    }

    fn line_customization_ids(&self, mode: FilterMode, filter_type: FilterType) -> Vec<LineId> {
        match mode {
            FilterMode::Design => self.active_line_ids(mode, filter_type),
            FilterMode::Operation => Self::operation_line_ids().to_vec(),
        }
    }

    fn ensure_active_line_settings(&mut self, mode: FilterMode, filter_type: FilterType) {
        for id in self.line_customization_ids(mode, filter_type) {
            self.line_visuals
                .entry(id)
                .or_insert_with(|| LineVisualSettings::default_for(id));
        }
    }

    fn compact_inner_row(
        ui: &mut egui::Ui,
        label: impl Into<egui::WidgetText>,
        content: impl FnOnce(&mut egui::Ui),
    ) -> egui::Response {
        const INNER_LABEL_WIDTH_FRACTION: f32 = 0.25;
        let width = ui.available_width();

        ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center),
            |ui| {
                ui.set_max_height(18.0);

                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::exact(width * INNER_LABEL_WIDTH_FRACTION))
                    .size(egui_extras::Size::remainder())
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(
                                    ui.available_width(),
                                    ui.text_style_height(&egui::TextStyle::Body),
                                ),
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.add(egui::Label::new(label.into()));
                                },
                            );
                        });

                        strip.cell(|ui| content(ui));
                    });
            },
        )
        .response
    }

    fn show_line_customization_list(
        &mut self,
        ui: &mut egui::Ui,
        mode: FilterMode,
        filter_type: FilterType,
    ) {
        let ids = self.line_customization_ids(mode, filter_type);
        if ids.is_empty() {
            return;
        }

        for id in ids {
            egui::Frame::new()
                .fill(ui.style().visuals.faint_bg_color)
                .inner_margin(ui.style().spacing.item_spacing)
                .show(ui, |ui| {
                    let width = ui.available_width();
                    let half_width = (width - ui.spacing().item_spacing.x) * 0.5;
                    let expanded_width = half_width + ui.spacing().item_spacing.x;

                    let section_id = ui.make_persistent_id(("line_customization", id));
                    let mut state =
                        egui::collapsing_header::CollapsingState::load_with_default_open(
                            ui.ctx(),
                            section_id,
                            true,
                        );

                    let mut changed = false;
                    let header_resp = ui.horizontal(|ui| {
                        let prev_spacing = ui.spacing_mut().item_spacing;
                        ui.spacing_mut().item_spacing.x = 0.0;
                        state
                            .show_toggle_button(ui, egui::collapsing_header::paint_default_icon)
                            .on_hover_cursor(egui::CursorIcon::Default);
                        ui.spacing_mut().item_spacing = prev_spacing;

                        if let Some(visible) = self.line_visible_mut(id) {
                            changed |= ui
                                .checkbox(visible, "")
                                .on_hover_cursor(egui::CursorIcon::Default)
                                .changed();
                        }

                        let text_button = egui::Button::new(id.label()).frame(false).small();
                        if ui
                            .add(text_button)
                            .on_hover_cursor(egui::CursorIcon::Default)
                            .clicked()
                        {
                            state.toggle(ui);
                        }
                    });

                    state.show_body_indented(&header_resp.response, ui, |ui| {
                        let entry = self
                            .line_visuals
                            .entry(id)
                            .or_insert_with(|| LineVisualSettings::default_for(id));

                        let mut style_response: Option<egui::Response> = None;
                        let _ = Self::compact_inner_row(
                            ui,
                            tr("label-line-style"),
                            |ui: &mut egui::Ui| {
                                let selected_text = match entry.style {
                                    LineDrawStyle::Solid => LineDrawStyle::Solid.label(),
                                    LineDrawStyle::Dotted => LineDrawStyle::Dotted.label(),
                                    LineDrawStyle::Dashed => LineDrawStyle::Dashed.label(),
                                };
                                style_response = Some(
                                    egui::ComboBox::from_id_salt(("line_style", id))
                                        .width(ui.available_width())
                                        .selected_text(selected_text)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut entry.style,
                                                LineDrawStyle::Solid,
                                                LineDrawStyle::Solid.label(),
                                            );
                                            ui.selectable_value(
                                                &mut entry.style,
                                                LineDrawStyle::Dotted,
                                                LineDrawStyle::Dotted.label(),
                                            );
                                            ui.selectable_value(
                                                &mut entry.style,
                                                LineDrawStyle::Dashed,
                                                LineDrawStyle::Dashed.label(),
                                            );
                                        })
                                        .response,
                                );
                            },
                        );
                        let style_changed = style_response.is_some_and(|r| r.changed());
                        changed |= style_changed;

                        match entry.style {
                            LineDrawStyle::Dotted => {
                                let _ = Self::compact_inner_row(
                                    ui,
                                    tr("label-line-spacing"),
                                    |ui: &mut egui::Ui| {
                                        let before = entry.dotted_spacing;
                                        ui.add(
                                            egui::Slider::new(
                                                &mut entry.dotted_spacing,
                                                2.0..=40.0,
                                            )
                                            .suffix(format!(" {}", tr("unit-px")))
                                            .step_by(1.0),
                                        );
                                        changed |= before != entry.dotted_spacing;
                                    },
                                );
                            }
                            LineDrawStyle::Dashed => {
                                let _ = Self::compact_inner_row(
                                    ui,
                                    tr("label-line-spacing"),
                                    |ui: &mut egui::Ui| {
                                        let before = entry.dashed_length;
                                        ui.add(
                                            egui::Slider::new(&mut entry.dashed_length, 2.0..=40.0)
                                                .suffix(format!(" {}", tr("unit-px")))
                                                .step_by(1.0),
                                        );
                                        changed |= before != entry.dashed_length;
                                    },
                                );
                            }
                            LineDrawStyle::Solid => {}
                        }

                        let _ =
                            Self::compact_inner_row(ui, tr("label-color"), |ui: &mut egui::Ui| {
                                let before = entry.color_index;
                                let palette = Self::color_palette();
                                crate::widgets::parameter_section_color_palette(
                                    ui,
                                    &mut entry.color_index,
                                    &palette,
                                );
                                changed |= before != entry.color_index;
                            });
                    });

                    let openness = state.openness(ui.ctx());
                    ui.set_width(half_width + (1.0 - openness) * expanded_width);

                    if changed {
                        self.plot_needs_reset = true;
                    }
                });
        }
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        self.theme.apply(ctx, self.theme_preference);
    }

    pub fn compute(&mut self) {
        self.compute_internal(false);
    }

    fn compute_internal(&mut self, triggered_by_auto_update: bool) {
        if let Err(error) = self.validate_inputs() {
            if triggered_by_auto_update {
                self.auto_compute = false;
                let message = tr_args(
                    "msg-compute-auto-disabled",
                    &[("error", FluentValue::from(error.as_str()))],
                );
                self.compute_error = None;
                self.notice_message = Some(message);
            } else {
                self.compute_error = None;
                self.notice_message = Some(error);
            }
            self.results = None;
            self.recompute_pending = false;
            self.last_computed_snapshot = Some(ParameterSnapshot::from_app(self));
            return;
        }

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            norfil2(
                &mut self.common_parameters,
                &mut self.design_parameters,
                &mut self.common_filter_parameters,
                &mut self.cutoff_parameters,
                &mut self.recursive_parameters,
                &mut self.operation_parameters,
            )
        }));

        match result {
            Ok(results) => {
                self.results = Some(results);
                self.compute_error = None;
                self.plot_needs_reset = true;
                self.recompute_pending = false;
                self.last_computed_snapshot = Some(ParameterSnapshot::from_app(self));
            }
            Err(_) => {
                self.results = None;
                self.recompute_pending = false;
                self.last_computed_snapshot = Some(ParameterSnapshot::from_app(self));
                self.auto_compute = false;
                self.compute_error = None;
                self.notice_message = Some(tr("msg-compute-failed-auto-disabled"));
            }
        }
    }

    fn track_input_changes(&mut self, ctx: &egui::Context) {
        match &self.last_seen_snapshot {
            Some(previous) if previous.matches_app(self) => {}
            _ => {
                self.last_seen_snapshot = Some(ParameterSnapshot::from_app(self));
                self.recompute_pending = true;
                self.input_change_time = ctx.input(|i| i.time);
            }
        }
    }

    fn maybe_auto_compute(&mut self, ctx: &egui::Context) {
        if !self.auto_compute || !self.recompute_pending {
            return;
        }

        let now = ctx.input(|i| i.time);
        let elapsed_ms = (now - self.input_change_time) * 1000.0;

        if elapsed_ms < self.auto_compute_delay_ms as f64 {
            ctx.request_repaint_after(std::time::Duration::from_millis(
                self.auto_compute_delay_ms as u64,
            ));
            return;
        }

        if self
            .last_computed_snapshot
            .as_ref()
            .is_some_and(|snapshot| snapshot.matches_app(self))
        {
            self.recompute_pending = false;
            return;
        }

        self.compute_internal(true);
    }

    fn validate_inputs(&self) -> Result<(), String> {
        const MAX_INDEX: i32 = 5000;

        if self.common_parameters.get_dt() <= 0.0 {
            return Err(tr("err-dt-positive"));
        }

        if self.common_parameters.mode == FilterMode::Design
            && self.design_parameters.get_df() <= 0.0
        {
            return Err(tr("err-dw-positive"));
        }

        let dt = self.common_parameters.get_dt();
        let df = self.design_parameters.get_df();
        let ws = self.design_parameters.get_os();
        let wp = self.design_parameters.get_of();
        let v1 = self.common_filter_parameters.get_v1();
        let dl = self.common_filter_parameters.get_dl();
        let dm = self.common_filter_parameters.get_dm();

        if !dt.is_finite() || !df.is_finite() || !ws.is_finite() || !wp.is_finite() {
            return Err(tr("err-nan-inf"));
        }
        if !v1.is_finite() || v1 <= 1.0 {
            return Err(tr("err-v1"));
        }
        if !dl.is_finite() || dl <= 0.0 || dl >= 1.0 {
            return Err(tr("err-delta-range"));
        }
        if !dm.is_finite() || dm <= 0.0 || dm >= 1.0 {
            return Err(tr("err-big-delta-range"));
        }

        if self.common_parameters.mode == FilterMode::Design {
            if ws < 0.0 || wp < 0.0 || wp < ws {
                return Err(tr("err-design-freq-order"));
            }

            let kf = (wp / df + 1.1) as i32;
            if kf > MAX_INDEX {
                return Err(tr_args(
                    "err-kf-overflow",
                    &[
                        ("kf", FluentValue::from(kf as f64)),
                        ("max", FluentValue::from(MAX_INDEX as f64)),
                    ],
                ));
            }

            if self.common_parameters.filter_type == FilterType::Recursive {
                let nt = (self.recursive_parameters.get_th() / dt + 1.1) as i32;
                if nt > MAX_INDEX {
                    return Err(tr_args(
                        "err-nt-overflow",
                        &[
                            ("nt", FluentValue::from(nt as f64)),
                            ("max", FluentValue::from(MAX_INDEX as f64)),
                        ],
                    ));
                }
            }
        } else {
            let ne = (self.operation_parameters.get_to() / dt + 1.1) as i32;
            if ne > MAX_INDEX {
                return Err(tr_args(
                    "err-ne-overflow",
                    &[
                        ("ne", FluentValue::from(ne as f64)),
                        ("max", FluentValue::from(MAX_INDEX as f64)),
                    ],
                ));
            }

            if self.common_parameters.filter_type == FilterType::NonRecursive {
                let l = self.operation_parameters.get_l();
                let nx = 3 * l + ne + 1;
                if nx > MAX_INDEX {
                    return Err(tr_args(
                        "err-nx-lto-overflow",
                        &[
                            ("nx", FluentValue::from(nx as f64)),
                            ("max", FluentValue::from(MAX_INDEX as f64)),
                        ],
                    ));
                }
            } else {
                let nf = (self.operation_parameters.get_tp() / dt + 1.1) as i32;
                let lf = (self.operation_parameters.get_tf() / dt + 0.1) as i32;
                let nx = nf + lf.abs() + ne + 1;
                if nx > MAX_INDEX {
                    return Err(tr_args(
                        "err-nx-tptf-overflow",
                        &[
                            ("nx", FluentValue::from(nx as f64)),
                            ("max", FluentValue::from(MAX_INDEX as f64)),
                        ],
                    ));
                }
            }
        }

        Ok(())
    }

    fn show_table_type_selector(&mut self, ui: &mut egui::Ui, filter_type: FilterType) {
        match (filter_type, &self.table_type) {
            (FilterType::NonRecursive, ValueTableType::Step) => {
                self.table_type = ValueTableType::Impulse;
            }
            (FilterType::Recursive, ValueTableType::Impulse) => {
                self.table_type = ValueTableType::Step
            }
            _ => {}
        }

        match filter_type {
            FilterType::NonRecursive => {
                let _ = crate::widgets::parameter_section_combo_box(
                    ui,
                    "value_table_type",
                    tr("label-table"),
                    &mut self.table_type,
                    [
                        (ValueTableType::Frequency, tr("table-type-frequency")),
                        (ValueTableType::Impulse, tr("table-type-impulse")),
                    ],
                );
            }
            FilterType::Recursive => {
                let _ = crate::widgets::parameter_section_combo_box(
                    ui,
                    "value_table_type",
                    tr("label-table"),
                    &mut self.table_type,
                    [
                        (ValueTableType::Frequency, tr("table-type-frequency")),
                        (ValueTableType::Step, tr("table-type-step")),
                    ],
                );
            }
        }
    }

    fn normalize_plot_type(&mut self, mode: FilterMode, filter_type: FilterType) {
        if mode == FilterMode::Design && self.plot_type == PlotType::Operation {
            self.plot_type = PlotType::FrequencyResponse;
            self.plot_needs_reset = true;
        }
        if mode == FilterMode::Operation && self.plot_type.mode() == FilterMode::Design {
            self.plot_type = PlotType::Operation;
            self.plot_needs_reset = true;
        }

        match (filter_type, self.plot_type) {
            (FilterType::NonRecursive, PlotType::StepResponse) => {
                self.plot_type = PlotType::ImpulseResponse;
                self.plot_needs_reset = true;
            }
            (FilterType::Recursive, PlotType::ImpulseResponse) => {
                self.plot_type = PlotType::StepResponse;
                self.plot_needs_reset = true;
            }
            _ => {}
        }
    }

    fn show_result_controls(
        &mut self,
        ui: &mut egui::Ui,
        mode: FilterMode,
        filter_type: FilterType,
        results: &Results,
    ) {
        self.ensure_active_line_settings(mode, filter_type);

        match mode {
            FilterMode::Design => match filter_type {
                FilterType::NonRecursive => {
                    crate::widgets::parameter_section_value(
                        ui,
                        "R",
                        format!("{:.4e}", results.outputs.r),
                    );
                    crate::widgets::parameter_section_value(
                        ui,
                        "L",
                        format!("{}", results.outputs.l),
                    );
                }
                FilterType::Recursive => {
                    crate::widgets::parameter_section_value(
                        ui,
                        "N",
                        format!("{}", results.outputs.n),
                    );
                    crate::widgets::parameter_section_value(
                        ui,
                        "Tφ",
                        format!("{:.4e}", results.outputs.tf),
                    );
                    crate::widgets::parameter_section_value(
                        ui,
                        "Tw",
                        format!("{:.4e}", results.outputs.tp),
                    );
                }
            },
            FilterMode::Operation => {
                crate::widgets::parameter_section_value(
                    ui,
                    "EY",
                    format!("{:.4e}", results.outputs.ey),
                );
                crate::widgets::parameter_section_value(
                    ui,
                    "EX",
                    format!("{:.4e}", results.outputs.ez),
                );
            }
        }

        crate::widgets::ParameterSection::new(tr("section-plot"))
            .with_fill(ui.visuals().panel_fill)
            .show(ui, |ui| {
                ui.indent("plot_controls_indent", |ui| {
                    if crate::widgets::parameter_section_button(ui, tr("button-save-plot"))
                        .clicked()
                    {
                        self.request_plot_save(ui.ctx());
                    }

                    if crate::widgets::parameter_section_checkbox_row(
                        ui,
                        "",
                        &mut self.plot_show_legend,
                        tr("label-legend"),
                    )
                    .changed()
                    {
                        self.plot_needs_reset = true;
                    };

                    let (scale_row, scale_action) =
                        crate::widgets::parameter_section_triple_button_row(
                            ui,
                            tr("label-plot-scale"),
                            "-",
                            tr("plot-tooltip-zoom-out"),
                            "0",
                            tr("plot-tooltip-fit"),
                            "+",
                            tr("plot-tooltip-zoom-in"),
                        );
                    let _ = scale_row;
                    match scale_action {
                        Some(crate::widgets::TripleButtonAction::Left) => {
                            self.plot_scale_action = Some(PlotScaleAction::Downscale);
                        }
                        Some(crate::widgets::TripleButtonAction::Center) => {
                            self.plot_scale_action = Some(PlotScaleAction::Fit);
                        }
                        Some(crate::widgets::TripleButtonAction::Right) => {
                            self.plot_scale_action = Some(PlotScaleAction::Upscale);
                        }
                        None => {}
                    }

                    let _ =
                        parameter_section_row!(ui, tr("label-line-width"), |ui: &mut egui::Ui| {
                            if ui
                                .add(
                                    egui::Slider::new(&mut self.plot_line_width, 1.0..=8.0)
                                        .suffix(format!(" {}", tr("unit-px")))
                                        .step_by(1.0),
                                )
                                .changed()
                            {
                                self.plot_needs_reset = true;
                            }
                        });

                    if mode == FilterMode::Design
                        && crate::widgets::parameter_section_combo_box(
                            ui,
                            "plot_type_selector",
                            tr("label-plot"),
                            &mut self.plot_type,
                            [
                                (
                                    PlotType::FrequencyResponse,
                                    PlotType::FrequencyResponse.label(),
                                ),
                                (PlotType::PhaseResponse, PlotType::PhaseResponse.label()),
                                match filter_type {
                                    FilterType::NonRecursive => (
                                        PlotType::ImpulseResponse,
                                        PlotType::ImpulseResponse.label(),
                                    ),
                                    FilterType::Recursive => {
                                        (PlotType::StepResponse, PlotType::StepResponse.label())
                                    }
                                },
                            ],
                        )
                        .changed()
                    {
                        self.plot_needs_reset = true;
                    }

                    self.show_line_customization_list(ui, mode, filter_type);
                });
            });
    }

    fn draw_results_plot(&mut self, ui: &mut egui::Ui, results: &Results) {
        let mode = results.inputs.common_parameters.mode;
        let dt = results.inputs.common_parameters.get_dt();

        let x_axis_label = match self.plot_type {
            PlotType::FrequencyResponse | PlotType::PhaseResponse => tr("x-axis-frequency"),
            _ => tr("x-axis-time"),
        };

        let mut plot = EguiPlot::new("results_plot")
            .allow_scroll(false)
            .allow_drag(true)
            .show_x(false)
            .show_y(false)
            .height(ui.available_height())
            .custom_x_axes([AxisHints::new_x().label(x_axis_label)].into());

        if self.plot_show_legend {
            plot = plot.legend(Legend::default());
        }
        if self.plot_needs_reset {
            plot = plot.reset();
            self.plot_needs_reset = false;
        }

        let plot_scale_action = self.plot_scale_action.take();
        let plot_response = plot.show(ui, |plot_ui| {
            if let Some(action) = plot_scale_action {
                match action {
                    PlotScaleAction::Downscale => {
                        let center = plot_ui.plot_bounds().center();
                        plot_ui.zoom_bounds(egui::Vec2::new(0.8, 0.8), center);
                    }
                    PlotScaleAction::Fit => {
                        plot_ui.set_auto_bounds([true, true]);
                    }
                    PlotScaleAction::Upscale => {
                        let center = plot_ui.plot_bounds().center();
                        plot_ui.zoom_bounds(egui::Vec2::new(1.25, 1.25), center);
                    }
                }
            }

            match mode {
                FilterMode::Design => match self.plot_type {
                    PlotType::FrequencyResponse => {
                        let lower = results.inputs.design_parameters.get_os();
                        let upper = results.inputs.design_parameters.get_of();
                        let dw = results.inputs.design_parameters.get_df();
                        let rows = ((upper - lower) / dw + 1.0).max(0.0) as usize;
                        let points: PlotPoints<'_> = (0..rows)
                            .map(|row| {
                                let idx = row + 1;
                                [
                                    lower + dw * row as f64,
                                    results.outputs.a.get(idx).copied().unwrap_or_default(),
                                ]
                            })
                            .collect();
                        plot_ui.line(
                            Line::new(PlotType::FrequencyResponse.label(), points)
                                .width(self.plot_line_width)
                                .style(self.line_style(LineId::DesignFrequency))
                                .color(self.line_color(LineId::DesignFrequency)),
                        );
                    }
                    PlotType::PhaseResponse => {
                        let lower = results.inputs.design_parameters.get_os();
                        let upper = results.inputs.design_parameters.get_of();
                        let dw = results.inputs.design_parameters.get_df();
                        let rows = ((upper - lower) / dw + 1.0).max(0.0) as usize;
                        let points: PlotPoints<'_> = (0..rows)
                            .map(|row| {
                                let idx = row + 1;
                                [
                                    lower + dw * row as f64,
                                    results.outputs.f.get(idx).copied().unwrap_or_default(),
                                ]
                            })
                            .collect();
                        plot_ui.line(
                            Line::new(PlotType::PhaseResponse.label(), points)
                                .width(self.plot_line_width)
                                .style(self.line_style(LineId::DesignPhase))
                                .color(self.line_color(LineId::DesignPhase)),
                        );
                    }
                    PlotType::ImpulseResponse => {
                        let l = results.outputs.l.max(0) as usize;
                        let rows = l.saturating_mul(2).saturating_add(1);
                        let points: PlotPoints<'_> = (0..rows)
                            .map(|row| {
                                let idx = row + 1;
                                [
                                    row as f64 * dt,
                                    results.outputs.w.get(idx).copied().unwrap_or_default(),
                                ]
                            })
                            .collect();
                        plot_ui.line(
                            Line::new(PlotType::ImpulseResponse.label(), points)
                                .width(self.plot_line_width)
                                .style(self.line_style(LineId::DesignImpulse))
                                .color(self.line_color(LineId::DesignImpulse)),
                        );
                    }
                    PlotType::StepResponse => {
                        let nt = results.outputs.nt.max(0) as usize;
                        let points: PlotPoints<'_> = (0..nt)
                            .map(|row| {
                                let idx = row + 1;
                                [
                                    row as f64 * dt,
                                    results.outputs.y.get(idx).copied().unwrap_or_default(),
                                ]
                            })
                            .collect();
                        plot_ui.line(
                            Line::new(PlotType::StepResponse.label(), points)
                                .width(self.plot_line_width)
                                .style(self.line_style(LineId::DesignStep))
                                .color(self.line_color(LineId::DesignStep)),
                        );
                    }
                    PlotType::Operation => {}
                },
                FilterMode::Operation => {
                    let nf = results.outputs.nf.max(0) as usize;
                    let ne = results.outputs.ne.max(0) as usize;

                    let mut series: Vec<(OperationSeries, LineId, PlotPoints<'_>)> = Vec::new();

                    if self.show_input_signal {
                        let points: PlotPoints<'_> = (0..=ne)
                            .map(|row| {
                                let k = nf + row;
                                let idx = k + 1;
                                [
                                    k as f64 * dt,
                                    results.outputs.x.get(idx).copied().unwrap_or_default(),
                                ]
                            })
                            .collect();
                        series.push((OperationSeries::Input, LineId::OperationInput, points));
                    }

                    if self.show_output_signal {
                        let points: PlotPoints<'_> =
                            match results.inputs.common_parameters.filter_type {
                                FilterType::NonRecursive => (0..=ne)
                                    .map(|row| {
                                        let idx = row + 1;
                                        let t = (nf + row) as f64 * dt;
                                        [t, results.outputs.y.get(idx).copied().unwrap_or_default()]
                                    })
                                    .collect(),
                                FilterType::Recursive => (0..=ne)
                                    .map(|row| {
                                        let k = nf + row;
                                        let idx = k + 1;
                                        [
                                            k as f64 * dt,
                                            results.outputs.y.get(idx).copied().unwrap_or_default(),
                                        ]
                                    })
                                    .collect(),
                            };
                        series.push((OperationSeries::Output, LineId::OperationOutput, points));
                    }

                    if self.show_corrected_signal {
                        let points: PlotPoints<'_> = (0..ne)
                            .map(|row| {
                                let idx = row + 1;
                                let t = (nf + row) as f64 * dt;
                                [t, results.outputs.z.get(idx).copied().unwrap_or_default()]
                            })
                            .collect();
                        series.push((
                            OperationSeries::Corrected,
                            LineId::OperationCorrected,
                            points,
                        ));
                    }

                    if self.show_reference_signal {
                        let points: PlotPoints<'_> = (0..ne)
                            .map(|row| {
                                let idx = row + 1;
                                let t = (nf + row) as f64 * dt;
                                [t, results.outputs.v.get(idx).copied().unwrap_or_default()]
                            })
                            .collect();
                        series.push((
                            OperationSeries::Reference,
                            LineId::OperationReference,
                            points,
                        ));
                    }

                    for (series_type, line_id, points) in series {
                        plot_ui.line(
                            Line::new(series_type.name(), points)
                                .width(self.plot_line_width)
                                .style(self.line_style(line_id))
                                .color(self.line_color(line_id)),
                        );
                    }
                }
            }
        });

        self.plot_rect = Some(plot_response.response.rect);
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let font_data = include_bytes!("../assets/fonts/STIXTwoText-Regular.ttf");
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "stix2".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(font_data)),
        );

        fonts.families.insert(
            egui::FontFamily::Name("math".into()),
            vec!["stix2".to_owned()],
        );

        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut app: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };
        set_language(app.ui_language);
        app.apply_theme(&cc.egui_ctx);
        app.last_seen_snapshot = Some(ParameterSnapshot::from_app(&app));
        app.last_computed_snapshot = Some(ParameterSnapshot::from_app(&app));
        app
    }
}

impl eframe::App for FilterApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_language(self.ui_language);
        ctx.set_pixels_per_point(self.ui_scale.value());

        self.show_top_bar(ctx);
        self.show_dialogs(ctx);
        self.show_left_value_panel(ctx);
        self.show_right_parameters_panel(ctx);

        self.handle_plot_save(ctx);
        self.drain_save_notices();
        self.track_input_changes(ctx);
        self.maybe_auto_compute(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.results.is_some() {
                let (mode, filter_type) = {
                    let results = self.results.as_ref().expect("results checked");
                    (
                        results.inputs.common_parameters.mode,
                        results.inputs.common_parameters.filter_type,
                    )
                };
                self.normalize_plot_type(mode, filter_type);
                let taken = self.results.take();
                if let Some(results) = taken {
                    self.draw_results_plot(ui, &results);
                    self.results = Some(results);
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label(tr("msg-no-results"));
                });
            }
        });
    }
}
