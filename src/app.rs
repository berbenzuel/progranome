use std::rc::Rc;
use slint::{ComponentHandle, Timer, TimerMode, VecModel};
use tinyaudio::{run_output_device, OutputDeviceParameters};
use crate::MainWindow;
use crate::MetronomeUnit;
use crate::app_state::AppState;


pub fn start_app() -> Result<(),  Box<dyn std::error::Error>>   {


    let main = MainWindow::new()?;
    let main_weak = main.as_weak();



    let timer = Rc::from(Timer::default());
    let audio_device_params =  OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };



    let model = Rc::from(
        VecModel::from(
            vec! [
                MetronomeUnit::new(3, 4, 120, 3),
                MetronomeUnit::new(6, 7, 150, 3)
            ]
        )
    );

    let mut app_state = AppState::new(&model);
    app_state.push(MetronomeUnit::new(4, 2, 100, 4));


    main.on_play_button_pressed(move || {
        let handle = main_weak.clone();
        if let Some(main) = handle.upgrade() {

            if main.get_playing() {
                timer.stop();
            }
            else {
                timer.start(
                    TimerMode::Repeated,
                    std::time::Duration::from_secs(1),
                    move || {
                        if let Some(ui) = handle.upgrade() {

                            //here it is crashing coz i dont have any metronomeunit, and it is trying to mod with 0-
                            
                            let numerator = ui.get_active_unit().numerator;
                            let mut beat = ui.get_actual_beat();
                            beat = ((beat) % numerator) +1;
                            ui.set_actual_beat(beat);
                            if beat == 1 {
                                async_std::task::spawn(tick(audio_device_params, 800.0));
                            }
                            else {
                                async_std::task::spawn(tick(audio_device_params, 400.0));
                            }
                        }
                    },
                );
            }
        }

    });


    main.run().or(Err(Box::from("MainWindow")))
}

async fn tick(params: OutputDeviceParameters, frequency: f32) {
    let _device = run_output_device(params, {
        let mut clock = 0f32;
        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                clock = (clock + 1.0) % params.sample_rate as f32;
                let value =
                    (clock * frequency * 2.0 * std::f32::consts::PI / params.sample_rate as f32).sin();
                for sample in samples {
                    *sample = value;
                }
            }
        }
    }).unwrap();
    async_std::task::sleep(std::time::Duration::from_millis(100)).await;
}