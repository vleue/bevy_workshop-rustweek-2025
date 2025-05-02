use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;

#[derive(Asset, TypePath)]
pub struct Level {
    pub width: u32,
    pub height: u32,
    pub asteroids: u32,
    pub lives: u32,
}

pub fn level_loader_plugin(app: &mut App) {
    app.init_asset::<Level>().init_asset_loader::<LevelLoader>();
}

#[derive(Default)]
struct LevelLoader;

#[derive(Debug, Error)]
enum LevelLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error in file format")]
    FormatError,
}

impl AssetLoader for LevelLoader {
    type Asset = Level;
    type Settings = ();
    type Error = LevelLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await?;

        let mut lines = buf.lines();
        Ok(Level {
            width: lines
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(LevelLoaderError::FormatError)?,
            height: lines
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(LevelLoaderError::FormatError)?,
            asteroids: lines
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(LevelLoaderError::FormatError)?,
            lives: lines
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(LevelLoaderError::FormatError)?,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["bw"]
    }
}
