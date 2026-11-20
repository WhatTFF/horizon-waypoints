fn waypoints_hud(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    use crate::game::waypoints_mod as m;
    let (Some(p), Some((pos, yaw, pitch, fov))) = (&game.player, game.locator_camera) else {
        return;
    };
    let drawn = m::Camera { pos, yaw, pitch, fov };
    let view = m::Camera { pos: game.view.position, yaw: game.view.y_rot, pitch: game.view.x_rot, fov };
    let size = [batch.width, batch.height];
    let step = 1.0 / batch.scale;
    let measure = |t: &str| game.native_text_width(batch, t);
    let draws = m::with(|s| {
        let camera = s.label_camera(drawn, view);
        s.labels(camera, [p.position.x, p.position.y, p.position.z], game.dim.id(), size, step, &measure)
    });
    for draw in draws {
        match draw {
            m::Draw::Fill(rect, argb) => game.native_fill(batch, rect, argb),
            m::Draw::Text(text, at, rgb, shadow) => game.native_text(batch, &text, at, rgb, shadow),
        }
    }
}
