use anyhow::Context;
use serde::Serialize;
use typescript_type_def::TypeDef;

use crate::utils::{ExtractableData, PakIndex};

mod normal;

#[derive(Serialize, TypeDef)]
pub struct QuestData {
    pub normal_quests: Vec<normal::NormalQuest>,
}

impl ExtractableData<super::Ryza3Context> for QuestData {
    const FILE_NAME: &'static str = "quests";

    fn read(pak_index: &mut PakIndex, ctx: &super::Ryza3Context) -> anyhow::Result<Self> {
        let normal_quests = normal::NormalQuest::read(pak_index, &ctx.strings_table)
            .context("read normal quests")?;

        Ok(Self { normal_quests })
    }
}
