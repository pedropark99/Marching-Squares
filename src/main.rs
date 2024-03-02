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

pub fn off_boundaries(x:usize, y:usize, limit:usize) -> bool {
    x <= 0 ||
    y <= 0 ||
    x >= limit ||
    y >= limit
}


pub fn get_square(binary_grid:&Array2D<u8>, x:usize, y:usize) -> Vec<u8> {
    let values = vec![
        binary_grid[(x, y)],
        binary_grid[(x, y + 1)],
        binary_grid[(x + 1, y + 1)],
        binary_grid[(x + 1, y)],
    ];
    values
}

pub fn calc_case_index(square: Vec<u8>) -> u8 {
    let case_index = square[0] * 8
        + square[1] * 4
        + square[2] * 2
        + square[3] * 1;

    case_index
}


struct Polygon {
    case_index: u8,
    x: Vec<f64>,
    y: Vec<f64>,
}

// Each pixel is filled with a polygon
struct Pixel {
    grid_x: u32,
    grid_y: u32,
    fill: Polygon
}



pub fn main() {
    let width = 100;
    let height = 100;
    let noise_field = NoiseField::new(50, width.into(), height.into());
    let binary_grid = threshold(&noise_field, width.into(), height.into());
    let dx = 1;
    let dy = 1;

    let a: [f64; 2] = [0.0, 1.0];
    let b: [f64; 2] = [1.0, 1.0];
    let c: [f64; 2] = [1.0, 0.0];
    let d: [f64; 2] = [0.0, 0.0];
    let ab: [f64; 2] = [0.5, 1.0];
    let bc: [f64; 2] = [1.0, 0.5];
    let cd: [f64; 2] = [0.5, 0.0];
    let da: [f64; 2] = [0.0, 0.5];
    let cases = vec![
        (0, vec![0.0], vec![0.0]),
        (1, vec![d[0], cd[0], da[0]], vec![d[1], cd[1], da[1]]),
        (2, vec![c[0], cd[0], bc[0]], vec![c[1], cd[1], bc[1]]),
        (3, vec![d[0], c[0], bc[0], da[0]], vec![d[1], c[1], bc[1], da[1]]),
        (4, vec![b[0], ab[0], bc[0]], vec![b[1], ab[1], bc[1]]),
        (5, vec![d[0], da[0], ab[0], b[0], bc[0], cd[0]], vec![d[1], da[1], ab[1], b[1], bc[1], cd[1]]),
        (6, vec![ab[0], b[0], c[0], cd[0]], vec![ab[1], b[1], c[1], cd[1]]),
        (7, vec![da[0], d[0], c[0], b[0], ab[0]], vec![da[1], d[1], c[1], b[1], ab[1]]),
        (8, vec![a[0], ab[0], da[0]], vec![a[1], ab[1], da[1]]),
        (9, vec![a[0], ab[0], cd[0], d[0]], vec![a[1], ab[1], cd[1], d[1]]),
        (10, vec![a[0], ab[0], bc[0], c[0], cd[0], da[0]], vec![a[1], ab[1], bc[1], c[1], cd[1], da[1]]),
        (11, vec![a[0], ab[0], bc[0], c[0], d[0]], vec![a[1], ab[1], bc[1], c[1], d[1]]),
        (12, vec![a[0], b[0], bc[0], da[0]], vec![a[1], b[1], bc[1], da[1]]),
        (13, vec![a[0], b[0], bc[0], cd[0], d[0]], vec![a[1], b[1], bc[1], cd[1], d[1]]),
        (14, vec![a[0], b[0], c[0], cd[0], da[0]], vec![a[1], b[1], c[1], cd[1], da[1]]),
        (15, vec![a[0], b[0], c[0], d[0]], vec![a[1], b[1], c[1], d[1]]),
    ];




    for x in 0..(width - 1) as usize {
        for y in 0..(height - 1) as usize {
            let square = get_square(&binary_grid, x, y);
            let case_index = calc_case_index(square);
            println!("{}\n", case_index);
        }
    }

}
