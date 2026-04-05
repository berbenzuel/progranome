


pub mod metronome_unit;
pub mod app_state;
pub mod app;


slint::include_modules!();



#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: slint::android::AndroidApp) -> Result<(),  Box<dyn std::error::Error>> {
    slint::android::init(app)?;
    app::start_app()
}




