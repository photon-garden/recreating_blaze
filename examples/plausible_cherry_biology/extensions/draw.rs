use core::clone::Clone;
use nannou::color::conv::IntoLinSrgba;
use nannou::draw::primitive::Path;
use nannou::draw::primitive::PathStroke;
use nannou::draw::properties::ColorScalar;
use nannou::draw::Drawing;
use nannou::prelude::*;

type Points = Vec<Point2>;

pub trait DrawExtension {
    fn polylines(&self, num_polylines: usize) -> PolylineDrawings<PathStroke>;
}

impl DrawExtension for Draw {
    fn polylines(&self, num_polylines: usize) -> PolylineDrawings<PathStroke> {
        let polyline_drawings: Vec<Drawing<PathStroke>> =
            (0..num_polylines).map(|_| self.polyline()).collect();

        PolylineDrawings { polyline_drawings }
    }
}

pub struct PolylineDrawings<'a, T> {
    polyline_drawings: Vec<Drawing<'a, T>>,
}

impl<'a> PolylineDrawings<'a, Path> {
    pub fn color<C>(self, color: C) -> Self
    where
        C: IntoLinSrgba<ColorScalar> + Clone,
    {
        let polyline_drawings: Vec<Drawing<Path>> = self
            .polyline_drawings
            .into_iter()
            .map(|drawing| drawing.color(color.clone()))
            .collect();

        PolylineDrawings { polyline_drawings }
    }
}

impl<'a> PolylineDrawings<'a, PathStroke> {
    pub fn color<C>(self, color: C) -> Self
    where
        C: IntoLinSrgba<ColorScalar> + Clone,
    {
        let polyline_drawings: Vec<Drawing<PathStroke>> = self
            .polyline_drawings
            .into_iter()
            .map(|drawing| drawing.color(color.clone()))
            .collect();

        PolylineDrawings { polyline_drawings }
    }
    pub fn points(self, paths: Vec<Vec<Point2>>) -> PolylineDrawings<'a, Path> {
        let polyline_drawings: Vec<Drawing<Path>> = self
            .polyline_drawings
            .into_iter()
            .zip(paths.into_iter())
            .map(|(drawing, path)| drawing.points(path))
            .collect();

        PolylineDrawings { polyline_drawings }
    }
}
