fn waypoints_keys(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    key: &str,
    repeat: bool,
    modifiers: u32,
    canceled: &mut bool,
) {
    use crate::game::waypoints_mod as m;
    if game.mod_screen() != Some(m::ID) {
        return;
    }
    let _ = (screen, repeat);
    *canceled = true;
    let here = game.player.as_ref().map(|p| m::Here {
        pos: [p.position.x, p.position.y, p.position.z],
        dimension: game.dim.id().to_owned(),
    });
    let create_bound = game.mod_key(m::ID, m::KEY_CREATE)
        .map_or_else(|| m::DEFAULT_CREATE.to_string(), |(k, _)| k);
    let list_bound = game.mod_key(m::ID, m::KEY_LIST)
        .map_or_else(|| m::DEFAULT_LIST.to_string(), |(k, _)| k);
    let shift = modifiers & 1 != 0;
    m::with(|s| s.key(key, shift, here.as_ref(), &list_bound, &create_bound));
}
