use plotters::prelude::*;

pub struct PlotColors {
    background: RGBColor,
    textcolor: RGBColor,
    highlight: RGBColor,
    darklight: RGBColor,
    labelcolor: ShapeStyle,
}

impl PlotColors {
    pub fn new() -> PlotColors {
        PlotColors {
            background: RGBColor(12, 22, 24),
            textcolor: RGBColor(209, 172, 0),
            highlight: RGBColor(250, 244, 211),
            darklight: RGBColor(60, 73, 76),
            labelcolor: ShapeStyle {
                color: RGBAColor(193, 41, 46, 1.0),
                filled: true,
                stroke_width: 2,
            },
        }
    }

    pub fn background(&self) -> &RGBColor {
        &self.background
    }

    pub fn textcolor(&self) -> &RGBColor {
        &self.textcolor
    }

    pub fn highlight(&self) -> &RGBColor {
        &self.highlight
    }

    pub fn darklight(&self) -> &RGBColor {
        &self.darklight
    }

    pub fn labelstyle(&self) -> &ShapeStyle {
        &self.labelcolor
    }
}
