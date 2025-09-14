use iced::{
    mouse,
    widget::{
        button,
        canvas::{self, Canvas, Event, Frame, Geometry, Path, Program, Image as CanvasImage},
        row, column, slider, container, text, horizontal_rule,
    },
    widget::image::Handle,
    Color, Element, Length, Point, Rectangle,
};

use noise::{NoiseFn, Perlin};
use rand::Rng;

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

fn main() -> iced::Result {
    iced::run("Canvas con imagen", PaintApp::update, PaintApp::view)
}

#[derive(Default)]
struct PaintApp {
    // TODO: hay que eliminar points de toda la app
    points: Vec<Point>,
    image: Option<(u32, u32, Vec<u8>)>, // ancho, alto, pixels RGBA
    octaves: u32,
    lacunarity: u32,
    persistence: u32,
    imgWidth: u32,
    imgHeight: u32,
}


// TODO: Implementar Default para PaintApp esto es solo un ejemplo
// impl Default for struct PaintApp {
//     fn default() -> Self {
//         Config {
//             width: 1920,
//             height: 1080,
//             fullscreen: false,
//         }
//     }
// }

#[derive(Debug, Clone)]
enum Message {
    Clicked(Point),
    Clear,
    ApplyTestImage,
    OctavesChanged(u32),
    LacunarityChanged(u32),
    PersistenceChanged(u32),
    ImgWidthChanged(u32),
    ImgHeightChanged(u32),
}

// TODO: Sliders para parámetros.

impl PaintApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::Clicked(pos) => self.points.push(pos),
            Message::Clear => {
              self.points.clear();
              self.image = None;
            },
            Message::ApplyTestImage => {
                // Creamos una imagen de prueba: 10x20 con patrón simple
                let randnum = rand::thread_rng().gen_range(0..100);
                // let width = 300;
                // let height = 300;
                let mut pixels = Vec::with_capacity((self.imgWidth * self.imgHeight * 4) as usize);
                let perlin = Perlin::new(randnum);
                let dlacunarity: f64  = (self.lacunarity as f64) / 10.0;
                let dpersistence: f64 = (self.persistence as f64) / 100.0;

                for j in 0..(self.imgHeight) {
                  for i in 0..(self.imgWidth) {
                    let x = i as f64 / self.imgWidth as f64;
                    let y = j as f64 / self.imgHeight as f64;
                    // let prev = fractal_noise(&perlin, [x, y], 8, 6.0, 0.9);
                    let prev = fractal_noise(&perlin, [x, y], self.octaves, dlacunarity, dpersistence);
                    let value: u8 = (prev * 255.999) as u8;
                    pixels.extend_from_slice(&[value, value, value, 255]);
                  }
                }

                self.image = Some((self.imgWidth, self.imgHeight, pixels));
            },
            Message::OctavesChanged(val) => self.octaves = val,
            Message::LacunarityChanged(val) => self.lacunarity = val,
            Message::PersistenceChanged(val) => self.persistence = val,
            Message::ImgWidthChanged(val) => self.imgWidth = val,
            Message::ImgHeightChanged(val) => self.imgHeight = val,
        }
    }

    fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(DotsProgram {
            points: self.points.clone(),
            image: self.image.clone(),
        })
        .width(Length::Fill)
        .height(Length::Fill);
        let octavesSlider = container(
            slider(1..=50, self.octaves, Message::OctavesChanged)
                .default(8u32)
                .shift_step(1u32),
        )
        .width(250);
        let octavesSliderText = text(format!("Octavas: {}", self.octaves));

        let lacunaritySlider = container(
            slider(1..=40, self.lacunarity, Message::LacunarityChanged)
                .default(8u32)
                .shift_step(1u32),
        )
        .width(250);
        let lacunaritySliderText = text(format!("lacunaridad: {}", (self.lacunarity as f64) / 10.0));

        let persistenceSlider = container(
            slider(1..=100, self.persistence, Message::PersistenceChanged)
                .default(8u32)
                .shift_step(1u32),
        )
        .width(250);
        let persistenceSliderText = text(format!("Persistencia: {}", (self.persistence as f64) / 100.0));

        let imgWidthSlider = container(
            slider(50..=2000, self.imgWidth, Message::ImgWidthChanged)
                .default(800u32)
                .shift_step(100u32),
        )
        .width(250);
        let imgWidthSliderText = text(format!("Ancho imagen: {}", self.imgWidth));

        let imgHeightSlider = container(
            slider(50..=2000, self.imgHeight, Message::ImgHeightChanged)
                .default(600u32)
                .shift_step(100u32),
        )
        .width(250);
        let imgHeightSliderText = text(format!("Alto imagen: {}", self.imgHeight));

        let controls = column![
            button("Limpiar").on_press(Message::Clear),
            button("Aplicar imagen de prueba").on_press(Message::ApplyTestImage),
            octavesSliderText, octavesSlider,
            horizontal_rule(1),
            lacunaritySliderText, lacunaritySlider,
            horizontal_rule(1),
            persistenceSliderText, persistenceSlider,
            horizontal_rule(1),
            imgWidthSliderText, imgWidthSlider,
            horizontal_rule(1),
            imgHeightSliderText, imgHeightSlider,
        ]
        .padding(12)
        .spacing(12);
        //.width(Length::Shrink);

        let viewer = column![
            canvas,
        ]
        .padding(12)
        .spacing(12);

        let content = row![
            controls,
            viewer,
        ]
        .padding(12)
        .spacing(12);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Clone)]
struct DotsProgram {
    // TODO: hay que eliminar points de toda la app
    points: Vec<Point>,
    image: Option<(u32, u32, Vec<u8>)>,
}

impl Program<Message> for DotsProgram {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if let Some(pos) = cursor.position_in(bounds) {
                return (
                    canvas::event::Status::Captured,
                    Some(Message::Clicked(pos)),
                );
            }
        }
        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        // TODO: Ya no vamos a pintar puntos, solo la imagen
        let mut frame = Frame::new(renderer, bounds.size());

        // Dibujar imagen si existe
        if let Some((width, height, pixels)) = &self.image {
            self.draw_image_from_rgba(&mut frame, *width, *height, pixels);
        }

        // Dibujar puntos clicados
        for &p in &self.points {
            let dot = Path::circle(p, 4.0);
            frame.fill(&dot, Color::from_rgb(0.2, 0.4, 0.9));
        }

        vec![frame.into_geometry()]
    }
}

impl DotsProgram {
    fn draw_image_from_rgba(
        &self,
        frame: &mut Frame,
        width: u32,
        height: u32,
        pixels: &[u8],
    ) {
        // Crear handle desde RGBA
        let handle = Handle::from_rgba(width, height, pixels.to_vec());

        // Crear CanvasImage
        let canvas_img = CanvasImage::new(handle);

        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
        };

        // Dibujar imagen
        frame.draw_image(bounds, canvas_img);
    }
}
