use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;
use log::debug;
use slint::{ComponentHandle, Model, ModelRc, Timer, TimerMode, ToSharedString, VecModel, Weak};
use slint::private_unstable_api::debug;
use tinyaudio::{run_output_device, OutputDeviceParameters};
use crate::MainWindow;
use crate::MetronomeUnit;
use crate::app_state::AppState;
use crate::timer_unit::TimerUnit;

pub fn start_app() -> Result<(),  Box<dyn std::error::Error>>   {


    let main = MainWindow::new()?;
    let main_weak = main.as_weak();



    let timer = Rc::from(Timer::default());




    //intit model from save or scratch
    let model = Rc::from(
        VecModel::from(
            vec! [
                MetronomeUnit::new(4, 4, 120, 8),
            ]
        )
    );

    let app_state = Rc::from(AppState::new(model.clone()));
    main.set_metronome_model(ModelRc::from(model.clone()));





    main.on_play_button_pressed(move || on_play_button_pressed(main_weak.clone(), timer.clone(), app_state.clone(), model.clone()));


    main.run().or(Err(Box::from("MainWindow")))
}

async fn tick(accent: bool) {
    let params =  OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };
    let _device = run_output_device(params, {
        let mut clock = 0f32;
        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                clock = (clock + 1.0) % params.sample_rate as f32;
                let frequency = if accent { 800.0 } else { 400.0 }; //later from config
                let value =
                    (clock * frequency * 2.0 * std::f32::consts::PI / params.sample_rate as f32).sin();
                for sample in samples {
                    *sample = value;
                }
            }
        }
    }).unwrap();
    async_std::task::sleep(std::time::Duration::from_millis(10)).await;
}

//better name?
fn set_beat(main_window: Weak<MainWindow>, app_state: Rc<AppState>, index: i32, beat: i32 ) {
    debug!("setting");
    let handler = main_window.upgrade().unwrap();
    handler.set_actual_beat(beat);
    handler.set_index(index);
    _ = app_state.select_unit(index as usize);

}

fn on_play_button_pressed(main_weak: Weak<MainWindow>, timer: Rc<Timer>, app_state: Rc<AppState>, model: Rc<VecModel<MetronomeUnit>> ) {
    let handle = main_weak.clone();
    if let Some(main) = handle.upgrade() {

        if main.get_playing() {
            timer.stop();
        }
        else {
            // if model.iter().count() == 0 {
            //     return;
            // }

            let app_state = app_state.clone();
            let mut vec = VecDeque::new();

            let start = app_state.selected_index().unwrap_or_else(|| 0);

            for i in start..app_state.model().row_count() {
                if let Some(munit) = model.row_data(i) {
                    let duration = std::time::Duration::from_millis(calc_duration(munit.tempo, munit.denominator).try_into().unwrap());

                    vec.extend((0..munit.count)
                        .map(|metronome_unit_index| {
                            return TimerUnit::new(duration, false, i, metronome_unit_index.clone() as usize)
                        }));
                }
            }

            process(timer, app_state, main_weak, vec);

        }
    }
}

fn process(timer: Rc<Timer>, app_state: Rc<AppState>, main_window: Weak<MainWindow>, mut data:VecDeque<TimerUnit>) {
    debug!("start");
    timer.clone().start(
        TimerMode::Repeated,
        Duration::default(),
        move || {
            debug!("{}", data.back().unwrap().beat());
            match data.pop_back() {
                Some(value) => {
                    timer.set_interval(value.duration());
                    async_std::task::spawn(tick( value.accent()));
                    _ = set_beat(main_window.clone(), app_state.clone(), value.index() as i32, value.beat() as i32 );
                }
                None => timer.stop()
            }
        },
    );
}


async fn move_beat(main_window: Weak<MainWindow>, numerator: i32) {
    let handler = main_window.upgrade().unwrap();
    let mut beat = handler.get_actual_beat();
    beat = ((beat) % numerator) +1;
    handler.set_actual_beat(beat);
}

fn tick_unit(app_state: Rc<AppState>) {

}
fn calc_duration(bpm: i32, denominator: i32) -> usize{
    if bpm == 0 || denominator == 0 {
        return 0;
    }
    60_000 * 4 / (bpm as u64 * denominator as u64) as usize
}