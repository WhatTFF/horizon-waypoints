fn waypoints_mouse(
    game: &crate::game::Game<'_>,
    screen: &mod_api::ScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    use crate::game::waypoints_mod as m;
    if game.mod_screen() != Some(m::ID) {
        return;
    }
    *canceled = true;
    if button != 0 {
        return;
    }
    let here = game.player.as_ref().map(|p| m::Here {
        pos: [p.position.x, p.position.y, p.position.z],
        dimension: game.dim.id().to_owned(),
    });
    let size = [screen.rect[2], screen.rect[3]];
    let measure = |t: &str| horizon_client::hud::HudBatch::text_width(&game.gui.atlas, t);
    m::with(|s| {
        let (_, widgets) = s.screen(size, screen.mouse, here.as_ref(), &measure);
        if let Some(action) = m::hit(&widgets, screen.mouse) {
            s.act(action, here.as_ref());
        }
    });
}
