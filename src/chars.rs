fn waypoints_chars(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    c: char,
    modifiers: u32,
    canceled: &mut bool,
) {
    use crate::game::waypoints_mod as m;
    if game.mod_screen() != Some(m::ID) {
        return;
    }
    let _ = (screen, modifiers);
    *canceled = true;
    m::with(|s| s.char_typed(c));
}
