use amber_light::ember::{self, Fireplace};
use canvas::digital_canvas::*;
use nannou::prelude::*;
use nannou_egui::{self, Egui};

struct Model {
    my_canvas: DigitalCanvas,
    fireplace: Fireplace,
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}

fn update(_app: &App, model: &mut Model, update: Update) {
    println!("{:?}", model.fireplace.heatmap);
    model.fireplace.update_embers();
    model.fireplace.find_heatmap();
    let g = &model.fireplace.settings.g;
    for (j, column) in &mut model.my_canvas.pixels.iter_mut().enumerate() {
        for (i, pixel) in &mut column.iter_mut().enumerate() {
            let color = g.at(model.fireplace.heatmap[i][j] as f64).to_rgba8();
            pixel.set_rgb((color[0], color[1], color[2]));
            //pixel.cycle();
        }
    }
}

fn model(app: &App) -> Model {
    // Create window
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .view(view)
        .build()
        .unwrap();
    // let window_id = app
    //     .new_window()
    //     .view(view)
    //     .raw_event(raw_window_event)
    //     .build()
    //     .unwrap();
    // let window = app.window(window_id).unwrap();

    //let egui = Egui::from_window(&window);

    let mut model = Model {
        my_canvas: DigitalCanvas::new(),
        fireplace: Fireplace::new(),
    };
    model.fireplace.find_heatmap();
    return model;
}

fn _raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    //model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    assert_ne!(model.fireplace.heatmap, [[0.0; 10]; 10]);
    let draw = app.draw();

    // Clear the background to blue.
    //draw.background().color(CORNFLOWERBLUE);
    let canv = model.my_canvas;

    let win = app.window_rect();
    let x_y_offset = (win.bottom_right() - win.top_left()) / 10.0;
    let offset = x_y_offset / 2.0;
    for (i, row) in canv.pixels.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            draw.rect()
                .x_y(
                    win.top_left()[0] + offset[0] + ((i) as f32 * x_y_offset[0]),
                    win.top_left()[1] + offset[1] + ((j) as f32 * x_y_offset[1]),
                )
                .w(x_y_offset[0])
                .h(x_y_offset[1])
                .rgb8(pixel.red, pixel.green, pixel.blue);
            //draw.to_frame(app, &frame).unwrap();
            //pixel.cycle();
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
