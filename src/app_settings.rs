use macroquad::{miniquad::conf::Icon, prelude::*};

fn load_img(bytes: &'static [u8]) -> Image {
    Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap()
}

fn populate_array(img: Image, array: &mut [u8]) {
    let mut index: usize = 0;
    for pixel in img.get_image_data() {
        for value in pixel.iter() {
            array[index] = *value;
            index += 1;
        }
    }
}

fn set_icon() -> Icon {
    let mut array_small: [u8; 1024] = [0; 1024];
    let mut array_medium: [u8; 4096] = [0; 4096];
    let mut array_big: [u8; 16384] = [0; 16384];

    populate_array(
        load_img(include_bytes!("../icons/icon_16.png")),
        &mut array_small,
    );
    populate_array(
        load_img(include_bytes!("../icons/icon_32.png")),
        &mut array_medium,
    );
    populate_array(
        load_img(include_bytes!("../icons/icon_64.png")),
        &mut array_big,
    );

    Icon {
        small: array_small,
        medium: array_medium,
        big: array_big,
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "QuadVec".into(),
        window_width: 800,
        window_height: 600,
        sample_count: 8,
        icon: Some(set_icon()),
        ..Default::default()
    }
}

pub struct FreedomCamera2D {
    pub camera: Camera2D,
    pub zoom: f32,
    pub zoom_value: f32,
}

impl FreedomCamera2D {
    pub fn new() -> Self {
        Self {
            camera: Camera2D::default(),
            zoom: 2.0,
            zoom_value: 0.2,
        }
    }

    pub fn update(&mut self) {
        self.camera.zoom = vec2(self.zoom / screen_width(), self.zoom / screen_height());

        if screen_width() < 320. {
            request_new_screen_size(320., screen_height());
        }

        if screen_height() < 240. {
            request_new_screen_size(screen_width(), 240.);
        }

        if mouse_wheel().1 > 0. {
            self.zoom = (self.zoom * 10.).round() / 10.;

            self.zoom += self.zoom_value;
        } else if mouse_wheel().1 < 0. && self.zoom > self.zoom_value {
            self.zoom = (self.zoom * 10.).round() / 10.;

            self.zoom -= self.zoom_value;

            if self.zoom < self.zoom_value {
                self.zoom = self.zoom_value;
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let mouse_pos = mouse_delta_position();

            self.camera.target.x += mouse_pos.x * screen_width() / self.zoom;
            self.camera.target.y += mouse_pos.y * screen_height() / self.zoom;
        }

        if is_key_pressed(KeyCode::Space) {
            self.camera.target = Vec2::ZERO;
            self.zoom = 2.0;
        }
    }

    pub fn world_mpos(&self) -> Vec2 {
        self.camera.screen_to_world(Vec2 {
            x: mouse_position().0,
            y: mouse_position().1,
        })
    }
}
