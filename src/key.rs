fn waypoints_key(
    game: &mut crate::game::Game<'_>,
    key: &str,
    action: mod_api::KeyAction,
    modifiers: u32,
) {
    use crate::game::waypoints_mod as m;
    if action != mod_api::KeyAction::Press || !game.native_window() || !game.playing() {
        return;
    }
    let create_bound = game.mod_key(m::ID, m::KEY_CREATE)
        .map_or_else(|| (m::DEFAULT_CREATE.to_string(), 0), |(k, mods)| (k, mods.bits));
    let list_bound = game.mod_key(m::ID, m::KEY_LIST)
        .map_or_else(|| (m::DEFAULT_LIST.to_string(), 0), |(k, mods)| (k, mods.bits));

    let is_create = key == create_bound.0 && (modifiers & 7 == create_bound.1 & 7);
    let is_list = key == list_bound.0 && (modifiers & 7 == list_bound.1 & 7);

    if !is_create && !is_list {
        return;
    }

    if game.mod_screen() == Some(m::ID) {
        game.mod_screen_close(m::ID);
        return;
    }

    let Some(p) = game.player.as_ref() else {
        return;
    };
    let here = m::Here {
        pos: [p.position.x, p.position.y, p.position.z],
        dimension: game.dim.id().to_owned(),
    };
    m::with(|s| {
        if is_create {
            s.open_create(&here, false, m::key_char(key));
        } else {
            s.open_list(m::key_char(key));
        }
    });
    if !game.mod_screen_open(m::ID) {
        m::with(|s| s.reset());
    }
}
