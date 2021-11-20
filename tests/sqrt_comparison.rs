use plotters::prelude::*;


/// Plots comparison between various sqrt implementations.
#[test]
fn compare_sqrt() -> Result<(), Box<dyn std::error::Error>> 
{
    use fixed_trigonometry::*;

    use fixed::FixedI32 as F;
    use fixed::types::extra::U29 as U;

    let root = BitMapBackend::new("figures/niirf_sqrt_comparison.png", (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        //.caption("Square root comparison", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d( 0.001f32..1f32, 0.001f32..1f32)?;

    chart.configure_mesh().draw()?;

    // Draws a sinle line
    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 / 500.0).map(|x| (x, sqrt::niirf(F::<U>::from_num(x), 2).to_num::<f32>() )),
            &RED,
        ))?
        .label("NIIRF")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Draws a sinle line
    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 / 500.0).map(|x| (x, x.sqrt() )),
            &GREEN,
        ))?
        .label("f32::sqrt")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    
    // Draws a sinle line
    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 / 500.0).map(|x| (x, 1000.0*f32::abs(x.sqrt()-sqrt::niirf(F::<U>::from_num(x), 2).to_num::<f32>()) )),
            &BLUE,
        ))?
        .label("error*1000")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}