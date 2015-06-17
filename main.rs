extern crate glutin;
extern crate gleam;
extern crate layers;
extern crate geom;
extern crate x11;

use gleam::gl;
use glutin::{Api, Event, GlRequest};
use layers::rendergl;
use layers::platform::surface::NativeCompositingGraphicsContext;
use layers::scene::Scene;
use geom::rect::{Rect};
use geom::point::{Point2D};
use geom::size::{Size2D};
use geom::matrix;
use geom::Matrix4;
use layers::color::Color;
use layers::layers::Layer;
use std::rc::Rc;
use std::cell::RefCell;

struct LayerData {
	_d: isize,
}

fn d2r(angle: f32) -> f32 {
	angle * 0.017453292519943295 // (angle / 180) * Math.PI;
}

fn main() {
	let tile_size: usize = 800;
	let ws = Point2D(tile_size as u32, tile_size as u32);

    let glutin_window = glutin::WindowBuilder::new()
                        .with_title("sandpit".to_string())
                        .with_dimensions(ws.x, ws.y)
                        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 0)))
                        .with_visibility(true)
                        .build()
                        .unwrap();
    unsafe { glutin_window.make_current() };

    gl::load_with(|s| glutin_window.get_proc_address(s));

    let display = unsafe { glutin_window.platform_display() as *mut x11::xlib::Display };
    let native_context = NativeCompositingGraphicsContext::from_display(display);
    let context = rendergl::RenderContext::new(native_context, false);

    let wsf = Size2D(ws.x as f32, ws.y as f32);

    let scene = Scene::new(Rect::from_untyped(&Rect {
        origin: Point2D::zero(),
        size: wsf,
	}));

    let origin = Point2D(100.0, 100.0);
    let size = Size2D(200.0, 200.0);
    let rect = Rect(origin, size);

    let mut root_layer = Layer::new(
    						 Rect::from_untyped(&rect),
                           	 tile_size,
                           	 Color {
                           	 	r: 1.0,
                           	 	g: 0.0,
                           	 	b: 0.0,
                           	 	a: 1.0,
                           	 },
                             1.0,
                             true,
                             LayerData {
                             	_d: 0,
                             });
    root_layer.cl_transform = RefCell::new(Matrix4::create_rotation(0.0, 0.0, 1.0, d2r(45.0)));
    root_layer.transform_origin = RefCell::new(Point2D(100.0, 100.0));

    let sub_rect = Rect(Point2D(50.0, 50.0), Size2D(50.0, 50.0));
    let sub_layer = Layer::new(
    					Rect::from_untyped(&sub_rect),
    					tile_size,
    					Color {
    						r: 0.0,
    						g: 1.0,
    						b: 0.0,
    						a: 1.0
    					},
    					1.0,
    					true,
    					LayerData {
    						_d: 0,
    					});

    root_layer.add_child(Rc::new(sub_layer));

    let root_layer = Rc::new(root_layer);

    loop {
	    gl::clear_color(0.6, 0.6, 0.6, 1.0);
    	gl::clear(gl::COLOR_BUFFER_BIT);

		rendergl::render_scene(root_layer.clone(), context, &scene);
		
	    glutin_window.swap_buffers();

		let event = glutin_window.wait_events().next();

		if let Some(event) = event {
	        match event {
	            Event::KeyboardInput(_element_state, scan_code, _virtual_key_code) => {
	            	if scan_code == 9 {
	            		break;
	            	}
	            }
	            _ => {}
			}
		}
    }
}
