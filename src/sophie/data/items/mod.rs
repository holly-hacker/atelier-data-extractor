use anyhow::Context;
use serde::Serialize;
use tracing::debug;
use typescript_type_def::TypeDef;

use crate::utils::{ExtractableData, PakIndex};

mod item_data;
mod library_item;

#[derive(Serialize, TypeDef)]
pub struct Item {
    pub name: String,
    pub tag: String,
    pub image_no: usize,
    /// The description of this item as shown in the in-game library.
    pub library_note: Option<String>,
    /// Whether this item is shown in the in-game library.
    pub library_note_permit: bool,
    pub library_note_icons: [Option<u32>; 4],
    pub cost: i32,
    pub use_type: String,
    pub base: String,
    pub level: usize,
    pub shape_type: String,
    pub base_size: usize,
    pub quality_name: String,
    pub size_name: String,
    pub color: String,
    pub categories: Vec<String>,
    pub reasonable: Vec<usize>,

    pub strengthening: Option<usize>,
    pub hp: Option<usize>,
    pub mp: Option<usize>,
    pub lp: Option<usize>,
    pub atk: Option<usize>,
    pub def: Option<usize>,
    pub spd: Option<usize>,
    pub damage_min: Option<usize>,
    pub damage_max: Option<usize>,

    pub doll_tendency_cute: Option<i32>,
    pub doll_tendency_wise: Option<i32>,
    pub doll_tendency_brave: Option<i32>,
    pub doll_tendency_fool: Option<i32>,

    /// Player character, relates to which player cannot use this item.
    pub player_characters: Vec<usize>,
}

impl ExtractableData<super::SophieContext> for Vec<Item> {
    const FILE_NAME: &'static str = "items";

    fn read(pak_index: &mut PakIndex, _ctx: &super::SophieContext) -> anyhow::Result<Self> {
        debug!("Reading item data");
        let item_data = item_data::ItemData::read(pak_index).context("read item data")?;
        let library_item =
            library_item::LibraryItem::read(pak_index).context("read item library data")?;

        let items = item_data
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                Ok(Item {
                    name: item.name,
                    tag: item.tag,
                    image_no: item.image_no,
                    library_note: library_item[i].note.clone(),
                    library_note_permit: library_item[i].permit,
                    library_note_icons: [
                        match library_item[i].icon0 {
                            -1 => None,
                            x => Some(x as u32),
                        },
                        match library_item[i].icon1 {
                            -1 => None,
                            x => Some(x as u32),
                        },
                        match library_item[i].icon2 {
                            -1 => None,
                            x => Some(x as u32),
                        },
                        match library_item[i].icon3 {
                            -1 => None,
                            x => Some(x as u32),
                        },
                    ],
                    cost: item.cost,
                    use_type: item.use_type,
                    base: item.base,
                    level: item.level,
                    shape_type: item.shape_type,
                    base_size: item.base_size,
                    quality_name: item.quality_name,
                    size_name: item.size_name,
                    color: item.color,
                    categories: item.category,
                    reasonable: item.reasonable,
                    strengthening: item.strengthening,
                    hp: item.hp,
                    mp: item.mp,
                    lp: item.lp,
                    atk: item.atk,
                    def: item.def,
                    spd: item.spd,
                    damage_min: item.damage_min,
                    damage_max: item.damage_max,
                    doll_tendency_cute: item.doll_tendency_cute,
                    doll_tendency_wise: item.doll_tendency_wise,
                    doll_tendency_brave: item.doll_tendency_brave,
                    doll_tendency_fool: item.doll_tendency_fool,
                    player_characters: item
                        .pc
                        .into_iter()
                        .enumerate()
                        .filter_map(|(idx, val)| val.map(|_| idx))
                        .collect(),
                })
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(items)
    }
}
