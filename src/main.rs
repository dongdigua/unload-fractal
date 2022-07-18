use unload_fractal::image;

fn main() {
    let filename = String::from("fractal.png");
    image::generate(1024, 1024, &filename);
    println!("saved: {}", filename);
}
