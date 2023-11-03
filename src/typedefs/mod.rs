use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::Context;
use argh::FromArgs;
use gust_pak::common::GameVersion;
use tracing::{debug, error, info};
use typescript_type_def::{write_definition_file, DefinitionFileOptions, TypeDef};

use crate::utils::game_slug;
use crate::{ryza3, sophie};

/// Generate typescript definitions
#[derive(FromArgs)]
#[argh(subcommand, name = "typedefs")]
pub struct Args {
    /// the output folder
    #[argh(option, short = 'o')]
    output_path: Option<PathBuf>,

    /// the categories of typedefs to extract. can be a specific game (eg. `A17`), `all-games` or `common`
    #[argh(option, short = 'c')]
    pub category: Vec<Category>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Category {
    Game(GameVersion),
    AllGames,
    Common,
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        GameVersion::from_str(s)
            .map(Self::Game)
            .or_else(|_| match s {
                "all-games" => Ok(Self::AllGames),
                "common" => Ok(Self::Common),
                _ => Err(format!("Unknown category {}", s)),
            })
    }
}

impl Args {
    pub fn handle(self) -> anyhow::Result<()> {
        let output_folder = self
            .output_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("typedefs"));
        debug!(?output_folder);

        debug!("Generating typedefs");
        if self.category.is_empty() || self.category.contains(&Category::AllGames) {
            use gust_pak::common::strum::IntoEnumIterator;
            for game in GameVersion::iter() {
                _ = self
                    .write_typedefs_for_game(&output_folder, game)
                    .transpose()
                    .context("write all game typedefs")?;
            }
        } else {
            for category in &self.category {
                if let Category::Game(game) = category {
                    if self
                        .write_typedefs_for_game(&output_folder, *game)
                        .transpose()?
                        .is_none()
                    {
                        error!("Game has not been implemented yet: {game:?}");
                    }
                }
            }
        }

        if self.category.is_empty() || self.category.contains(&Category::Common) {
            gen_typedefs::<crate::utils::images::texture_atlas::UniformTextureAtlasInfo>(
                &output_folder,
                "texture_atlas.d.ts",
            )
            .context("generate texture atlas typedefs")?;
            gen_typedefs::<crate::ryza3::extract_images::MapInfoList>(
                &output_folder,
                "map_data.d.ts",
            )
            .context("generate map data typedefs")?;
        }

        info!("Wrote all typedefs to {:?}", output_folder);

        Ok(())
    }

    fn write_typedefs_for_game(
        &self,
        output_folder: &Path,
        game: GameVersion,
    ) -> Option<anyhow::Result<()>> {
        match game {
            GameVersion::A17 => Some(
                sophie::write_typedefs(&output_folder.join(game_slug(GameVersion::A17)))
                    .context("generate typedefs for sophie"),
            ),
            GameVersion::A18 => None,
            GameVersion::A19 => None,
            GameVersion::A21 => None,
            GameVersion::A22 => None,
            GameVersion::A23 => None,
            GameVersion::A24 => Some(
                ryza3::write_typedefs(&output_folder.join(game_slug(GameVersion::A24)))
                    .context("generate typedefs for ryza3"),
            ),
        }
    }
}

pub fn gen_typedefs<T>(output_folder: &Path, file_name: &str) -> anyhow::Result<()>
where
    T: TypeDef,
{
    let mut buf = Vec::new();
    write_definition_file::<_, T>(&mut buf, DefinitionFileOptions::default())
        .context("generate definition")?;
    let ts_module = String::from_utf8(buf).context("convert typedef to string")?;

    let output_file = output_folder.join(file_name);
    std::fs::create_dir_all(output_folder).context("create output folder")?;
    std::fs::write(output_file, ts_module).context("write output file")?;

    Ok(())
}
