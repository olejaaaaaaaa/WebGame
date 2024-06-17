


extern "C" {
   fn js_clear_screen_to_color(r: f32, g: f32, b: f32, a: f32);
}

#[no_mangle]
fn main() {
unsafe {
   js_clear_screen_to_color(0.5, 0.3, 0.0, 1.0);
}
}


