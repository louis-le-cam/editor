pub fn setup_logger() {
    log_panics::init();

    #[cfg(not(debug_assertions))]
    let file_path = {
        let Ok(current_exe) = std::env::current_exe() else {
            return;
        };
        let Some(current_exe_dir) = current_exe.parent() else {
            return;
        };
        let Ok(file_path) = fern::log_file(current_exe_dir.join("log.txt")) else {
            return;
        };

        file_path
    };

    #[cfg(debug_assertions)]
    let Ok(file_path) = fern::log_file("log.txt") else {
        return;
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(file_path)
        .apply()
        .unwrap();
}
