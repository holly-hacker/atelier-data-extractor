use std::path::Path;

use anyhow::Context;
use gust_pak::common::GameVersion;
use tracing::{debug, info};

use crate::utils::{game_slug, PakIndex};

mod doll_making;
mod item_effects;
mod items;
mod presents;
mod rumors;

pub struct SophieContext;

pub fn extract(mut pak_index: PakIndex, output_directory: &Path) -> anyhow::Result<()> {
    let output_directory = output_directory.join(game_slug(GameVersion::A17));

    debug!("Creating output directory");
    std::fs::create_dir_all(&output_directory).context("create output directory")?;

    info!("Writing files");
    write_to_files(&mut pak_index, &SophieContext, &output_directory)
        .context("write data to files")?;

    info!("Wrote sophie data to {:?}", output_directory);

    Ok(())
}

crate::generate_data_functions!(
    context SophieContext:
    Vec::<items::Item>,
    presents::PresentInfo,
    Vec::<rumors::Rumor>,
    Vec::<doll_making::Doll>,
    Vec::<item_effects::ItemEffect>,
);
