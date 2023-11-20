use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use std::time::Duration;

#[derive(Debug, Clone, Resource)]
pub struct Rng(pub rand::rngs::SmallRng);

#[derive(Debug, Clone, Resource)]
pub struct GameTime(pub Duration);

#[derive(Debug, Clone, Resource)]
pub struct Killed(pub u32);

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/ZCOOLKuaiLe-Regular.ttf")]
    pub chs: Handle<Font>,
    #[asset(path = "fonts/JetBrainsMono-Italic.ttf")]
    pub eng: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/Hills-of-Radiant-Wind.ogg")]
    pub bgm: Handle<AudioSource>,
}
