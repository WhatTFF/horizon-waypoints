use mod_api::{self, Hooks, KeyRegistration, Registrar, RegistrationContext, SettingSpec};

#[derive(Default)]
struct Mod;

pub const KEY_CREATE: &str = "create";
pub const KEY_LIST: &str = "list";
pub const DEFAULT_CREATE: &str = "key.keyboard.n";
pub const DEFAULT_LIST: &str = "key.keyboard.m";

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, cx: &RegistrationContext) {
        let _ = cx;
        r.register_key_mapping(
            KeyRegistration::new(KEY_CREATE, DEFAULT_CREATE).label("Create Waypoint"),
        );
        r.register_key_mapping(
            KeyRegistration::new(KEY_LIST, DEFAULT_LIST).label("List Waypoints"),
        );
        let _ = r.register_setting(SettingSpec::choice(
            "labels",
            "Waypoint Labels in World",
            vec!["Off".to_string(), "On".to_string()],
            1,
        ));
        let _ = r.register_setting(SettingSpec::choice(
            "deaths",
            "Death Waypoints",
            vec!["Off".to_string(), "On".to_string()],
            1,
        ));
    }
}
