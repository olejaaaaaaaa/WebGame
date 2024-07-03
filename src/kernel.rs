
#![macro_use]
use js_sys::JsString;
use shaders::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::shaders;
extern crate js_sys;

use std::collections::HashMap;
use std::convert::TryInto;
use std::cell::RefCell;



#[derive(Clone, Debug)]
pub struct Context {
    gl: WebGlRenderingContext,
    pub elements: Vec<RenderObject>,                 
    clear_color: (f32, f32, f32, f32),
}


#[derive(Clone, Debug)]
pub struct RenderObject {
    pub shader_program: WebGlProgram,
    pub attributes: Vec<u32>,
    pub buffers: Vec<WebGlBuffer>,
    pub vertex: Vec<Vec<f32>>,
    pub vertex_count: i32,
    pub draw_type: u32,
}

impl Context {

    pub fn new(canvas: &str) -> Self {

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
            elements: vec![],
            clear_color: (0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r, g, b, a);    
    }

    pub fn clear(&self) {
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }


    pub fn draw(&self) {
        
        for render in &self.elements {

            self.gl.use_program(Some(&render.shader_program));

            for buf in &render.buffers {
                self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(buf));

                for atr in &render.attributes {
                    self.gl.enable_vertex_attrib_array(*atr);
                }
                
            }  

            self.gl.draw_arrays(render.draw_type, 0, 3);          
        }

    }

    pub fn create_render_object(
        &mut self, 
        vs: &str, 
        fs: &str, 
        vertex: HashMap<&str, Vec<f32>>,
        draw_type: u32,
    ) {        

        let vertex_shader = self.gl.create_shader(WebGlRenderingContext::VERTEX_SHADER).unwrap();
        self.gl.shader_source(&vertex_shader, &vs);
        self.gl.compile_shader(&vertex_shader);
        //print(self.gl.get_shader_info_log(&vertex_shader)); // log

        let fragment_shader = self.gl.create_shader(WebGlRenderingContext::FRAGMENT_SHADER).unwrap();
        self.gl.shader_source(&fragment_shader, &fs);
        self.gl.compile_shader(&fragment_shader);
        //print(self.gl.get_shader_info_log(&fragment_shader)); // log

        let shader_program = self.gl.create_program().unwrap();
        self.gl.attach_shader(&shader_program, &vertex_shader);
        self.gl.attach_shader(&shader_program, &fragment_shader);
        self.gl.link_program(&shader_program);
        //print(self.gl.get_program_info_log(&shader_program)); // log        

        self.gl.delete_shader(Some(&vertex_shader));
        self.gl.delete_shader(Some(&fragment_shader));

        let mut buffer: Vec<WebGlBuffer> = vec![];
        let mut _vertex: Vec<Vec<f32>> = vec![];
        let mut atr: Vec<u32> = vec![];

        for i in vertex {

            let vertex_buffer = self.gl.create_buffer().unwrap();
            let vertices_array = unsafe { js_sys::Float32Array::view(&i.1) };

            self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vertices_array,
                WebGlRenderingContext::STATIC_DRAW,
            );

            let a = self.gl.get_attrib_location(&shader_program, i.0) as u32;
            atr.push(a);        

            self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
            self.gl.vertex_attrib_pointer_with_i32(
                a,
                3,
                WebGlRenderingContext::FLOAT,
                false,
                0,
                0,
            );

            buffer.push(vertex_buffer);
            _vertex.push(i.1);
        }


        self.elements.push(RenderObject {
            shader_program: shader_program,
            attributes: atr,
            buffers: buffer,
            vertex: _vertex,
            vertex_count: 3,
            draw_type,
        });
         
    }

}




pub enum Event<'k> {
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

pub fn print<T: Into<JsValue>>(v: T) {
    console::log_1(&v.into());
}




#[wasm_bindgen]
pub fn touch_event_up(x: usize, y: usize) {
    EVENT_HANDLER.with(|_ev|{_ev.borrow_mut()(Event::TouchUp(x, y))});    
}

#[wasm_bindgen]
pub fn touch_event_down(x: usize, y: usize) {
    EVENT_HANDLER.with(|_ev|{_ev.borrow_mut()(Event::TouchDown(x, y))});    
}