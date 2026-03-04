use super::*;
use egui_extras::{Column, TableBuilder};

type RowBuilder<'a> = dyn Fn(usize) -> Vec<String> + 'a;

struct TableSchema<'a> {
    headers: Vec<String>,
    rows: usize,
    row_builder: Box<RowBuilder<'a>>,
}

impl FilterApp {
    pub(super) fn show_frequency_table(ui: &mut egui::Ui, results: &Results) {
        let inputs = &results.inputs;
        let outputs = &results.outputs;

        let upper = inputs.design_parameters.get_of();
        let lower = inputs.design_parameters.get_os();
        let dw = inputs.design_parameters.get_df();
        let total_rows = ((upper - lower) / dw + 1.0).max(0.0) as usize;

        let schema = TableSchema {
            headers: vec![
                "N".to_owned(),
                tr("table-col-frequency"),
                tr("table-col-magnitude"),
                tr("table-col-phase"),
            ],
            rows: total_rows,
            row_builder: Box::new(move |row_index| {
                let idx = row_index + 1;
                vec![
                    format!("{}", row_index + 1),
                    fmt(lower + dw * row_index as f64),
                    fmt(outputs.a.get(idx).copied().unwrap_or_default()),
                    fmt(outputs.f.get(idx).copied().unwrap_or_default()),
                ]
            }),
        };

        render_schema_table(ui, &schema);
    }

    pub(super) fn show_impulse_table(ui: &mut egui::Ui, results: &Results) {
        let inputs = &results.inputs;
        let outputs = &results.outputs;

        let dt = inputs.common_parameters.get_dt();
        let l = outputs.l.max(0) as usize;
        let total_rows = l.saturating_mul(2).saturating_add(1);

        let schema = TableSchema {
            headers: vec!["N".to_owned(), "t".to_owned(), "F(t)".to_owned()],
            rows: total_rows,
            row_builder: Box::new(move |row_index| {
                let idx = row_index + 1;
                vec![
                    format!("{}", row_index + 1),
                    fmt(row_index as f64 * dt),
                    fmt(outputs.w.get(idx).copied().unwrap_or_default()),
                ]
            }),
        };

        render_schema_table(ui, &schema);
    }

    pub(super) fn show_step_table(ui: &mut egui::Ui, results: &Results) {
        let inputs = &results.inputs;
        let outputs = &results.outputs;
        let dt = inputs.common_parameters.get_dt();
        let nx = outputs.nx.max(0) as usize;

        let schema = TableSchema {
            headers: vec!["N".to_owned(), "t".to_owned(), "w".to_owned()],
            rows: nx,
            row_builder: Box::new(move |row_index| {
                let idx = row_index + 1;
                vec![
                    format!("{}", row_index + 1),
                    fmt(row_index as f64 * dt),
                    fmt(outputs.y.get(idx).copied().unwrap_or_default()),
                ]
            }),
        };

        render_schema_table(ui, &schema);
    }

    pub(super) fn show_operation_table(ui: &mut egui::Ui, results: &Results) {
        let inputs = &results.inputs;
        let outputs = &results.outputs;
        let length = inputs.operation_parameters.get_to();
        let dt = inputs.common_parameters.get_dt();
        let start = if inputs.common_parameters.filter_type == FilterType::NonRecursive {
            0.0
        } else {
            inputs.recursive_parameters.get_th()
        };
        let total_rows = if outputs.ne > 0 {
            outputs.ne as usize
        } else {
            (length / dt).max(0.0) as usize
        };
        let start_index = (start / dt).max(0.0) as usize;

        let schema = TableSchema {
            headers: vec![
                "N".to_owned(),
                "t".to_owned(),
                "x(t)".to_owned(),
                "y(t)".to_owned(),
            ],
            rows: total_rows,
            row_builder: Box::new(move |row_index| {
                let idx = start_index + row_index + 1;
                vec![
                    format!("{}", row_index + 1),
                    fmt(row_index as f64 * dt),
                    fmt(outputs.x.get(idx).copied().unwrap_or_default()),
                    fmt(outputs.y.get(idx).copied().unwrap_or_default()),
                ]
            }),
        };

        render_schema_table(ui, &schema);
    }

    pub(super) fn build_csv(
        results: &Results,
        mode: FilterMode,
        table_type: ValueTableType,
    ) -> String {
        match mode {
            FilterMode::Operation => csv_operation(results),
            FilterMode::Design => match table_type {
                ValueTableType::Frequency => csv_frequency(results),
                ValueTableType::Impulse => csv_impulse(results),
                ValueTableType::Step => csv_step(results),
            },
        }
    }
}

fn build_table(ui: &mut egui::Ui, columns: usize) -> TableBuilder<'_> {
    let height = ui.available_height();
    let width = ui.available_width()
        - ui.style().spacing.item_spacing.x * 2.0
        - ui.style().spacing.scroll.bar_width
        - ui.style().spacing.scroll.bar_inner_margin
        - ui.style().spacing.scroll.bar_outer_margin;

    let mut table = TableBuilder::new(ui)
        .auto_shrink(false)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::exact(20.0));

    let col_width = width / (columns.saturating_sub(1).max(1)) as f32;
    for _ in 0..columns.saturating_sub(2) {
        table = table.column(Column::exact(col_width));
    }

    table
        .column(Column::remainder())
        .min_scrolled_height(0.0)
        .max_scroll_height(height)
}

fn render_schema_table(ui: &mut egui::Ui, schema: &TableSchema<'_>) {
    let table = build_table(ui, schema.headers.len());
    table
        .header(20.0, |mut header| {
            for text in &schema.headers {
                header.col(|ui| {
                    ui.strong(text);
                });
            }
        })
        .body(|body| {
            body.rows(20.0, schema.rows, |mut row| {
                let values = (schema.row_builder)(row.index());
                for value in values {
                    row.col(|ui| {
                        ui.label(value);
                    });
                }
            });
        });
}

fn csv_from_schema(schema: TableSchema<'_>) -> String {
    let mut out = String::new();
    out.push_str(&schema.headers.join(","));
    out.push('\n');

    for row in 0..schema.rows {
        out.push_str(&(schema.row_builder)(row).join(","));
        out.push('\n');
    }

    out
}

fn csv_frequency(results: &Results) -> String {
    let inputs = &results.inputs;
    let outputs = &results.outputs;
    let upper = inputs.design_parameters.get_of();
    let lower = inputs.design_parameters.get_os();
    let dw = inputs.design_parameters.get_df();
    let total_rows = ((upper - lower) / dw + 1.0).max(0.0) as usize;

    let schema = TableSchema {
        headers: tr("csv-header-frequency")
            .split(',')
            .map(ToOwned::to_owned)
            .collect(),
        rows: total_rows,
        row_builder: Box::new(move |row| {
            let idx = row + 1;
            vec![
                format!("{}", row + 1),
                format!("{:.6e}", lower + dw * row as f64),
                format!("{:.6e}", outputs.a.get(idx).copied().unwrap_or_default()),
                format!("{:.6e}", outputs.f.get(idx).copied().unwrap_or_default()),
            ]
        }),
    };

    csv_from_schema(schema)
}

fn csv_impulse(results: &Results) -> String {
    let inputs = &results.inputs;
    let outputs = &results.outputs;
    let dt = inputs.common_parameters.get_dt();
    let l = outputs.l.max(0) as usize;
    let total_rows = l.saturating_mul(2).saturating_add(1);

    let schema = TableSchema {
        headers: vec!["N".to_owned(), "t".to_owned(), "F(t)".to_owned()],
        rows: total_rows,
        row_builder: Box::new(move |row| {
            let idx = row + 1;
            vec![
                format!("{}", row + 1),
                format!("{:.6e}", row as f64 * dt),
                format!("{:.6e}", outputs.w.get(idx).copied().unwrap_or_default()),
            ]
        }),
    };

    csv_from_schema(schema)
}

fn csv_step(results: &Results) -> String {
    let inputs = &results.inputs;
    let outputs = &results.outputs;
    let dt = inputs.common_parameters.get_dt();
    let nx = outputs.nx.max(0) as usize;

    let schema = TableSchema {
        headers: vec!["N".to_owned(), "t".to_owned(), "w".to_owned()],
        rows: nx,
        row_builder: Box::new(move |row| {
            let idx = row + 1;
            vec![
                format!("{}", row + 1),
                format!("{:.6e}", row as f64 * dt),
                format!("{:.6e}", outputs.y.get(idx).copied().unwrap_or_default()),
            ]
        }),
    };

    csv_from_schema(schema)
}

fn csv_operation(results: &Results) -> String {
    let inputs = &results.inputs;
    let outputs = &results.outputs;
    let length = inputs.operation_parameters.get_to();
    let dt = inputs.common_parameters.get_dt();
    let start = if inputs.common_parameters.filter_type == FilterType::NonRecursive {
        0.0
    } else {
        inputs.recursive_parameters.get_th()
    };
    let total_rows = if outputs.ne > 0 {
        outputs.ne as usize
    } else {
        (length / dt).max(0.0) as usize
    };
    let start_index = (start / dt).max(0.0) as usize;

    let schema = TableSchema {
        headers: vec![
            "N".to_owned(),
            "t".to_owned(),
            "x(t)".to_owned(),
            "y(t)".to_owned(),
        ],
        rows: total_rows,
        row_builder: Box::new(move |row| {
            let idx = start_index + row + 1;
            vec![
                format!("{}", row + 1),
                format!("{:.6e}", row as f64 * dt),
                format!("{:.6e}", outputs.x.get(idx).copied().unwrap_or_default()),
                format!("{:.6e}", outputs.y.get(idx).copied().unwrap_or_default()),
            ]
        }),
    };

    csv_from_schema(schema)
}

fn fmt(v: f64) -> String {
    format!("{v:.6e}")
}
