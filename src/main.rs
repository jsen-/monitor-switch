mod config;
mod state;

use config::Config;
use pulsectl::controllers::SinkController;
use state::State;
use std::{
    env::var_os,
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let home = var_os("HOME");
    let home = home.as_ref().map(Path::new).expect("unable to determine home dir");
    let config_path = match var_os("XDG_CONFIG_HOME") {
        Some(p) => PathBuf::from(p),
        None => home.join(".config"),
    }
    .join(env!("CARGO_PKG_NAME"))
    .join("config.toml");
    let monitors_path = match var_os("XDG_CONFIG_HOME") {
        Some(p) => PathBuf::from(p),
        None => home.join(".config"),
    }
    .join("monitors.xml");

    let state_dir = match var_os("XDG_CACHE_HOME") {
        Some(p) => PathBuf::from(p),
        None => home.join(".cache"),
    }
    .join(env!("CARGO_PKG_NAME"));
    let state_path = state_dir.join("state");
    let state_tmp_path = state_dir.join("state.tmp");

    let config = Config::from_file(config_path).unwrap();
    if config.profiles.is_empty() {
        panic!("no profiles in config");
    }
    let mut it = config.profiles.iter();

    let mut state = State::from_file(&state_path).unwrap_or_default();
    let (profile_name, profile) = it
        .find(|(name, _)| *name == &state.profile)
        .and_then(|_| it.next())
        .unwrap_or_else(|| config.profiles.first().unwrap());
    state.profile = profile_name.clone();
    state.to_file(&state_tmp_path).unwrap();

    let dbus = zbus::blocking::Connection::session().unwrap();
    let mut handler = SinkController::create().unwrap();
    use pulsectl::controllers::DeviceControl;
    if handler
        .list_devices()
        .expect("Could not get list of playback devices.")
        .into_iter()
        .filter_map(|device| device.name)
        .find(|device_name| device_name == &profile.pulseaudio_sink)
        .is_none()
    {
        panic!(r#"pulse audio sink "{}" not found"#, profile.pulseaudio_sink);
    }
    if !handler.set_default_device(&profile.pulseaudio_sink).unwrap() {
        panic!(r#"failed to set pulse audio default sink to "{}""#, profile.pulseaudio_sink);
    }
    fs::write(monitors_path, &profile.monitors).unwrap();
    dbus.call_method(
        Some("org.gnome.Shell"),
        "/org/gnome/Shell",
        Some("org.gnome.Shell"),
        "Eval",
        &("Meta.restart(\"Restarting gnome shell…\")"),
    )
    .unwrap();

    fs::rename(state_tmp_path, state_path).unwrap();
}
