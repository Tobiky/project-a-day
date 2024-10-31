use raylib::{self, color::Color, drawing::RaylibDraw, texture::Image};

fn main() {
    let (screen_width, screen_height) = (640, 480);
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Hello, World")
        .build();

    let cabbage_image = Image::load_image("cabbage.png").unwrap();
    let cabbage_texture = rl.load_texture_from_image(&thread, &cabbage_image).unwrap();
    drop(cabbage_image);

    while !rl.window_should_close() {
        let mut draw_handle = rl.begin_drawing(&thread);

        draw_handle.clear_background(Color::BLACK);
        draw_handle.draw_texture(
            &cabbage_texture,
            screen_width / 2 - cabbage_texture.width / 2,
            screen_height / 2 - cabbage_texture.height / 2,
            Color::WHITE,
        );
    }
}
