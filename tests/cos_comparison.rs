use plotters::prelude::*;

/// Plots comparison between various sqrt implementations.
#[test]
fn compare_polynomial_cosine() -> Result<(), Box<dyn std::error::Error>> 
{
    use fixed_trigonometry::*;

    use fixed::FixedI32 as F;
    use fixed::types::extra::U20 as U;
    use std::f32::consts::PI as PI;

    let root = BitMapBackend::new("figures/polynomial_cosine_comparison.png", (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        //.caption("title", ("sans-serif", 25).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d( -PI..PI, -1.1f32..1.1f32 )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("cos(θ)")
        .x_desc("θ")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.configure_mesh().draw()?;

    let poly_sin_series = LineSeries::new(
        (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, cos(F::<U>::from_num(x) ).to_num::<f32>() )),
        &RED);

    // Draws a sinle line
    chart
        .draw_series( poly_sin_series )?
        .label("Polynomial cos")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    let std_sin_series = LineSeries::new(
        (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, x.cos() )),
        &BLUE);

    // Draws a sinle line
    chart
        .draw_series( std_sin_series )?
        .label("f32::cos")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
    // Draws a sinle line
    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, 100000.0*f32::abs(x.cos()-cos( F::<U>::from_num(x) ).to_num::<f32>()) )),
            &GREEN,
        ))?
        .label("error*100 000")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}


/// Plots comparison between Errors of the polynomial and cordic sin implementation.
#[test]
fn poly_vs_cordic_error() -> Result<(), Box<dyn std::error::Error>> 
{
    use cordic;
    use fixed_trigonometry::*;

    use fixed::FixedI32 as F;
    use fixed::types::extra::U22 as U;
    use std::f32::consts::PI as PI;

    let root = BitMapBackend::new("figures/cordic_poly_cos_error_comparison.png", (1000, 300)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        //.caption("title", ("sans-serif", 25).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d( -PI..PI, 0f32..5e-6f32 )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Error(θ)")
        .x_desc("θ")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.configure_mesh().draw()?;

    // Draws a sinle line
    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, f32::abs(x.cos()-cos( F::<U>::from_num(x) ).to_num::<f32>()) )),
            &BLUE,
        ))?
        .label("fixed_trigonometry::cos error")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
    // Draws a sinle line
    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, f32::abs(x.cos()-cordic::cos( F::<U>::from_num(x) ).to_num::<f32>()) )),
            &RED,
        ))?
        .label("cordic::cos error")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}