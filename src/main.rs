use noise::{NoiseFn, OpenSimplex};
use array2d::Array2D;
use json;
use json::{JsonValue, JsonError};


pub struct NoiseField {
    field:Array2D<f64>,
    height:u8,
    width:u8
}

impl NoiseField {
    pub fn new(seed:u8, field_width:u8, field_height:u8) -> NoiseField {
        let noise_gen = OpenSimplex::new(seed.into());
        let mut flow_field = Array2D::filled_with(
            0 as f64,
            field_height.into(),
            field_width.into()
        );

        let width_as_double = f64::from(field_width);
        for y in 0..field_height {
            for x in 0..field_width {
                let x_as_double = f64::from(x);
                let y_as_double = f64::from(y);
                let xp = x_as_double / width_as_double;
                let yp = y_as_double / width_as_double;
                flow_field[(x as usize, y as usize)] = noise_gen.get([xp, yp]);
            }
        }

        NoiseField {
            field: flow_field,
            height: field_height,
            width: field_width
        }
    }

    pub fn get_angle(&self, x:f64, y:f64) -> f64 {
        let x = NoiseField::get_flow_field_col(x);
        let y = NoiseField::get_flow_field_row(y);
        self.field[(x as usize, y as usize)]
    }

    pub fn get_flow_field_col(x:f64) -> u8 {
        x as u8
    }

    pub fn get_flow_field_row(y:f64) -> u8 {
        y as u8
    }

    pub fn off_boundaries(&self, x:f64, y:f64) -> bool {
        x <= 0.0 ||
        y <= 0.0 ||
        x >= self.width.into() ||
        y >= self.height.into()
    }
}



pub fn threshold(noise_field:&NoiseField, width:u32, height:u32) -> Array2D<u8> {
    let mut grid = Array2D::filled_with(0 as u8, width as usize, height as usize);
    for x in 0..width as usize {
        for y in 0..height as usize {
            grid[(x, y)] = (noise_field.field[(x, y)] > 0.0) as u8;
        }
    }
    return grid
}



pub fn main() {
    let width = 100;
    let height = 100;
    let noise_field = NoiseField::new(50, width.into(), height.into());
    let binary_grid = threshold(&noise_field, width.into(), height.into());
    let dx = 1;
    let dy = 1;

    for x in 0..width as usize {
        for y in 0..height as usize {
            a
        }
    }

}
