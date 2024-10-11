use bevy::log::error;

/// Handles assertion type flows
///
/// - In development environments (config `debug_assertions`) panics with message
/// - In release builds an error message is printed to the log
///
/// Arguably it would be easier if this was a macro, but then I'd have to learn macros
pub fn assert_dev(message: &str) {
    if cfg!(debug_assertions) {
        panic!("Devtime assertion found false: {}", message);
    } else {
        error!("Devtime assertion (non-breaking): {}", message);
    }
}
