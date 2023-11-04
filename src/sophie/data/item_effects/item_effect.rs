use crate::utils::{self, ElementReader, PakIndex};

pub struct ItemEffect {
    pub item_effect_name: String,
    pub item_effect_tag: String,
    pub hit_se: Option<String>,
    pub g_tag: String,
    pub attribute_tag: String,
    pub attribute_tag_2: String,
    pub act_tag: String,
    pub act_tag_2: String,
    pub min: Vec<Option<String>>,
    pub max: Vec<Option<String>>,
}

impl ItemEffect {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\Saves\item\item_effect.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "item_effect");

        for element in elements {
            let reader = ElementReader(&element);

            let item_effect_name = reader.read("ItemEffectName")?;
            let item_effect_tag = reader.read("ItemEffTag")?;
            let hit_se = reader.read_opt("HitSE")?;
            let g_tag = reader.read("g_tag")?;
            let attribute_tag = reader.read("attributeTag")?;
            let attribute_tag_2 = reader.read("attributeTag2")?;
            let act_tag = reader.read("actTag")?;
            let act_tag_2 = reader.read("actTag2")?;
            let min = reader.read_sparse_list("min_*")?;
            let max = reader.read_sparse_list("max_*")?;

            ret.push(Self {
                item_effect_name,
                item_effect_tag,
                hit_se,
                g_tag,
                attribute_tag,
                attribute_tag_2,
                act_tag,
                act_tag_2,
                min,
                max,
            })
        }

        Ok(ret)
    }
}
