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
        }
    }

    fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r, g, b, a);    
    }

    fn clear(&self) {
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    fn create_vertex_shader(&mut self, src: &str) {
        let vs = self.gl.create_shader(WebGlRenderingContext::VERTEX_SHADER).unwrap();
        self.gl.shader_source(&vs, src);
        self.gl.compile_shader(&vs);
        self.vertex_shader.push(vs);
    }

    fn create_fragment_shader(&mut self, src: &str) {
        let fs = self.gl.create_shader(WebGlRenderingContext::FRAGMENT_SHADER).unwrap();
        self.gl.shader_source(&fs, src);
        self.gl.compile_shader(&fs);
        self.fragment_shader.push(fs);
    }

    fn create_program_shader(&mut self, vertex_shader_index: usize, fragment_shader_index: usize) {
            let program = self.gl.create_program().unwrap();
            self.gl.attach_shader(&program, &self.vertex_shader[vertex_shader_index]);
            self.gl.attach_shader(&program, &self.fragment_shader[fragment_shader_index]);
            self.gl.link_program(&program);
    }

    fn create_program_from(&mut self, vertex_shader: &str, fragment_shader: &str) {

        let vs = self.gl.create_shader(WebGlRenderingContext::VERTEX_SHADER).unwrap();
        self.gl.shader_source(&vs, vertex_shader);
        self.gl.compile_shader(&vs);
        let log = self.gl.get_shader_info_log(&vs).unwrap();
        print(log);

        let fs = self.gl.create_shader(WebGlRenderingContext::FRAGMENT_SHADER).unwrap();
        self.gl.shader_source(&fs, fragment_shader);
        self.gl.compile_shader(&fs);
        let log = self.gl.get_shader_info_log(&vs).unwrap();
        print(log);


        let program = self.gl.create_program().unwrap();
        self.gl.attach_shader(&program, &vs);
        self.gl.attach_shader(&program, &fs);
        self.gl.link_program(&program);


        let log = self.gl.get_program_info_log(&program).unwrap();
        print(log);

        self.shader_program.push(program);

        self.gl.delete_shader(Some(&vs));
        self.gl.delete_shader(Some(&fs));
    }


    fn create_vertex_buffer_dynamic(&mut self, vertex: Vec<f32>) {
        let vertices_array = unsafe { js_sys::Float32Array::view(&vertex) };
        let vertex_buffer = self.gl.create_buffer().unwrap();

        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        self.gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
        );

        let coordinates_location = self.gl.get_attrib_location(&self.shader_program[0], "coordinates");

        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        self.gl.vertex_attrib_pointer_with_i32(
        coordinates_location as u32,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
        );

        self.gl.enable_vertex_attrib_array(coordinates_location as u32);
        self.gl.use_program(Some(&self.shader_program[0]));
        
    }

    fn create_vertex_buffer_static(&mut self) {

    }


}

enum Event<'k> {
    KeyUp(&'k str),
    KeyDown(&'k str),
    Update(),
    Resize(usize, usize),
    Touch,
    Mouse,
}

enum EventTouch {
    Start,
    Move,
    End,
}

enum MouseEvent {
    Start,
    End,
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

    if ev.chars().next() == Some('u') {
        //print("up");
        EVENT_HANDLER.with(|_ev|{_ev.borrow_mut()(Event::KeyUp(&ev[1..]))});
    } else {
        //print("down");
        EVENT_HANDLER.with(|_ev|{_ev.borrow_mut()(Event::KeyDown(&ev[1..]))});
    }
    
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








#[wasm_bindgen]
pub fn main(canvas: &str) {

    
let vertices = vec![
    0.0, 0.5, 0.0, // top
    -0.5, -0.5, 0.0, // bottom left
    0.5, -0.5, 0.0, // bottom right
];



    let mut gl = Context::new(canvas); 

    
gl.create_program_from(VERTEX_SHADER_1, FRAGMENT_SHADER_1);
gl.create_vertex_buffer_dynamic(vertices);


        //gl.set_clear_color(0.0, 0.0, 0.0, 1.0);

        event(move |mut ev| {    

            match ev {

                Event::KeyUp(key) => {
                    gl.clear();
                }


                Event::Update() => {
                      
                    gl.clear();
            
            
                    gl.gl.draw_arrays(
                        WebGlRenderingContext::LINE_LOOP,
                        0,
                        (3) as i32,
                    );
                }


                Event::Resize(width, height) => {

                    
                
                    gl.clear();
                }

                _ => ()
            }

        });
}