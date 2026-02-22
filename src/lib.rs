use std::cell::RefMut;
use std::rc::Rc;
use std::time::Duration;
use slint::{run_event_loop, Timer, TimerMode, Weak};
use log::{log, Log};

slint::include_modules!();
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: slint::android::AndroidApp) -> Result<(),  Box<dyn std::error::Error>> {
    slint::android::init(app)?;


    let main = MainWindow::new()?;
    let main_weak = main.as_weak();
    let logic = main.global::<Logic>();
    let timer = Rc::from(Timer::default());


    logic.on_play_button_pressed(move || {
        let handle = main_weak.clone();
        if let Some(main) = handle.upgrade() {
            let logic = main.global::<Logic>();
            if logic.get_playing() {
                timer.stop();
            }
            else {
                timer.start(
                    TimerMode::Repeated,
                    Duration::from_secs(1),
                    move || {
                        if let Some(ui) = handle.upgrade() {
                            let state = ui.global::<Logic>();
                            let value = state.get_numerator();
                            state.set_numerator(value + 1);
                        }
                    },
                );
            }
        }

    });


    main.run().or(Err(Box::from("MainWindow")))
}

// fn foo(weak: Weak<MainWindow>) -> Result<(), Box<dyn std::error::Error>> {
//     let main_window = weak.upgrade().ok_or("error while handling window")?;
//     main_window.global::<Logic>().set_numerator(10);
//     Ok(())
// }

