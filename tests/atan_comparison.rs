use plotters::prelude::*;


/// Plots comparison between various atan implementations.
#[test]
fn atan2_comparison() -> Result<(), Box<dyn std::error::Error>> 
{
    use fixed_trigonometry::*;

    use fixed::FixedI32 as F;
    use fixed::types::extra::U28 as U;
    use std::f32::consts::PI as PI;

    let root = BitMapBackend::new("figures/atan2_comparisons.png", (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        //.caption("title", ("sans-serif", 25).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d( -PI..PI, -PI..PI)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("atan2( Im{e^(iθ)) , Re{e^(iθ))} )")
        .x_desc("θ")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.configure_mesh().draw()?;

    let atan2_series = LineSeries::new(
        (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, atan::atan2(F::<U>::from_num(x.sin()), F::<U>::from_num(x.cos()) ).to_num::<f32>() )),
        &RED);

    // Draws a sinle line
    chart
        .draw_series( atan2_series )?
        .label("atan::atan2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    let std_atan_series = LineSeries::new(
    (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, f32::atan2(x.sin(),x.cos()) )),
    &BLUE);

    // Draws a sinle line
    chart
        .draw_series( std_atan_series )?
        .label("f32::atan2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

#[test]
fn atan_comparison() -> Result<(), Box<dyn std::error::Error>> 
{
    use fixed_trigonometry::*;

    use fixed::FixedI32 as F;
    use fixed::types::extra::U22 as U;
    use std::f32::consts::PI as PI;

    let root = BitMapBackend::new("figures/atan_comparisons.png", (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        //.caption("title", ("sans-serif", 25).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d( -10f32..10f32, -PI/2.0..PI/2.0)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("atan2(θ)")
        .x_desc("θ")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.configure_mesh().draw()?;

    let atan2_series = LineSeries::new(
        (-500..=500).map(|x| x as f32 *10.0 / 500.0).map(|x| (x, atan::atan( F::<U>::from_num(x) ).to_num::<f32>() )),
        &RED);

    // Draws a sinle line
    chart
        .draw_series( atan2_series )?
        .label("atan::atan")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    let std_atan_series = LineSeries::new(
        (-500..=500).map(|x| x as f32 *10.0 / 500.0).map(|x| (x, f32::atan(x) )),
        &BLUE);

    // Draws a sinle line
    chart
        .draw_series( std_atan_series )?
        .label("f32::atan")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}