use std::{
    fmt::{self},
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

use bevy::{asset::AssetPath, gltf::GltfAssetLabel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameAssetPath {
    short: String,
    full: String,
    exists: bool,
}

const PATH_DATA: &'static str = "data";
const PATH_MODELS: &'static str = "models";
const PATH_TEXTURES: &'static str = "textures";
const PATH_SFX: &'static str = "sfx";
const CORE_DIR: &'static str = "core";

impl GameAssetPath {
    fn new(shortcode: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        let short: String = shortcode.into();
        let full: String = Into::<PathBuf>::into(path)
            .to_str()
            .unwrap_or_default()
            .to_string();
        let f = File::open(PathBuf::from("assets").join(full.clone()));
        Self {
            short,
            full,
            exists: f.is_ok(),
        }
    }

    fn parse_path(shortcode: String, infix_path: &str) -> PathBuf {
        let short: &String = (&shortcode).into();
        if short.contains("::") {
            let (root, path) = short.split_once("::").unwrap();

            PathBuf::new()
                .join(if root.is_empty() { CORE_DIR } else { root })
                .join(infix_path)
                .join(path)
        } else {
            PathBuf::from_str(shortcode.as_str()).unwrap_or_default()
        }
    }

    pub fn new_data<S>(shortcode: S) -> Self
    where
        S: Into<String> + Clone,
    {
        Self::new(
            shortcode.clone(),
            Self::parse_path(shortcode.into(), PATH_DATA),
        )
    }

    pub fn new_model<S>(shortcode: S) -> Self
    where
        S: Into<String> + Clone,
    {
        Self::new(
            shortcode.clone(),
            Self::parse_path(shortcode.into(), PATH_MODELS),
        )
    }

    pub fn new_texture<S>(shortcode: S) -> Self
    where
        S: Into<String> + Clone,
    {
        Self::new(
            shortcode.clone(),
            Self::parse_path(shortcode.into(), PATH_TEXTURES),
        )
    }

    pub fn new_sfx<S>(shortcode: S) -> Self
    where
        S: Into<String> + Clone,
    {
        Self::new(
            shortcode.clone(),
            Self::parse_path(shortcode.into(), PATH_SFX),
        )
    }

    /// Returns the path relative to the "root" execution directory. Expected
    /// that this directory contains the executable and the "assets" directory
    pub fn path_relative(&self) -> PathBuf {
        Path::new("assets").join(self.full.clone())
    }

    /// Returns the canonical path to the asset, which may fail depending on the
    /// conditions.
    pub fn path_canonical(&self) -> Option<PathBuf> {
        let Ok(c) = Path::new("assets").join(self.full.clone()).canonicalize() else {
            return None;
        };
        Some(c)
    }

    pub fn gltf_scene(self) -> AssetPath<'static> {
        GltfAssetLabel::Scene(0).from_asset(self)
    }

    pub fn exists(&self) -> bool {
        self.exists
    }
}

impl fmt::Display for GameAssetPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[{}]({})", self.short, self.full))
    }
}

impl<'a> From<GameAssetPath> for AssetPath<'a> {
    fn from(value: GameAssetPath) -> Self {
        value.full.into()
    }
}
