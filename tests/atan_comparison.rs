use plotters::prelude::*;
use fixed_trigonometry::*;
use mixed_num::*;

/// Plots comparison between various atan implementations.
#[test]
fn atan2_comparison() -> Result<(), Box<dyn std::error::Error>> 
{

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

/// Atan comparison
#[test]
fn atan_comparison() -> Result<(), Box<dyn std::error::Error>> 
{
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
        .right_y_label_area_size(60)
        .build_cartesian_2d( -10f32..10f32, -PI/2.0..PI/2.0)?
        .set_secondary_coord( -10f32..10f32, 0f32..0.005f32);

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("atan(θ)")
        .x_desc("θ")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;
    
    chart
        .configure_secondary_axes()
        .y_desc("Error")
        .draw()?;

    chart.configure_mesh().draw()?;

    let trig_atan_series = LineSeries::new(
        (-500..=500).map(|x| x as f32 *10.0 / 500.0).map(|x| (x, atan::atan( F::<U>::mixed_from_num(x) ).to_num::<f32>() )),
        &RED);

    // Draws a sinle line
    chart
        .draw_series( trig_atan_series )?
        .label("atan::atan")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    let std_atan_error = LineSeries::new(
        (-500..=500).map(|x| x as f32 *10.0 / 500.0).map(|x| (x, f32::abs(f32::atan(x) - atan::atan( F::<U>::mixed_from_num(x) ).to_num::<f32>()) )),
        &BLUE);

    // Draws a sinle line
    chart
        .draw_secondary_series( std_atan_error )?
        .label("Error")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

/// Error comparison
#[test]
fn atan_error_comparison() -> Result<(), Box<dyn std::error::Error>> 
{
    use fixed::FixedI32 as F;
    use fixed::types::extra::U22 as U;

    use cordic;

    let root = BitMapBackend::new("figures/atan_error_comparisons.png", (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        //.caption("title", ("sans-serif", 25).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(70)
        .right_y_label_area_size(70)
        .build_cartesian_2d( -10f32..10f32,  0f32..0.005f32)?
        .set_secondary_coord( -10f32..10f32, 0f32..1e-4f32);

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Error (atan::atan)")
        .x_desc("θ")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;
    
    chart
        .configure_secondary_axes()
        .y_desc("Error (cordic)")
        .draw()?;

    chart.configure_mesh().draw()?;

    let std_atan_error = LineSeries::new(
        (-500..=500).map(|x| x as f32 *10.0 / 500.0).map(|x| (x, f32::abs(f32::atan(x) - atan::atan( F::<U>::mixed_from_num(x) ).to_num::<f32>()) )),
        &RED);

    // Draws a sinle line
    chart
        .draw_series( std_atan_error )?
        .label("atan::atan error")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
    let cordic_std_atan_error = LineSeries::new(
        (-500..=500).map(|x| x as f32 *10.0 / 500.0).map(|x| (x, f32::abs(f32::atan(x) - cordic::atan( F::<U>::from_num(x) ).to_num::<f32>()) )),
        &BLUE);
    
    // Draws a sinle line
    chart
        .draw_secondary_series( cordic_std_atan_error )?
        .label("cordic::atan error")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}