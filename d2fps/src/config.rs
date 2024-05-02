use crate::{
  features::{AtomicFeatures, Features},
  util::{AtomicRatio, Ratio},
  GAME_FPS,
};
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use core::num::NonZeroU32;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ConfigFramerate {
  pub foreground: u32,
  pub background: u32,
}

#[derive(Serialize, Deserialize)]
struct ConfigFeatures {
  pub menu_fps: bool,
  pub game_fps: bool,
  pub motion_smoothing: bool,
  pub weather_smoothing: bool,
  pub arcane_bg: bool,
  pub anim_rate: bool,
}

#[derive(Serialize, Deserialize)]
struct ConfigCompatibility {
  pub reapply_patches: bool,
  pub integrity_checks: bool,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
  pub framerate: ConfigFramerate,
  pub features: ConfigFeatures,
  pub compatibility: ConfigCompatibility
}

pub(crate) struct Config {
  pub fps: AtomicRatio,
  pub bg_fps: AtomicRatio,
  pub features: AtomicFeatures,
  pub reapply_patches: AtomicBool,
  pub integrity_checks: AtomicBool,
}

impl Config {
  pub const fn new() -> Self {
    Self {
      fps: AtomicRatio::ZERO,
      bg_fps: AtomicRatio::new(GAME_FPS),
      features: AtomicFeatures::ALL,
      reapply_patches: AtomicBool::new(true),
      integrity_checks: AtomicBool::new(true),
    }
  }

  pub fn load_config(&self) {
    if let Ok(file) = fs::read_to_string("d2fps.json") {

      let json : ConfigFile = serde_json::from_str(file.as_str()).expect("D2FPS.json was not well-formatted!");
      self.fps.store_relaxed(Ratio::new(json.framerate.foreground, unsafe { NonZeroU32::new_unchecked(1) }));
      self.bg_fps.store_relaxed(Ratio::new(json.framerate.background, unsafe { NonZeroU32::new_unchecked(1) }));

      self.features.set_relaxed(Features::MenuFps, json.features.menu_fps);
      self.features.set_relaxed(Features::GameFps, json.features.game_fps);
      self.features.set_relaxed(Features::MotionSmoothing, json.features.motion_smoothing);
      self.features.set_relaxed(Features::ArcaneBg, json.features.arcane_bg);
      self.features.set_relaxed(Features::AnimRate, json.features.anim_rate);
      self.features.set_relaxed(Features::Weather, json.features.weather_smoothing);

      self.reapply_patches.store(json.compatibility.reapply_patches, Relaxed);
      self.integrity_checks.store(json.compatibility.integrity_checks, Relaxed);
    }

    log!(
      "Loaded config:\
      \n  fps: {}\
      \n  bg-fps: {}\
      \n  features: {}\
      \n  reapply-patches: {}\
      \n  integrity-checks: {}",
      self.fps.load_relaxed(),
      self.bg_fps.load_relaxed(),
      self.features.load_relaxed(),
      self.reapply_patches.load(Relaxed),
      self.integrity_checks.load(Relaxed),
    );
  }

  pub fn save_config(&self) {
    let fts = self.features.load_relaxed();
    let conf = ConfigFile {
      framerate: ConfigFramerate {
        foreground: self.fps.load_relaxed().num,
        background: self.bg_fps.load_relaxed().num,
      },
      features: ConfigFeatures {
        menu_fps: fts.intersects(Features::MenuFps),
        game_fps: fts.intersects(Features::GameFps),
        motion_smoothing: fts.intersects(Features::MotionSmoothing),
        weather_smoothing: fts.intersects(Features::Weather),
        arcane_bg: fts.intersects(Features::ArcaneBg),
        anim_rate: fts.intersects(Features::AnimRate)
      },
      compatibility: ConfigCompatibility {
        reapply_patches: self.reapply_patches.load(Relaxed),
        integrity_checks: self.integrity_checks.load(Relaxed)
      }
    };

    let json = serde_json::to_string_pretty(&conf).expect("Unable to serialize settings");
    fs::write("d2fps.json", json).expect("Unable to write d2fps.json");
  }
}
