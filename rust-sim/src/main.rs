
use zcr_wsn::config::{AREA_HEIGHT, AREA_WIDTH, CH_PROBABILITY, NUM_NODES};
use zcr_wsn::leach::LEACH;
use zcr_wsn::simulator::SIMULATOR;
use macroquad::prelude::*;

const SIM_FPS: f32 = 10.0; // simulation steps per second


fn window_conf() -> Conf {
    Conf {
        window_title: "WSN Simulator".to_owned(),
        window_width: 1200,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulator = SIMULATOR::new(AREA_WIDTH, AREA_HEIGHT, NUM_NODES);
    let mut protocol = LEACH::new(CH_PROBABILITY);

    let mut acc = 0.0;
    let step = 1.0 / SIM_FPS;

    loop {
        let dt = get_frame_time();
        acc += dt;

        while acc >= step {
            simulator.update(&mut protocol);
            acc -= step;
        }

        clear_background(BLACK);
        simulator.render();

        next_frame().await;
    }
}

