fn waypoints_screen(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    use crate::game::waypoints_mod as m;
    if game.mod_screen() != Some(m::ID) {
        return;
    }
    let here = game.player.as_ref().map(|p| m::Here {
        pos: [p.position.x, p.position.y, p.position.z],
        dimension: game.dim.id().to_owned(),
    });
    let size = [batch.width, batch.height];
    let measure = |t: &str| game.native_text_width(batch, t);
    let cursor = [game.cursor.0, game.cursor.1];
    let (draws, _) = m::with(|s| s.screen(size, cursor, here.as_ref(), &measure));
    for draw in draws {
        match draw {
            m::Draw::Fill(rect, argb) => game.native_fill(batch, rect, argb),
            m::Draw::Text(text, at, rgb, shadow) => game.native_text(batch, &text, at, rgb, shadow),
        }
    }
}
