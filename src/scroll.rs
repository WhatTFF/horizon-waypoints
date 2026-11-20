fn waypoints_scroll(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    scroll: [f64; 2],
    canceled: &mut bool,
) {
    use crate::game::waypoints_mod as m;
    if game.mod_screen() != Some(m::ID) {
        return;
    }
    let _ = screen;
    *canceled = true;
    m::with(|s| s.scroll_by(scroll[1]));
}
