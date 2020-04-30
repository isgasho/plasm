use three_d::*;
use crate::expression::Expression;
use three_d::Program;
use crate::plotter;

pub struct Plotter2d {
    plot: Plot,
    program: Program,
    expression: Expression,
    camera: Camera,
    screen_size: (usize, usize)
}

struct Camera {
    position: (f32, f32),
    size: f32
}

struct Plot {
    position_buffer: VertexBuffer,
    position_buffer_size: u32,
    axis_buffer: VertexBuffer,
}

impl Plotter2d {
    pub fn new(gl: &Gl, expression: Expression, screen_size: (usize, usize)) -> Plotter2d {

        let program = plotter::load_program(gl);
        let camera = Camera {position: (0.0, 0.0), size: 10.0};
        let plot = Plot::new(gl, &expression, screen_size.0 as u32, &camera);

        Plotter2d {
            plot,
            program,
            expression,
            camera,
            screen_size
        }
    }
}

impl plotter::Plotter for Plotter2d {

    fn set_expression(&mut self, expression: Expression) {
        self.expression = expression
    }

    fn zoom(&mut self, delta: f32) {
        self.camera.size *= (1.1 as f32).powf(delta);
    }

    fn translate(&mut self, delta_x: f32, delta_y: f32) {
        self.camera.position.0 += delta_x * self.camera.size / self.screen_size.0 as f32;
        self.camera.position.1 += delta_y * self.camera.size / self.screen_size.1 as f32;
    }

    fn draw(&self, gl: &Gl) {

        Screen::write(gl, 0, 0, self.screen_size.0, self.screen_size.1, Some(&vec4(0.9, 0.9, 0.9, 1.0)), Some(1.0), &|| {

            self.plot.draw(&self.program);

        }).unwrap();

    }

    fn update_view(&mut self, gl: &Gl) {
        self.plot = Plot::new(gl, &self.expression, self.screen_size.0 as u32, &self.camera);
    }
}


impl Camera { 
    // project a point to screen coordinate [-1,1]
    fn to_screen_coordinate(&self, point: (f32, f32)) -> (f32, f32) {
        let x_proj = 2.0*(point.0 - self.position.0)/self.size;
        let y_proj = 2.0*(point.1 - self.position.1)/self.size;
        (x_proj, y_proj)
    }
}

impl Plot {

    fn new(gl: &Gl, expression: &Expression, count: u32, camera: &Camera) -> Plot {
        let points = Plot::generate_points(expression, count, camera);
        let axis_points = Plot::generate_axis_lines(camera);

        let position_buffer = VertexBuffer::new_with_static_f32(&gl, &points).unwrap();
        let position_buffer_size = count;
        let axis_buffer = VertexBuffer::new_with_static_f32(&gl, &axis_points).unwrap();

        Plot {
            position_buffer,
            position_buffer_size,
            axis_buffer
        }
    }

    fn draw(&self, program: &Program) {
        program.add_uniform_mat4("worldViewProjectionMatrix", &Mat4::identity()).unwrap();

        program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        program.add_uniform_vec4("color", &vec4(0.3, 0.3, 0.3, 1.0)).unwrap();
        program.draw_arrays_mode(self.position_buffer_size, consts::LINE_STRIP);

        // draw axis
        program.use_attribute_vec3_float(&self.axis_buffer, "position").unwrap();
        program.add_uniform_vec4("color", &vec4(0.5, 0.5, 0.5, 1.0)).unwrap();
        program.draw_arrays_mode(4, consts::LINES);
    }

    fn generate_points(expression: &Expression, count: u32, camera: &Camera) -> Vec<f32> {
        
        let mut points: Vec<f32> = Vec::with_capacity((count*3) as usize);

        let x_start = camera.position.0 - camera.size/2.0;

        for i in 0..count {

            // divide camera width into count segments and evaluate y
            let i = i as f32;
            let count = count as f32;
            let x = x_start + i * camera.size/count;
            let y = expression.eval((x, 0.0)); // y=0 as there is no y value
            let (x_screen, y_screen) = camera.to_screen_coordinate((x, y));

            points.push(x_screen);
            points.push(y_screen);
            points.push(0.0);
        }

        points
    }

    fn generate_axis_lines(camera: &Camera) -> Vec<f32> {
        let (x_zero, y_zero) = camera.to_screen_coordinate((0.0, 0.0));

        vec![-1.0, y_zero, 0.0,
            1.0, y_zero, 0.0,
            x_zero, -1.0, 0.0,
            x_zero, 1.0, 0.0]
    }

}