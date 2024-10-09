use std::char;

use approx::abs_diff_ne;
use full_palette::ORANGE;
use image::{ImageBuffer, Rgb};
use plotters::{chart, prelude::*};
use plotters_arrows::ThinArrow;
type Chart<'a> = ChartContext<'a, BitMapBackend<'a>, Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>>;

use crate::types::{Dem1D, DispData, DispProfile, Surface1D};

pub fn plot_section(size: (u32, u32), dem: (&Dem1D, ShapeStyle), surfaces: Vec<(&Surface1D, ShapeStyle)>, disp_profiles: Vec<&DispProfile>, disp_datas: Vec<&DispData>) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut graph_buffer = ImageBuffer::new(size.0, size.1);
    {
        let (dem, dem_style) = (dem.0, dem.1);
        // init graph
        let graph_root = BitMapBackend::with_buffer(&mut graph_buffer, size);
        let drawing_area = graph_root.into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();

        // boundaries
        let x_margin = (dem.x.last().unwrap() - dem.x.first().unwrap()) * 0.05;
        let x_spec = ((dem.x.first().unwrap() - x_margin) as f64)..((dem.x.last().unwrap() + x_margin) as f64);
        let mut y_min = dem.surface.z[0];
        let mut y_max = dem.surface.z[0];
        let mut all_y = vec![&dem.surface.z];
        surfaces.iter().for_each(|(s, style)| all_y.push(&s.z));
        for s in all_y {
            s.iter().for_each(|k| 
                match k.to_owned() {
                    y if y > y_max => y_max = y,
                    y if y < y_min => y_min = y,
                    _ => (), 
                }
            );
        };
        let y_margin = (y_max - y_min) * 0.05;
        let y_spec = ((y_min - y_margin) as f64)..((y_max + y_margin) as f64);
        
        // Decomment for pseudo orthonormal grid
        // let x_spec = (0.)..(600.);
        // let y_spec = (-150.)..(150.);

        // init mesh
        let mut chart = ChartBuilder::on(&drawing_area)
            .margin(8)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_spec, y_spec).unwrap();

        chart
            .configure_mesh()
            .disable_mesh()
            .draw().unwrap();


        // draw
        let mut chart_extended = ChartExtended::new(chart, dem.x.to_owned(), dem.surface.z.to_owned());
        surfaces.iter().for_each(|(s, style)| s.plot(&mut chart_extended, *style));
        dem.surface.plot(&mut chart_extended, dem_style);
        disp_datas.iter().for_each(|d| d.plot_arrows(&mut chart_extended));
        disp_profiles.iter().for_each(|p| p.plot_arrows(&mut chart_extended));
        disp_profiles.iter().for_each(|p| p.plot_arrows_regul(&mut chart_extended));
        let mut surface = Vec::new();
        surfaces.iter().for_each(|(s, _style)| surface.push(s.z.clone()));
        disp_profiles.iter().enumerate().for_each(|(i, p)| p.plot_pillars(&mut chart_extended, surface[i].clone()));
        
        // do all pending tasks
        drawing_area.present().expect("draw please");
    }
    graph_buffer
}

struct ChartExtended<'a> {
    pub chart: Chart<'a>,
    pub x_support: Vec<f32>,
    pub z_support : Vec<f32>,
    // pub full_mesh: Option<bool>, // None: no mesh, true full mesh, false light mesh // TODO
}

pub fn get_style(color: RGBColor, opacity: f64, is_filled: bool, width: u32) -> ShapeStyle {
    ShapeStyle {
        color: color.mix(opacity),
        filled: is_filled,
        stroke_width: width,
    }
}

impl<'a> ChartExtended<'a> {
    pub fn new(chart: Chart<'a>, x_support: Vec<f32>, z_support: Vec<f32>) -> Self {
        ChartExtended { chart, x_support, z_support }
    }
}

impl Surface1D {
    fn plot(&self, chart_extended: &mut ChartExtended<'_>, shape_style: ShapeStyle) {
        let nb = chart_extended.x_support.len();
        let line = LineSeries::new((0..nb).map(|k| (chart_extended.x_support[k] as f64, self.z[k] as f64)), shape_style);
        chart_extended.chart.draw_series(line).unwrap();
    }
}

impl DispProfile {
    fn plot_arrows(&self, chart_extended: &mut ChartExtended<'_>) {
        let nb = chart_extended.x_support.len();
        let (vec_x, vec_z) = self.get_xz_vec();
        let arrows = (0..nb)
            .filter(|k| abs_diff_ne!(self.amplitude_vec[*k], 0.))
            .map(|k| {
                let origin = (self.origin_x[k] as f64, self.origin_z[k] as f64);
                let target = ((self.origin_x[k] + vec_x[k]) as f64, (self.origin_z[k] + vec_z[k]) as f64);
                ThinArrow::new(origin, target, &ORANGE)
            }
        );
        chart_extended.chart.draw_series(arrows).unwrap();
    }

    fn plot_arrows_regul(&self, chart_extended: &mut ChartExtended<'_>) {
        let nb = chart_extended.x_support.len();
        let dem_x = chart_extended.x_support.to_owned();
        let dem_z = chart_extended.z_support.to_owned();
        let (vec_x, vec_z) = self.get_xz_vec_regul();
        let arrows = (0..nb)
            .filter(|k| abs_diff_ne!(self.amplitude_regul[*k], 0.))
            .map(|k| {
                let origin = (dem_x[k] as f64, dem_z[k] as f64);
                let target = ((dem_x[k] + vec_x[k]) as f64, (dem_z[k] + vec_z[k]) as f64);
                ThinArrow::new(origin, target, &RED)
            }
        );
        chart_extended.chart.draw_series(arrows).unwrap();
    }

    fn plot_pillars(&self, chart_extended: &mut ChartExtended<'_>, z_slide: Vec<f32>) {
        let nb = chart_extended.x_support.len();
        (0..nb).for_each(|k|{
            let line = LineSeries::new(
                vec![(chart_extended.x_support[k] as f64, z_slide[k] as f64), (self.origin_x[k] as f64, self.origin_z[k] as f64)], 
                &BLACK);
            chart_extended.chart.draw_series(line).unwrap();
        });
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