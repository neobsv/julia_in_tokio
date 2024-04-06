use image::{ImageBuffer, Rgb};
use itertools::Itertools;
use num_complex::Complex;
use std::{env, {thread, thread::JoinHandle}};


fn color_generator(z0: Complex<f64>, c: Complex<f64>, iterations: u32) -> Rgb<u8> {
    let mut z = z0;
    let mut current = 0;

    while z.norm() <= 2.0 && current < iterations {
        z = z * z + c;
        current += 1;
    }

    let current = current as f64;
    let iterations = iterations as f64;

    let color = match current == iterations {
        true => Rgb([0, 0, 0]),
        false => Rgb([
            ((current / iterations).powf(0.2) * 255.0) as u8,
            ((current / iterations).powf(0.4) * 255.0) as u8,
            (1.0 - (current / iterations).powf(0.9) * 255.0) as u8,
        ]),
    };

    color
}

fn generate_image_buffer(
    width: u32,
    height: u32,
    iterations: u32,
    scale: f64,
    zoom: f64,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image_buffer = ImageBuffer::new(width, height);

    let c = Complex::new(0.353343, 0.5133225);
    let (w, h) = (width as f64, height as f64);
    let (c_w, c_h) = ((w / zoom) as u32, (h / zoom) as u32);

    let mut thread_matrix: Vec<Vec<Option<JoinHandle<_>>>> = Vec::new();

    for x in 0..width as usize {
        thread_matrix.push(Vec::new());
        for y in 0..height as usize {
            let cx = (x as f64 - 0.5 * c_w as f64) * scale / w;
            let cy = (y as f64 - 0.5 * c_h as f64) * scale / h;
            let z = Complex::new(cx, cy);
            let handle = Some(thread::spawn(
                move || { color_generator(z, c, iterations) },
            ));
            thread_matrix[x].push(handle);
        }
    }

    for x in 0..width as usize {
        for y in 0..height as usize {
            let color: JoinHandle<_> = thread_matrix[x][y].take().unwrap();
            let res_color: Rgb<u8> = color.join().unwrap();
            image_buffer.put_pixel(x as u32, y as u32, res_color);
        }
    }

    image_buffer
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("Refer to the README doc for usage details!");
        return Ok(());
    }

    let (width, height) = args[1]
        .split('x')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .next_tuple()
        .unwrap();

    let (capture_width, capture_height) = args[2]
        .split('x')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .next_tuple()
        .unwrap();

    let iterations = args[3].parse::<u32>().unwrap();
    let scale = args[4].parse::<f64>().unwrap();
    let image_buffer = generate_image_buffer(capture_width, capture_height, iterations, scale, 1.0);

    let image: image::RgbImage =
        image::ImageBuffer::from_vec(width, height, image_buffer.to_vec()).unwrap();
    image.save("julia.png").unwrap();

    Ok(())
}
