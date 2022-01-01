use plotters::prelude::*;



/// Plots comparison error various sqrt implementations.
#[test]
fn compare_error_sqrt() -> Result<(), Box<dyn std::error::Error>> 
{
    use fixed_trigonometry::*;

    use fixed::FixedI32 as F;
    use fixed::types::extra::U29 as U;
    use cordic;

    let root = BitMapBackend::new("figures/sqrt_error_comparison.png", (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        //.caption("title", ("sans-serif", 25).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .right_y_label_area_size(60)
        .build_cartesian_2d( 0.001f32..1f32,  0f32..0.001f32)?
        .set_secondary_coord( 0.001f32..1f32, 0f32..0.001f32);

    chart
        .configure_secondary_axes()
        .y_desc("Error")
        .draw()?;

    chart.configure_mesh().draw()?;

    
    chart
        .draw_series(LineSeries::new(
            (1..=100).map(|x| x as f32 / 100.0).map(|x| (x, f32::abs(x.sqrt()-sqrt::niirf(F::<U>::from_num(x), 2).to_num::<f32>()) )),
            &BLUE,
        ))?
        .label("sqrt::niirf error")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
    chart
        .draw_secondary_series(LineSeries::new(
            (1..=100).map(|x| x as f32 / 100.0).map(|x| (x, f32::abs(x.sqrt()-cordic::sqrt(F::<U>::from_num(x)).to_num::<f32>()) )),
            &RED,
        ))?
        .label("cordic::sqrt error")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}