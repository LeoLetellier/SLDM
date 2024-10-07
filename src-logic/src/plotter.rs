use std::char;

use plotters::prelude::*;
use plotters_arrows::ThinArrow;
type Chart<'a> = ChartContext<'a, BitMapBackend<'a>, Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>>;

use crate::types::{Dem1D, DispData, DispProfile, Surface1D};

// section graph / profile graph / correlation graph
// option path -> None = use buffer
pub fn graph_init_from_memory(size: (u32, u32)) {
    let mut graph_buffer = vec![0; size.0 as usize * size.1 as usize * 3];
    // height width rgb
    let root = BitMapBackend::with_buffer(&mut graph_buffer, size);
}

pub fn graph_init(path: &str, size: (u32, u32)) {

}

struct ChartExtended<'a> {
    pub chart: Chart<'a>,
    pub x_support: Vec<f32>,
}

impl Dem1D {
    fn plot(&self, chart_extended: &mut ChartExtended<'_>) {
        let nb = chart_extended.x_support.len();
        let line = LineSeries::new((0..nb).map(|k| (chart_extended.x_support[k] as f64, self.z[k] as f64)), &BLACK);
        chart_extended.chart.draw_series(line).unwrap();
    }
}

impl Surface1D {
    fn plot(&self, chart_extended: &mut ChartExtended<'_>) {
        let nb = chart_extended.x_support.len();
        let line = LineSeries::new((0..nb).map(|k| (chart_extended.x_support[k] as f64, self.z[k] as f64)), &BLUE);
        chart_extended.chart.draw_series(line).unwrap();
    }
}

impl DispProfile {
    fn plot_arrows(&self, chart_extended: &mut ChartExtended<'_>) {
        let nb = chart_extended.x_support.len();
        let (vec_x, vec_z) = self.get_xz_vec();
        let arrows = (0..nb).map(|k| {
            let origin = (self.origin_x[k] as f64, self.origin_z[k] as f64);
            let target = ((self.origin_x[k] + vec_x[k]) as f64, (self.origin_z[k] + vec_z[k]) as f64);
            ThinArrow::new(origin, target, &BLACK)
        });
        chart_extended.chart.draw_series(arrows).unwrap();
    }

    fn plot_pillars(&self, chart_extended: &mut ChartExtended<'_>, z_slide: Vec<f32>) {
        let nb = chart_extended.x_support.len();
        // let x
    }
}

impl DispData {
    fn plot_arrows(&self, chart_extended: &mut ChartExtended<'_>) {
        // need projections beforehand
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plotters() {
        // let x: Vec<f32> = vec![0., 100., 200., 300., 400., 500.];
        // let z: Vec<f32> = vec![0., 10., 22., 35., 40., 43.];
        
        // const OUT_FILE_NAME: &str = "figures_out/test_plotters.png";
        // let root = BitMapBackend::new(OUT_FILE_NAME, (1000, 600)).into_drawing_area();
        // root.fill(&WHITE).unwrap();

        // let mut chart = ChartBuilder::on(&root).margin(10).caption("DEM Profile");
        // chart.build_cartesian_2d(-10..510, -5..48).unwrap();


    }
}