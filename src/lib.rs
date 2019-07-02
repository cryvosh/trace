extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;
extern crate specs;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use specs::{Read, Component, VecStorage, World, Builder, System, ReadStorage, RunNow};
use web_sys::{CanvasRenderingContext2d};

// console.log macro
macro_rules! console_log {
	($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// JS imports
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(a: &str);
}

#[derive(Debug)]
struct Light {
	// x: f64,
	// y: f64,
	ray_count: u32
}
impl Component for Light {
	type Storage = VecStorage<Self>;
}

struct RenderSystem{
	ctx: CanvasRenderingContext2d
}
impl<'a> System<'a> for RenderSystem {
	type SystemData = (Read<'a, Window>, Read<'a, Mouse>, ReadStorage<'a, Light>);

	fn run(&mut self, data: Self::SystemData) {
		use specs::Join;
		let (win, mouse, light) = data;

		self.ctx.clear_rect(0.0,0.0,win.width,win.height);

		for light in light.join() {

			for i in 0..light.ray_count {
				let ray_ang_0 = f64::consts::PI * 2.0 * (i as f64) / (light.ray_count as f64);

				let scale = 3.0;
				let epsilon = 3.0;

				let mut x_0 = mouse.x;
				let mut y_0 = mouse.y;

				let mut x_1 = 0.0;
				let mut y_1 = 0.0;

				let mut x_comp_0 = ray_ang_0.cos();
				let mut y_comp_0 = ray_ang_0.sin();

				let mut x_comp_1 = 0.0;
				let mut y_comp_1 = 0.0;

				let mut normal_x = 0.0;
				let mut normal_y = 0.0;

				self.ctx.begin_path();

				for i in 0..700 {
					normal_x = ior(x_0+epsilon, y_0) - ior(x_0-epsilon, y_0);
					normal_y = ior(x_0, y_0+epsilon) - ior(x_0, y_0-epsilon);
					let normal_len = (normal_x.powi(2) + normal_y.powi(2)).sqrt();

					if (normal_len == 0.0) {
						normal_x = -x_comp_0;
						normal_y = -y_comp_0;
					} else {
						normal_x /= normal_len;
						normal_y /= normal_len;
					}

					let mut cos_theta_0 = (-x_comp_0*normal_x - y_comp_0*normal_y);

					if (cos_theta_0 < 0.0) {
						normal_x *= -1.0;
						normal_y *= -1.0;
						cos_theta_0 = (-x_comp_0*normal_x - y_comp_0*normal_y);
					}

					x_1 = x_0 + x_comp_0*scale;
					y_1 = y_0 + y_comp_0*scale;
					let n_0 = ior(x_0, y_0);
					let n_1 = ior(x_1, y_1);
					let r = n_0 / n_1;

					let radicand = 1.0 - r*r * (1.0 - cos_theta_0*cos_theta_0);

					if (radicand < 0.0) {
						x_comp_1 = x_comp_0 + 2.0 * cos_theta_0 * normal_x;
						y_comp_1 = y_comp_0 + 2.0 * cos_theta_0 * normal_y;

						x_1 = x_0+x_comp_1*scale;
						y_1 = y_0+y_comp_1*scale;
					} else {
						let cos_theta_1 = radicand.sqrt();

						x_comp_1 = r * x_comp_0 + (r * cos_theta_0 - cos_theta_1) * normal_x;
						y_comp_1 = r * y_comp_0 + (r * cos_theta_0 - cos_theta_1) * normal_y;

						x_1 = x_0+x_comp_1*scale;
						y_1 = y_0+y_comp_1*scale;
					}

					self.ctx.move_to(x_0, y_0);
					self.ctx.line_to(x_1, y_1);

					x_0 = x_1;
					y_0 = y_1;
					x_comp_0 = x_comp_1;
					y_comp_0 = y_comp_1;
				}

				self.ctx.stroke();
			}
		}
	}
}

pub fn ior(x: f64, y: f64) -> f64 {
	return 100.0/((x-600.0).abs().powi(2) + (y-400.0).abs().powi(2)).sqrt();
	//return (x/50.0).sin()*20.0 + y/10.0;
}

#[derive(Default, Debug)]
struct Mouse {
	x: f64,
	y: f64,
	left: u32
}
#[derive(Default, Debug)]
struct Window {
	width: f64,
	height: f64
}

#[wasm_bindgen]
pub struct App {
	tracer: RenderSystem,
	world: World
}

#[wasm_bindgen]
impl App {
	#[wasm_bindgen(constructor)]
	pub fn new(ctx: CanvasRenderingContext2d, width: f64, height: f64) -> Result<App, JsValue>{
		let mut world = World::new();
		let mut tracerSystem = RenderSystem{ctx};

		world.register::<Light>();
		world.add_resource(Mouse{x: 0.0, y: 0.0, left: 0});
		world.add_resource(Window{width: width, height: height});

		let light = world.create_entity().with(Light {ray_count: 100}).build();	

		return Ok(App {
			tracer: tracerSystem,
			world: world,
		});
	}

	pub fn mouse_move(&mut self, x: f64, y: f64) {
		let mut mouse = self.world.write_resource::<Mouse>();
		*mouse = Mouse{x: x, y: y, left: mouse.left};
	}

	pub fn mouse_left(&mut self, state: u32) {
		let mut mouse = self.world.write_resource::<Mouse>();
		*mouse = Mouse{x: mouse.x, y: mouse.y, left: state};
	}

	pub fn resize(&mut self, width: f64, height:f64) {
		let mut window = self.world.write_resource::<Window>();
		*window = Window{width: width, height: height};
	}

	pub fn update(&mut self, width: f64, height: f64) {
		self.tracer.run_now(&self.world.res);
		self.world.maintain();
	}
}
























