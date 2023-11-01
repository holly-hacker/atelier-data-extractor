use std::path::Path;

use anyhow::Context;
use gust_pak::common::GameVersion;
use tracing::{debug, info};

use self::strings_table::StringsTable;
use super::executable::Ryza3ExecutableData;
use crate::utils::{game_slug, PakIndex};

mod enemies;
mod feeding;
mod field_data;
mod field_map;
mod item_categories;
mod item_effects;
mod items;
mod quests;
mod recipes;
mod strings_table;

pub struct Ryza3Context {
    pub executable_data: Ryza3ExecutableData,
    pub strings_table: StringsTable,
}

pub fn extract(
    game_directory: &Path,
    mut pak_index: PakIndex,
    output_directory: &Path,
) -> anyhow::Result<()> {
    let output_directory = output_directory.join(game_slug(GameVersion::A24));

    debug!("reading executable data");
    let executable_data =
        Ryza3ExecutableData::read_all(game_directory).context("read executable data")?;

    debug!("Creating output directory");
    std::fs::create_dir_all(&output_directory).context("create output directory")?;

    info!("Writing files");

    let ctx = Ryza3Context {
        executable_data,
        strings_table: StringsTable::read(&mut pak_index).context("read strings table")?,
    };

    write_to_files(&mut pak_index, &ctx, &output_directory).context("write data to files")?;

    info!("Wrote ryza3 data to {:?}", output_directory);

    Ok(())
}

crate::generate_data_functions!(
    context Ryza3Context:
    Vec::<items::Item>,
    item_categories::ItemCategoryData,
    item_effects::ItemEffectData,
    recipes::RecipeData,
    field_map::FieldMapData,
    field_data::FieldData,
    Vec::<enemies::Enemy>,
    feeding::PuniFeedingData,
    quests::QuestData,
);
