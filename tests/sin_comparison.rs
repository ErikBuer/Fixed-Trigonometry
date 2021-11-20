use plotters::prelude::*;


/// Plots comparison between various sqrt implementations.
#[test]
fn compare_polynomial_sine() -> Result<(), Box<dyn std::error::Error>> 
{
    use fixed_trigonometry::*;

    use fixed::FixedI32 as F;
    use fixed::types::extra::U20 as U;
    use std::f32::consts::PI as PI;

    let root = BitMapBackend::new("figures/polynomial_sine_comparison.png", (1000, 500)).into_drawing_area();
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
        .y_desc("sin(θ)")
        .x_desc("θ")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.configure_mesh().draw()?;

    let poly_sin_series = LineSeries::new(
        (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, sin(F::<U>::from_num(x) ).to_num::<f32>() )),
        &RED);

    // Draws a sinle line
    chart
        .draw_series( poly_sin_series )?
        .label("Polynomial Sin")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    let std_sin_series = LineSeries::new(
        (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, x.sin() )),
        &BLUE);

    // Draws a sinle line
    chart
        .draw_series( std_sin_series )?
        .label("f32::sin")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
    // Draws a sinle line
    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, 100000.0*f32::abs(x.sin()-sin( F::<U>::from_num(x) ).to_num::<f32>()) )),
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
    use fixed::types::extra::U20 as U;
    use std::f32::consts::PI as PI;

    let root = BitMapBackend::new("figures/cordic_poly_sine_error_comparison.png", (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        //.caption("title", ("sans-serif", 25).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(100)
        .build_cartesian_2d( -PI..PI, 0f32..0.00001f32 )?;

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
            (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, f32::abs(x.sin()-sin( F::<U>::from_num(x) ).to_num::<f32>()) )),
            &BLUE,
        ))?
        .label("fixed_trigonometry::sin error")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
    // Draws a sinle line
    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, f32::abs(x.sin()-cordic::sin( F::<U>::from_num(x) ).to_num::<f32>()) )),
            &RED,
        ))?
        .label("cordic::sin error")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}