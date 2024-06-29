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


enum Event<'k> {
    KeyUp(&'k str),
    KeyDown(&'k str),

    Update(),
    FixedUpdate(),
    Resize(usize, usize),

    TouchUp(usize, usize),
    TouchDown(usize, usize),

    MouseUp(bool, usize, usize),
    MouseDown(bool, usize, usize),
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
pub fn keyboard_event_up(ev: &str) {
    EVENT_HANDLER.with(|_ev|{_ev.borrow_mut()(Event::KeyUp(ev))});    
}

#[wasm_bindgen]
pub fn keyboard_event_down(ev: &str) {
    EVENT_HANDLER.with(|_ev|{_ev.borrow_mut()(Event::KeyDown(ev))});    
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


#[derive(Clone, Debug)]
struct Context {
    gl: WebGlRenderingContext,
    vertex_buffer: Vec<WebGlBuffer>,
    shader_program: Vec<WebGlProgram>,
    vertex_shader: Vec<WebGlShader>,
    fragment_shader: Vec<WebGlShader>,
    elements: Vec<[u8; 2]>,                 // index shader program, index vertex_buffer
    clear_color: (f32, f32, f32, f32),
    vertex: Vec<Vec<f32>>,
    uniforms: Vec<WebGlUniformLocation>,
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

        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        Self { 
            gl,
            vertex_buffer: vec![],
            shader_program: vec![],
            vertex_shader: vec![],
            fragment_shader: vec![],
            elements: vec![],
            clear_color: (0.0, 0.0, 0.0, 1.0),
            vertex: vec![vec![]],
            uniforms: vec![],
        }
    }

    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r, g, b, a);    
    }

    fn clear(&self) {
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }


    fn draw(&self) {
        
    }


}




#[wasm_bindgen]
pub fn main(canvas: &str) {

    
    let vertices = vec![
        0.0, 0.5, 0.0, 
        -0.5, -0.5, 0.0, 
        0.5, -0.5, 0.0, 
    ];


        event(move |mut ev| {    

            match ev {

                Event::KeyUp(key) => {
                    
                }

                Event::KeyDown(key) => {
                    
                }

                Event::Update() => {
                    Event::FixedUpdate();  
                }

                Event::FixedUpdate() => {
                    print("FIXED");
                }

                Event::Resize(width, height) => {
                    
                }

                _ => ()
            }

        });
}