
use slint::platform::SetPlatformError;
use slint::Weak;

slint::include_modules!();
#[unsafe(no_mangle)]
pub fn android_main(app: slint::android::AndroidApp) -> Result<(),  Box<dyn std::error::Error>> {
    slint::android::init(app)?;


    let main = MainWindow::new()?;
    let main_weak = main.as_weak();

    // foo(main_weak)?;

    main.run().or(Err(Box::from("MainWindow")))
}

// fn foo(weak: Weak<MainWindow>) -> Result<(), Box<dyn std::error::Error>> {
//     let main_window = weak.upgrade().ok_or("error while handling window")?;
//     main_window.global::<Logic>().set_numerator(10);
//     Ok(())
// }

