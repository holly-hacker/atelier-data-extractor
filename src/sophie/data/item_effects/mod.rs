mod item_effect;

use anyhow::Context;

use crate::utils::ExtractableData;

use super::SophieContext;

#[derive(serde::Serialize, typescript_type_def::TypeDef)]
pub struct ItemEffect {
    pub name: String,
    pub tag: String,
    /// Hit sound effect
    pub hit_se: Option<String>,
    pub group_tag: String,
    pub actions: [EffectAction; 2],
}

#[derive(serde::Serialize, typescript_type_def::TypeDef)]
pub struct EffectAction {
    /// The tag, or `ACT_NONE` for none.
    pub act_tag: String,
    /// The damage attribute, or `ATT_NONE` for none.
    pub attribute_tag: String,
    /// Minimum parameters for this effect.
    pub min: [Option<String>; 2],
    /// Maximum parameters for this effect.
    pub max: [Option<String>; 2],
}

impl ExtractableData<SophieContext> for Vec<ItemEffect> {
    const FILE_NAME: &'static str = "item_effects";

    fn read(pak_index: &mut crate::utils::PakIndex, _ctx: &SophieContext) -> anyhow::Result<Self> {
        let item_effects =
            item_effect::ItemEffect::read(pak_index).context("read item effect data")?;

        let mapped = item_effects
            .into_iter()
            .map(|e| ItemEffect {
                name: e.item_effect_name,
                tag: e.item_effect_tag,
                hit_se: e.hit_se,
                group_tag: e.g_tag,
                actions: [
                    EffectAction {
                        act_tag: e.act_tag,
                        attribute_tag: e.attribute_tag,
                        min: [
                            e.min.get(1).cloned().flatten(),
                            e.min.get(2).cloned().flatten(),
                        ],
                        max: [
                            e.max.get(1).cloned().flatten(),
                            e.max.get(2).cloned().flatten(),
                        ],
                    },
                    EffectAction {
                        act_tag: e.act_tag_2,
                        attribute_tag: e.attribute_tag_2,
                        min: [
                            e.min.get(3).cloned().flatten(),
                            e.min.get(4).cloned().flatten(),
                        ],
                        max: [
                            e.max.get(3).cloned().flatten(),
                            e.max.get(4).cloned().flatten(),
                        ],
                    },
                ],
            })
            .collect();

        Ok(mapped)
    }
}
