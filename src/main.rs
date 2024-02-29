use noise::{NoiseFn, OpenSimplex};
use array2d::Array2D;
use libm;


pub struct FlowField {
    field:Array2D<f64>,
    height:u8,
    width:u8
}

impl FlowField {
    pub fn new(seed:u8, field_width:u8, field_height:u8) -> FlowField {
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

        FlowField {
            field: flow_field,
            height: field_height,
            width: field_width
        }
    }

    pub fn get_angle(&self, x:f64, y:f64) -> f64 {
        let x = FlowField::get_flow_field_col(x);
        let y = FlowField::get_flow_field_row(y);
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





pub fn main() -> void {
    let flow_field = FlowField::new(50, 100, 100);

}
