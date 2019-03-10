// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use glyph_brush_layout::FontId;
pub use rusttype::Error as CreateFontError;

use nova_assets::{AssetId, ReadAssets};
use nova_core::ecs;
use nova_core::engine::{Engine, Resources};
use nova_core::quick_error;
use std::collections::HashMap;
use std::fs::File;
use std::io;

pub type Font = rusttype::Font<'static>;

pub type ReadFonts<'a> = ecs::ReadResource<'a, Fonts>;
pub type WriteFonts<'a> = ecs::WriteResource<'a, Fonts>;

#[derive(Debug, Default)]
pub struct Fonts {
  asset_table: HashMap<AssetId, FontId>,
  list: Vec<Font>,
}

impl Fonts {
  pub fn get(&self, id: FontId) -> &Font {
    &self.list[id.0]
  }

  pub fn create(
    &mut self,
    bytes: &'static [u8],
  ) -> Result<FontId, CreateFontError> {
    self.list.push(Font::from_bytes(bytes)?);

    Ok(FontId(self.list.len() - 1))
  }

  pub fn load_asset(
    &mut self,
    asset_id: AssetId,
    assets: &ReadAssets,
  ) -> Result<FontId, LoadFontAssetError> {
    use std::io::Read as _;

    if let Some(id) = self.asset_table.get(&asset_id) {
      return Ok(*id);
    }

    let asset = assets
      .get(asset_id)
      .ok_or_else(|| LoadFontAssetError::AssetNotFound(asset_id))?;

    let mut file = File::open(asset.fs_path())?;
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)?;

    self.list.push(Font::from_bytes(bytes)?);

    Ok(FontId(self.list.len() - 1))
  }
}

impl glyph_brush_layout::FontMap<'static> for Fonts {
  fn font(&self, i: glyph_brush_layout::FontId) -> &Font {
    &self.list[i.0]
  }
}

pub fn setup(engine: &mut Engine) {
  engine
    .resources_mut()
    .entry()
    .or_insert_with(Fonts::default);
}

pub fn read(res: &Resources) -> ReadFonts {
  ecs::SystemData::fetch(res)
}

pub fn write(res: &Resources) -> WriteFonts {
  ecs::SystemData::fetch(res)
}

quick_error! {
  #[derive(Debug)]
  pub enum LoadFontAssetError {
    AssetNotFound(asset_id: AssetId) {
      from()
      display("asset {:?} not found", asset_id)
    }
    Io(err: io::Error) {
      from()
      display("could not read font data: {}", err)
    }
    CreateFont(err: CreateFontError) {
      from()
      display("could not create font: {}", err)
    }
  }
}