#![allow(warnings)]

use std::convert::TryInto;
use std::cell::RefCell;

use shaders::*;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram, console};
use web_sys::*;
extern crate js_sys;

mod shaders;



#[wasm_bindgen]
pub fn Event(ev: &str) {
    
    panic!("SUKA");
    //if ev != "w" {
       // console::log_1(&js_sys::JsString::from("RUST"));
    
    //console::log_1(&ev.into());

}


#[wasm_bindgen]
pub fn main() {
   
}