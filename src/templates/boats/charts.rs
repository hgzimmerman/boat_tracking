use chrono::NaiveDate;
use plotters::prelude::*;

/// Generate SVG for monthly usage chart (30 days)
pub fn monthly_usage_chart(data: &[(NaiveDate, i64)]) -> Result<String, Box<dyn std::error::Error>> {
    if data.is_empty() {
        return Ok(empty_chart_svg("No usage data available"));
    }

    let mut buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut buffer, (800, 400)).into_drawing_area();
        root.fill(&WHITE)?;

        let max_count = data.iter().map(|(_, c)| *c).max().unwrap_or(1);
        let data_len = data.len();

        let mut chart = ChartBuilder::on(&root)
            .caption("Daily Uses (Last 30 Days)", ("sans-serif", 24))
            .margin(15)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0..data_len, 0..max_count + 1)?;

        chart
            .configure_mesh()
            .x_label_formatter(&|idx| {
                if *idx < data.len() {
                    data[*idx].0.format("%m-%d").to_string()
                } else {
                    String::new()
                }
            })
            .x_labels(10)
            .y_desc("Uses")
            .draw()?;

        // Draw bars
        chart.draw_series(
            data.iter().enumerate().map(|(idx, (_date, count))| {
                Rectangle::new([(idx, 0), (idx, *count)], BLUE.mix(0.6).filled())
            }),
        )?;
    }
    Ok(buffer)
}

/// Generate SVG for yearly usage chart (12 months)
pub fn yearly_usage_chart(data: &[(NaiveDate, i64)]) -> Result<String, Box<dyn std::error::Error>> {
    if data.is_empty() {
        return Ok(empty_chart_svg("No usage data available"));
    }

    let mut buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut buffer, (800, 400)).into_drawing_area();
        root.fill(&WHITE)?;

        let max_count = data.iter().map(|(_, c)| *c).max().unwrap_or(1);
        let data_len = data.len();

        let mut chart = ChartBuilder::on(&root)
            .caption("Monthly Uses (Last 12 Months)", ("sans-serif", 24))
            .margin(15)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0..data_len, 0..max_count + 1)?;

        chart
            .configure_mesh()
            .x_label_formatter(&|idx| {
                if *idx < data.len() {
                    data[*idx].0.format("%y-%m").to_string()
                } else {
                    String::new()
                }
            })
            .x_labels(12)
            .y_desc("Uses")
            .draw()?;

        // Draw bars
        chart.draw_series(
            data.iter().enumerate().map(|(idx, (_date, count))| {
                Rectangle::new([(idx, 0), (idx, *count)], GREEN.mix(0.6).filled())
            }),
        )?;
    }
    Ok(buffer)
}

/// Generate empty chart SVG with message
fn empty_chart_svg(message: &str) -> String {
    let mut buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut buffer, (800, 400)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        root.draw_text(
            message,
            &TextStyle::from(("sans-serif", 20).into_font()).color(&BLACK),
            (400, 200),
        )
        .unwrap();
    }
    buffer
}
