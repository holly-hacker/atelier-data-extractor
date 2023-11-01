use std::collections::BTreeMap;

use anyhow::Context;
use serde::Serialize;
use typescript_type_def::TypeDef;

use crate::utils::{ExtractableData, PakIndex};

#[derive(Serialize, TypeDef)]
pub struct ItemCategoryData {
    pub categories: BTreeMap<String, String>,
}

impl ExtractableData<super::Ryza3Context> for ItemCategoryData {
    const FILE_NAME: &'static str = "item_categories";

    fn read(_pak_index: &mut PakIndex, ctx: &super::Ryza3Context) -> anyhow::Result<Self> {
        let categories = ctx
            .executable_data
            .item_categories
            .iter()
            .enumerate()
            .map(|(i, category)| {
                let name = ctx
                    .strings_table
                    .id_lookup
                    .get(&format!("STR_ITEM_CATEGORY_{i:03}"))
                    .cloned()
                    .with_context(|| format!("cannot find string for category {category}"))?;

                Ok((category.clone(), name))
            })
            .collect::<anyhow::Result<_>>()
            .context("read categories from string table")?;

        Ok(Self { categories })
    }
}
