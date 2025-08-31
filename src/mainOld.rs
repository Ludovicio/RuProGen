use minifb::{Key, Window, WindowOptions};
use noise::{NoiseFn, Perlin};

// const WIDTH: usize = 10;
// const HEIGHT: usize = 10;

fn to_argb(val: u32) -> u32 {
    let alpha = 0x00; // Transparente
    let rgb = (val << 16) | (val << 8) | val; // R=G=B=val
    (alpha << 24) | rgb
}

fn matrix2buffer_color( matrix: &Vec<Vec<u32>>, colors: &Vec<u32>, xscale: usize, yscale: usize ) -> Vec<u32> {
    let width: usize = matrix[0].len();
    let height: usize = matrix.len();
    let window_width = width * xscale;
    let window_height = height * yscale;

    let mut buffer = vec![0; window_width * window_height];

    for y in 0..height {
        for x in 0..width {
            let color = colors[matrix[y][x] as usize];
            for dy in 0..yscale {
                for dx in 0..xscale {
                    let px = x * xscale + dx;
                    let py = y * yscale + dy;
                    buffer[py * window_width + px] = color;
                }
            }
        }
    }
    buffer
}

fn matrix2buffer_simple( matrix: &Vec<Vec<u32>>) -> Vec<u32> {
    let width: usize = matrix[0].len();
    let height: usize = matrix.len();
    let window_width = width;
    let window_height = height;

    let mut buffer = vec![0; window_width * window_height];

    for y in 0..height {
        for x in 0..width {
            buffer[y * window_width + x] = to_argb(matrix[y][x]);
        }
    }
    buffer
}

fn print_typeof<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn fractal_noise(perlin: &Perlin, pos: [f64; 2], octaves: u32, lacunarity: f64, persistence: f64) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut maxvalue = 0.0;

    for _ in 0..octaves {
        total += perlin.get([pos[0] * frequency, pos[1] * frequency]) * amplitude;

        maxvalue += amplitude;
        amplitude *= persistence;
        frequency *= lacunarity;
    }

    total / maxvalue // Normalizamos a -1.0..1.0 (más o menos)
}

fn main() {

    let xscale : usize = 1;
    let yscale : usize = 1;

    let width: usize = 1000;// matrix[0].len();
    let height: usize = 1000;// matrix.len();
    // let window_width = width * xscale;
    // let window_height = height * yscale;

    let perlin = Perlin::new(0);
    

    // let matrix : Vec<Vec<u32>> = vec![
    //     vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1],
    //     vec![1, 2, 3, 0, 1, 2, 3, 0, 1, 2],
    //     vec![2, 3, 0, 1, 2, 3, 0, 1, 2, 3],
    //     vec![3, 0, 1, 2, 3, 0, 1, 2, 3, 0],
    //     vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1],
    //     vec![1, 2, 3, 0, 1, 2, 3, 0, 1, 2],
    //     vec![2, 3, 0, 1, 2, 3, 0, 1, 2, 3],
    //     vec![3, 0, 1, 2, 3, 0, 1, 2, 3, 0],
    //     vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1],
    //     vec![1, 2, 3, 0, 1, 2, 3, 0, 1, 2],
    // ];
    
    // TODO solo se ve gris. Hay que ver la salida de  perlin.get
    println!("perlin: {:>3}", perlin.get([0.0,0.0]));
    println!("perlin: {:>3}", perlin.get([0.1,0.0]));
    println!("perlin: {:>3}", perlin.get([0.0,0.1]));
    println!("perlin: {:>3}", perlin.get([7.0,2.0]));
    
    // Generar la matriz
    let matrix: Vec<Vec<u32>> = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    // Coordenadas escaladas
                    let nx = (x * xscale) as f64 / (width as f64);
                    let ny = (y * yscale) as f64 / (height as f64);

                    // Obtener valor Perlin en [-1.0, 1.0]
                    // let value = perlin.get([nx, ny]);

                    let value = fractal_noise(&perlin, [nx, ny], 8, 4.0, 0.8);

                    // Escalar a rango 0-255 y convertir a u32
                    let mapped = ((value + 1.0) * 0.5 * 255.0).round() as u32;
                    mapped
                })
                .collect()
        })
        .collect();

    let colors : Vec<u32> = vec![
        0x00FFFFFF, // Blanco
        0x00FF0000, // Rojo
        0x0000FF00, // Verde
        0x000000FF, // Azul
    ];

    print_typeof(&colors);

    // let scale = 50;
    // let window_width = WIDTH * scale;
    // let window_height = HEIGHT * scale;

    // let mut buffer = vec![0; window_width * window_height];

    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         let color = colors[matrix[y][x] as usize];
    //         for dy in 0..scale {
    //             for dx in 0..scale {
    //                 let px = x * scale + dx;
    //                 let py = y * scale + dy;
    //                 buffer[py * window_width + px] = color;
    //             }
    //         }
    //     }
    // }

    // let buffer = matrix2buffer_color(&matrix, &colors, xscale, yscale);
    let buffer = matrix2buffer_simple(&matrix);
    

    let mut window = Window::new("Matriz Colorida", width, height, WindowOptions::default())
        .expect("No se pudo crear la ventana");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

// TODO perlin furula.
// Estamos configurando icde para meter controles y configurar el perlin al vuelo.
// https://book.iced.rs/first-steps.html