/// Check if the point (x, y) is inside the given rect.
/// If it is, return the coordinates relative to the rect's top-left corner.
/// # Arguments
///
/// * `x` - The x coordinate of the point.
/// * `y` - The y coordinate of the point.
/// * `rect` - The rect to check.
///
/// # Returns
///
/// Returns `Some((x - rect.x, y - rect.y))` if the point is inside the rect, otherwise `None`.
pub fn is_contains_rect(x: u16, y: u16, rect: &ratatui_core::layout::Rect) -> Option<(u16, u16)> {
    if x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height {
        Some((x - rect.x, y - rect.y))
    } else {
        None
    }
}
