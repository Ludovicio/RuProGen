use iced::{
    mouse,
    widget::{
        button,
        canvas::{self, Canvas, Event, Frame, Geometry, Program, Image as CanvasImage},
        row, column, slider, container, text, horizontal_rule,
    },
    widget::image::Handle, widget::scrollable::Scrollbar, widget::scrollable::Direction,
    Element, Length, Rectangle,
};

use noise::{NoiseFn, Perlin};
use rand::Rng;

fn fractal_noise(perlin: &Perlin, pos: [f64; 2], octaves: u32, lacunarity: f64, persistence: f64,
                    mut frequency: f64, mut amplitude: f64) -> f64 {
    let mut total = 0.0;
    // let mut frequency = 1.0;
    // let mut amplitude = 1.0;
    let mut maxvalue = 0.0;

    for _ in 0..octaves {
        total += perlin.get([pos[0] * frequency, pos[1] * frequency]) * amplitude;

        maxvalue += amplitude;
        amplitude *= persistence;
        frequency *= lacunarity;
    }

    total / maxvalue // Normalizamos a -1.0..1.0 (más o menos)
}

fn perlin_to_color(value: f64) -> [u8; 4] {
    let normalized = (((value + 1.0) / 2.0) * 255.999) as u8; // Normalizamos a 0..255
    [normalized, normalized, normalized, 255] // RGBA
}

fn main() -> iced::Result {
    iced::run("Canvas con imagen", PaintApp::update, PaintApp::view)
}

struct PaintApp {
    image: Option<(u32, u32, Vec<u8>)>, // ancho, alto, pixels RGBA
    octaves: u32,
    lacunarity: u32,
    persistence: u32,
    frequency: u32,
    amplitude: u32,
    img_width: u32,
    img_height: u32,
}

impl Default for PaintApp {
    fn default() -> Self {
        PaintApp {
            image: None,
            octaves: 8,
            lacunarity: 20,
            persistence: 50,
            frequency: 50,
            amplitude: 50,
            img_width: 800,
            img_height: 600,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Clear,
    ApplyTestImage,
    OctavesChanged(u32),
    LacunarityChanged(u32),
    PersistenceChanged(u32),
    DAmplitudeChanged(u32),
    DFrequencyChanged(u32),
    ImgWidthChanged(u32),
    ImgHeightChanged(u32),
}

impl PaintApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::Clear => {
              self.image = None;
            },
            Message::ApplyTestImage => {
                let randnum = rand::thread_rng().gen();
                let mut pixels = Vec::with_capacity((self.img_width * self.img_height * 4) as usize);
                let perlin = Perlin::new(randnum);
                let dlacunarity:  f64 = (self.lacunarity as f64) / 10.0;
                let dpersistence: f64 = (self.persistence as f64) / 100.0;
                let d_amplitude:   f64 = (self.amplitude as f64) / 100.0;
                let d_frequency:   f64 = (self.frequency as f64) / 100.0;

                for j in 0..(self.img_height) {
                  for i in 0..(self.img_width) {
                    let x = i as f64 / self.img_width as f64;
                    let y = j as f64 / self.img_height as f64;
                    let prev = fractal_noise(&perlin, [x, y], self.octaves, dlacunarity, dpersistence, d_frequency, d_amplitude);
                    pixels.extend_from_slice(&perlin_to_color(prev));
                    //let value: u8 = (prev * 255.999) as u8;
                    //pixels.extend_from_slice(&[value, value, value, 255]);
                  }
                }

                self.image = Some((self.img_width, self.img_height, pixels));
            },
            Message::OctavesChanged(val) => self.octaves = val,
            Message::LacunarityChanged(val) => self.lacunarity = val,
            Message::PersistenceChanged(val) => self.persistence = val,
            Message::DAmplitudeChanged(val) => self.amplitude = val,
            Message::DFrequencyChanged(val) => self.frequency = val,
            Message::ImgWidthChanged(val) => self.img_width = val,
            Message::ImgHeightChanged(val) => self.img_height = val,
        }
    }

    fn view(&self) -> Element<Message> {
        use iced::widget::scrollable;
        let canvas = Canvas::new(DotsProgram {
            image: self.image.clone(),
        })
        .width(Length::Fixed(self.img_width as f32))
        .height(Length::Fixed(self.img_height as f32));

        let scrollable_canvas = scrollable(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .direction(Direction::Both { vertical: Scrollbar::new(), horizontal: Scrollbar::new() });
            
        let octaves_slider = container(
            slider(1..=20, self.octaves, Message::OctavesChanged)
                .default(8u32)
                .shift_step(1u32),
        )
        .width(250);
        let octaves_slider_text = text(format!("Octavas: {}", self.octaves));

        let lacunarity_slider = container(
            slider(1..=40, self.lacunarity, Message::LacunarityChanged)
                .default(8u32)
                .shift_step(1u32),
        )
        .width(250);
        let lacunarity_slider_text = text(format!("lacunaridad: {}", (self.lacunarity as f64) / 10.0));

        let persistence_slider = container(
            slider(1..=100, self.persistence, Message::PersistenceChanged)
                .default(8u32)
                .shift_step(1u32),
        )
        .width(250);
        let persistence_slider_text = text(format!("Persistencia: {}", (self.persistence as f64) / 100.0));

        let d_amplitude_slider = container(
            slider(1..=1000, self.amplitude, Message::DAmplitudeChanged)
                .default(50u32)
                .shift_step(1u32),
        )
        .width(250);
        let d_amplitude_slider_text = text(format!("Amplitud: {}", (self.amplitude as f64) / 100.0));

        let d_frequency_slider = container(
            slider(1..=10000, self.frequency, Message::DFrequencyChanged)
                .default(50u32)
                .shift_step(1u32),
        )
        .width(250);
        let d_frequency_slider_text = text(format!("Frecuencia: {}", (self.frequency as f64) / 100.0));

        let img_width_slider = container(
            slider(50..=2000, self.img_width, Message::ImgWidthChanged)
                .default(800u32)
                .shift_step(100u32),
        )
        .width(250);
        let img_width_slider_text = text(format!("Ancho imagen: {}", self.img_width));

        let img_height_slider = container(
            slider(50..=2000, self.img_height, Message::ImgHeightChanged)
                .default(600u32)
                .shift_step(100u32),
        )
        .width(250);
        let img_height_slider_text = text(format!("Alto imagen: {}", self.img_height));

        let controls = column![
            button("Limpiar").on_press(Message::Clear),
            button("Aplicar imagen de prueba").on_press(Message::ApplyTestImage),
            octaves_slider_text, octaves_slider,
            horizontal_rule(1),
            lacunarity_slider_text, lacunarity_slider,
            horizontal_rule(1),
            persistence_slider_text, persistence_slider,
            horizontal_rule(1),
            d_amplitude_slider_text, d_amplitude_slider,
            horizontal_rule(1),
            d_frequency_slider_text, d_frequency_slider,
            horizontal_rule(1),
            img_width_slider_text, img_width_slider,
            horizontal_rule(1),
            img_height_slider_text, img_height_slider,
        ]
        .padding(12)
        .spacing(12)
        .width(Length::Shrink);

        let viewer = column![
            scrollable_canvas,
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
    image: Option<(u32, u32, Vec<u8>)>,
}

impl Program<Message> for DotsProgram {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
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
        let mut frame = Frame::new(renderer, bounds.size());
        // Dibujar imagen si existe
        if let Some((width, height, pixels)) = &self.image {
            self.draw_image_from_rgba(&mut frame, *width, *height, pixels);
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

// TODO revisar, entender y reescribit pintado.
// Centralizar valores minimos y máximos de sliders y demás controles
