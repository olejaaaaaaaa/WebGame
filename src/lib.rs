#![allow(warnings)]

use std::collections::HashMap;

use wasm_bindgen::prelude::*;

mod shaders;
use shaders::*;


mod kernel;
use kernel::{event, print, Context, Event};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};


#[wasm_bindgen]
pub fn main(canvas: &str) {








    let mut gl = Context::new("canvas");
    gl.set_clear_color(1.0, 0.5, 0.0, 1.0);
    gl.clear();
    


    


    let mut x = 0.0f32;
    let mut y = 0.0f32;

        event(move |mut ev| {

            let mut vertices = vec![
                0.0 + x, 0.5 + y, 0.0, 
                -0.5 + x, -0.5 + y, 0.0, 
                0.5 + x, -0.5 + y, 0.0, 
            ];    

            match ev {


                Event::KeyUp(key) => {
                    x += 0.1;
                    y += 0.1;
                }

                Event::TouchDown(_, _) => {
                    print("Коснулся");
                    x += 0.1;
                    y += 0.1;
                }

                Event::KeyDown(key) => {
                    
                }

                Event::Update() => {

                    gl.clear();

                    gl.create_render_object(
                        VERTEX_SHADER_1, 
                        FRAGMENT_SHADER_1, 
                        HashMap::from( [("pos", vertices)] ), 
                        WebGlRenderingContext::TRIANGLES
                    );

                    gl.draw();

                    
                }

                Event::Resize(width, height) => {
                    
                }

                _ => ()
            }

        });
}