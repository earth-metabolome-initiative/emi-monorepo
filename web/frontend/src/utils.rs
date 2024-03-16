use web_sys::window;

/// Returns whether the user is currently online.
pub fn is_online() -> bool {
    window().map_or(false, |w| w.navigator().on_line())
}
