use super::*;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
static PENDING_IMPORTED_PARAMS: Lazy<Mutex<Vec<Result<ComputeParametersRon, String>>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

#[derive(serde::Serialize, serde::Deserialize)]
struct ComputeParametersRon {
    common_parameters: CommonParameters,
    common_filter_parameters: CommonFilterParameters,
    cutoff_parameters: CutoffParameters,
    design_parameters: DesignParameters,
    operation_parameters: OperationParameters,
    recursive_parameters: RecursiveParameters,
}

impl From<&FilterApp> for ComputeParametersRon {
    fn from(app: &FilterApp) -> Self {
        Self {
            common_parameters: app.common_parameters.clone(),
            common_filter_parameters: app.common_filter_parameters.clone(),
            cutoff_parameters: app.cutoff_parameters.clone(),
            design_parameters: app.design_parameters.clone(),
            operation_parameters: app.operation_parameters.clone(),
            recursive_parameters: app.recursive_parameters.clone(),
        }
    }
}

impl ComputeParametersRon {
    fn apply_to(self, app: &mut FilterApp) {
        app.common_parameters = self.common_parameters;
        app.common_filter_parameters = self.common_filter_parameters;
        app.cutoff_parameters = self.cutoff_parameters;
        app.design_parameters = self.design_parameters;
        app.operation_parameters = self.operation_parameters;
        app.recursive_parameters = self.recursive_parameters;
    }
}

impl FilterApp {
    pub(super) fn request_plot_save(&mut self, ctx: &egui::Context) {
        self.notice_message = None;
        self.plot_save_pending = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
    }

    pub(super) fn request_csv_save(&mut self, ctx: &egui::Context, csv_content: String) {
        self.notice_message = None;
        save_csv_with_native_dialog(ctx.clone(), csv_content);
    }

    pub(super) fn save_compute_parameters(&mut self, ctx: &egui::Context) {
        self.notice_message = None;
        let params = ComputeParametersRon::from(&*self);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = ctx;
            self.notice_message = match save_params_with_native_dialog(&params) {
                Ok(Some(path)) => Some(tr_args(
                    "msg-params-saved",
                    &[("path", FluentValue::from(path.as_str()))],
                )),
                Ok(None) => None,
                Err(error) => Some(format!("{} {error}", tr("msg-error-prefix"))),
            };
        }

        #[cfg(target_arch = "wasm32")]
        {
            save_params_with_native_dialog(ctx.clone(), params);
        }
    }

    pub(super) fn load_compute_parameters(&mut self, ctx: &egui::Context) {
        self.notice_message = None;
        #[cfg(not(target_arch = "wasm32"))]
        {
            match load_params_with_native_dialog() {
                Ok(Some((path, params))) => {
                    params.apply_to(self);
                    self.results = None;
                    self.compute_error = None;
                    self.plot_needs_reset = true;
                    self.recompute_pending = true;
                    self.input_change_time = ctx.input(|i| i.time);
                    self.notice_message = Some(tr_args(
                        "msg-params-loaded",
                        &[("path", FluentValue::from(path.as_str()))],
                    ));
                    ctx.request_repaint();
                }
                Ok(None) => {}
                Err(error) => {
                    self.notice_message = Some(format!("{} {error}", tr("msg-error-prefix")));
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            load_params_with_native_dialog(ctx.clone());
        }
    }

    pub(super) fn handle_plot_save(&mut self, ctx: &egui::Context) {
        if !self.plot_save_pending {
            return;
        }

        let screenshot = ctx.input(|i| {
            i.raw.events.iter().find_map(|event| {
                if let egui::Event::Screenshot { image, .. } = event {
                    return Some(image.clone());
                }
                None
            })
        });

        let Some(screenshot) = screenshot else {
            return;
        };

        self.plot_save_pending = false;
        save_plot_with_native_dialog(ctx.clone(), screenshot, self.plot_rect);
    }

    pub(super) fn drain_save_notices(&mut self) {
        if let Ok(mut notices) = SAVE_NOTICES.lock() {
            for (_kind, message) in notices.drain(..) {
                self.notice_message = Some(message);
            }
        }

        #[cfg(target_arch = "wasm32")]
        if let Ok(mut pending) = PENDING_IMPORTED_PARAMS.lock() {
            for result in pending.drain(..) {
                match result {
                    Ok(params) => {
                        params.apply_to(self);
                        self.results = None;
                        self.compute_error = None;
                        self.plot_needs_reset = true;
                        self.recompute_pending = true;
                        self.input_change_time = 0.0;
                    }
                    Err(message) => {
                        self.notice_message = Some(message);
                    }
                }
            }
        }
    }
}

pub(super) fn push_save_notice(kind: SaveNoticeKind, message: String) {
    if let Ok(mut notices) = SAVE_NOTICES.lock() {
        notices.push((kind, message));
    }
}

fn encode_png(image: &egui::ColorImage) -> Result<Vec<u8>, String> {
    use image::ImageEncoder as _;

    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            image.as_raw(),
            image.width() as u32,
            image.height() as u32,
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| {
            let error = e.to_string();
            tr_args("err-png-prepare", &[("error", FluentValue::from(error))])
        })?;
    Ok(bytes)
}

fn encode_compute_parameters_ron(params: &ComputeParametersRon) -> Result<String, String> {
    ron::ser::to_string_pretty(params, ron::ser::PrettyConfig::new()).map_err(|e| {
        let error = e.to_string();
        tr_args(
            "err-params-serialize",
            &[("error", FluentValue::from(error))],
        )
    })
}

fn decode_compute_parameters_ron(content: &str) -> Result<ComputeParametersRon, String> {
    ron::from_str::<ComputeParametersRon>(content).map_err(|e| {
        let error = e.to_string();
        tr_args("err-params-decode", &[("error", FluentValue::from(error))])
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn save_params_with_native_dialog(params: &ComputeParametersRon) -> Result<Option<String>, String> {
    let file = rfd::FileDialog::new()
        .add_filter("RON", &["ron"])
        .set_file_name("filter_parameters.ron")
        .save_file();

    let Some(mut path) = file else {
        return Ok(None);
    };

    if path.extension().is_none() {
        path.set_extension("ron");
    }

    let encoded = encode_compute_parameters_ron(params)?;
    let path_display = path.display().to_string();

    std::fs::write(&path, encoded).map_err(|e| {
        let error = e.to_string();
        tr_args(
            "err-params-write",
            &[
                ("path", FluentValue::from(path_display.as_str())),
                ("error", FluentValue::from(error)),
            ],
        )
    })?;

    Ok(Some(path_display))
}

#[cfg(target_arch = "wasm32")]
fn save_params_with_native_dialog(ctx: egui::Context, params: ComputeParametersRon) {
    wasm_bindgen_futures::spawn_local(async move {
        let file = rfd::AsyncFileDialog::new()
            .set_file_name("filter_parameters.ron")
            .save_file()
            .await;
        if let Some(handle) = file {
            let encoded = match encode_compute_parameters_ron(&params) {
                Ok(value) => value,
                Err(error) => {
                    push_save_notice(
                        SaveNoticeKind::Csv,
                        format!("{} {error}", tr("msg-error-prefix")),
                    );
                    ctx.request_repaint();
                    return;
                }
            };
            if let Err(e) = handle.write(encoded.as_bytes()).await {
                let error = e.to_string();
                push_save_notice(
                    SaveNoticeKind::Csv,
                    tr_args("err-params-save", &[("error", FluentValue::from(error))]),
                );
            }
        }
        ctx.request_repaint();
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn load_params_with_native_dialog() -> Result<Option<(String, ComputeParametersRon)>, String> {
    let file = rfd::FileDialog::new()
        .add_filter("RON", &["ron"])
        .pick_file();

    let Some(path) = file else {
        return Ok(None);
    };

    let path_display = path.display().to_string();
    let content = std::fs::read_to_string(&path).map_err(|e| {
        let error = e.to_string();
        tr_args(
            "err-params-read",
            &[
                ("path", FluentValue::from(path_display.as_str())),
                ("error", FluentValue::from(error)),
            ],
        )
    })?;

    let decoded = decode_compute_parameters_ron(&content).map_err(|e| {
        tr_args(
            "err-params-invalid-ron",
            &[
                ("path", FluentValue::from(path_display.as_str())),
                ("error", FluentValue::from(e)),
            ],
        )
    })?;

    Ok(Some((path_display, decoded)))
}

#[cfg(target_arch = "wasm32")]
fn load_params_with_native_dialog(ctx: egui::Context) {
    wasm_bindgen_futures::spawn_local(async move {
        let file = rfd::AsyncFileDialog::new()
            .add_filter("RON", &["ron"])
            .pick_file()
            .await;
        if let Some(handle) = file {
            let bytes = handle.read().await;
            let decoded = String::from_utf8(bytes)
                .map_err(|e| {
                    let error = e.to_string();
                    tr_args(
                        "err-params-encoding",
                        &[("error", FluentValue::from(error))],
                    )
                })
                .and_then(|content| decode_compute_parameters_ron(&content));

            if let Ok(mut pending) = PENDING_IMPORTED_PARAMS.lock() {
                pending.push(decoded.map_err(|e| format!("{} {e}", tr("msg-error-prefix"))));
            }
        }
        ctx.request_repaint();
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn save_csv_with_native_dialog(_ctx: egui::Context, csv: String) {
    use std::io::Write as _;

    let file = rfd::FileDialog::new()
        .add_filter("CSV", &["csv"])
        .set_file_name("filter_table.csv")
        .save_file();

    let message = match file {
        Some(mut path) => {
            if path.extension().is_none() {
                path.set_extension("csv");
            }
            let path_display = path.display().to_string();
            match std::fs::File::create(&path).and_then(|mut f| f.write_all(csv.as_bytes())) {
                Ok(()) => tr_args(
                    "msg-csv-saved",
                    &[("path", FluentValue::from(path_display.as_str()))],
                ),
                Err(e) => {
                    let error = e.to_string();
                    tr_args(
                        "err-csv-write",
                        &[
                            ("path", FluentValue::from(path_display.as_str())),
                            ("error", FluentValue::from(error)),
                        ],
                    )
                }
            }
        }
        None => tr("msg-csv-cancelled"),
    };

    push_save_notice(SaveNoticeKind::Csv, message);
}

#[cfg(target_arch = "wasm32")]
fn save_csv_with_native_dialog(ctx: egui::Context, csv: String) {
    wasm_bindgen_futures::spawn_local(async move {
        let file = rfd::AsyncFileDialog::new()
            .set_file_name("filter_table.csv")
            .save_file()
            .await;
        if let Some(handle) = file {
            if let Err(e) = handle.write(csv.as_bytes()).await {
                let error = e.to_string();
                push_save_notice(
                    SaveNoticeKind::Csv,
                    tr_args(
                        "err-csv-write-short",
                        &[("error", FluentValue::from(error))],
                    ),
                );
            }
        }
        ctx.request_repaint();
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn save_plot_with_native_dialog(
    ctx: egui::Context,
    screenshot: Arc<egui::ColorImage>,
    plot_rect: Option<egui::Rect>,
) {
    let message = match save_plot_with_native_dialog_impl(&ctx, screenshot, plot_rect) {
        Ok(msg) => msg,
        Err(e) => format!("{} {e}", tr("msg-error-prefix")),
    };
    push_save_notice(SaveNoticeKind::Plot, message);
}

#[cfg(not(target_arch = "wasm32"))]
fn save_plot_with_native_dialog_impl(
    ctx: &egui::Context,
    screenshot: Arc<egui::ColorImage>,
    plot_rect: Option<egui::Rect>,
) -> Result<String, String> {
    let plot_rect = plot_rect.ok_or_else(|| tr("err-plot-region"))?;
    let pixels_per_point = ctx.pixels_per_point();
    let plot_image = screenshot.region(&plot_rect, Some(pixels_per_point));
    let png_bytes = encode_png(&plot_image)?;

    let file = rfd::FileDialog::new()
        .add_filter("PNG", &["png"])
        .set_file_name("filter_plot.png")
        .save_file();
    let Some(mut path) = file else {
        return Ok(tr("msg-png-cancelled"));
    };
    if path.extension().is_none() {
        path.set_extension("png");
    }
    let path_display = path.display().to_string();
    std::fs::write(&path, png_bytes).map_err(|e| {
        let error = e.to_string();
        tr_args(
            "err-png-save",
            &[
                ("path", FluentValue::from(path_display.as_str())),
                ("error", FluentValue::from(error)),
            ],
        )
    })?;
    Ok(tr_args(
        "msg-plot-saved",
        &[("path", FluentValue::from(path_display.as_str()))],
    ))
}

#[cfg(target_arch = "wasm32")]
fn save_plot_with_native_dialog(
    ctx: egui::Context,
    screenshot: Arc<egui::ColorImage>,
    plot_rect: Option<egui::Rect>,
) {
    let Some(plot_rect) = plot_rect else {
        push_save_notice(SaveNoticeKind::Plot, tr("err-plot-region-short"));
        return;
    };

    let pixels_per_point = ctx.pixels_per_point();
    let plot_image = screenshot.region(&plot_rect, Some(pixels_per_point));
    let png_bytes = match encode_png(&plot_image) {
        Ok(bytes) => bytes,
        Err(error) => {
            push_save_notice(
                SaveNoticeKind::Plot,
                format!("{} {error}", tr("msg-error-prefix")),
            );
            return;
        }
    };

    wasm_bindgen_futures::spawn_local(async move {
        let file = rfd::AsyncFileDialog::new()
            .set_file_name("filter_plot.png")
            .save_file()
            .await;
        if let Some(handle) = file {
            if let Err(e) = handle.write(&png_bytes).await {
                let error = e.to_string();
                push_save_notice(
                    SaveNoticeKind::Plot,
                    tr_args("err-png-save-short", &[("error", FluentValue::from(error))]),
                );
            }
        }
        ctx.request_repaint();
    });
}
