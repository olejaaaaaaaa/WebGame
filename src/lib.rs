#![allow(warnings)]

use std::convert::TryInto;
use std::cell::RefCell;

use js_sys::JsString;
use shaders::*;
use wasm_bindgen::convert::OptionIntoWasmAbi;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram, console};
use web_sys::*;
extern crate js_sys;

mod shaders;

struct Context {
    gl: WebGlRenderingContext,
}

impl Context {
    fn new(canvas: &str) -> Self {

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let gl: WebGlRenderingContext = canvas
            .get_context("webgl").unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();
    
        gl.viewport(
            0,
            0,
            canvas.width().try_into().unwrap(),
            canvas.height().try_into().unwrap(),
        );

        Self { gl }
    }

    fn clear(&self, r: f32, g: f32, b: f32) {
        self.gl.clear_color(r, g, b, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }
}

enum Event<'s> {
    Key(&'s str),
    Update(),
    Resize(usize, usize)
}

thread_local! {
    pub static EVENT_HANDLER: RefCell<Box<dyn FnMut(Event)>> = RefCell::new(Box::new(|_|{}));
}

pub fn event(function: impl FnMut(Event) + 'static) {
    EVENT_HANDLER.with(|event_handler| {
        *event_handler.borrow_mut() = Box::new(function);
    });
}

#[wasm_bindgen]
pub fn keyboard_event(ev: &str) {
    EVENT_HANDLER.with(|_ev|{_ev.borrow_mut()(Event::Key(ev))});
}

#[wasm_bindgen]
pub fn update_event() {
    EVENT_HANDLER.with(|_ev|{_ev.borrow_mut()(Event::Update())});
}

#[wasm_bindgen]
pub fn resize_event(width: usize, height: usize) {
    EVENT_HANDLER.with(|_ev|{_ev.borrow_mut()(Event::Resize(width, height))});
}

use js_sys::Date;


fn print<T: Into<JsValue>>(v: T) {
    console::log_1(&v.into());
}

fn print2<T: Into<JsValue>>(v: T, v2: T) {
    console::log_2(&v.into(), &v2.into());
}

pub fn create_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(JsValue::from_str(
            &gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".into()),
        ))
    }
}

pub fn setup_shaders(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vertex_shader_source = "
        attribute vec3 coordinates;
        void main(void) {
            gl_Position = vec4(coordinates, 1.0);
        }
        ";

    let fragment_shader_source = "
        precision mediump float;
        
        void main(void) {
            gl_FragColor = vec4(1.0, 0.5, 0.0, 1.0);
        }
        ";

    let vertex_shader = create_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        vertex_shader_source,
    )
    .unwrap();
    let fragment_shader = create_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        fragment_shader_source,
    )
    .unwrap();

    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vertex_shader);
    gl.attach_shader(&shader_program, &fragment_shader);
    gl.link_program(&shader_program);

    if gl
        .get_program_parameter(&shader_program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        gl.use_program(Some(&shader_program));
        Ok(shader_program)
    } else {
        return Err(JsValue::from_str(
            &gl.get_program_info_log(&shader_program)
                .unwrap_or_else(|| "Unknown error linking program".into()),
        ));
    }
}

pub fn setup_vertices(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram) {
    let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
    let vertex_buffer = gl.create_buffer().unwrap();

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    let coordinates_location = gl.get_attrib_location(&shader_program, "coordinates");

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(
        coordinates_location as u32,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.enable_vertex_attrib_array(coordinates_location as u32);
}

#[wasm_bindgen]
pub fn main(canvas: &str) {

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl").unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    gl.viewport(
        0,
        0,
        canvas.width().try_into().unwrap(),
        canvas.height().try_into().unwrap(),
    );

    let shader_program: WebGlProgram = setup_shaders(&gl).unwrap();
    let vertices: [f32; 9] = [
        0.0, 1.0, 0.0, // top
        -1.0, -1.0, 0.0, // bottom left
        1.0, -1.0, 0.0, // bottom right
    ];

    setup_vertices(&gl, &vertices, &shader_program);

    print("AAAA");
    gl.draw_arrays(
        WebGlRenderingContext::LINE_LOOP,
        0,
        (vertices.len() / 3) as i32,
    );
  
}