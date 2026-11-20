fn waypoints_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::waypoints_mod as m;
    let key = if game.player.is_none() {
        None
    } else if let (None, Some(remote)) = (&game.integrated, &game.remote) {
        Some(m::server_key(&remote.session.address))
    } else if let Some(folder) = game
        .world_icon
        .as_ref()
        .and_then(|icon| icon.path.parent()?.file_name())
    {
        Some(m::world_key(&folder.to_string_lossy()))
    } else {
        let current = m::with(|s| s.world.clone()).filter(|w| w.starts_with("world-"));
        let named = game.integrated.as_ref().map(|i| m::world_key(&i.world_name));
        current.or(named).or_else(|| Some("world".into()))
    };
    let dir = game.native_data_dir(m::ID);
    let (no_labels, no_deaths) = m::with(|s| (s.labels.is_none(), s.deaths.is_none()));
    let stored = no_labels.then(|| game.stored_setting(m::ID, "labels")).flatten();
    let deaths = no_deaths.then(|| game.stored_setting(m::ID, "deaths")).flatten();
    let dead = game.screen == crate::game::Screen::Dead;
    let here = game.player.as_ref().map(|p| m::Here {
        pos: [p.position.x, p.position.y, p.position.z],
        dimension: game.dim.id().to_owned(),
    });
    let known = game.level.last_death.as_ref();
    let close = m::with(|s| {
        s.load_world(key, dir);
        s.labels.get_or_insert_with(|| m::read_on(stored));
        let deaths = *s.deaths.get_or_insert_with(|| m::read_on(deaths));
        s.track_death(deaths, dead, known, here.as_ref());
        s.take_close()
    });
    if close {
        game.mod_screen_close(m::ID);
    }
    let ours = game.mod_screen() == Some(m::ID);
    m::with(|s| s.sync_open(ours));
}
