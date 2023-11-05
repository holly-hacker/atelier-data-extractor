use crate::utils::{self, ElementReader, PakIndex};

pub struct LibraryItem {
    pub note: Option<String>,
    pub icon0: i32,
    pub icon1: i32,
    pub icon2: i32,
    pub icon3: i32,
    pub permit: bool,
}

impl LibraryItem {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\Saves\library\LibraryItem.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "LibraryItem");

        for element in elements {
            let reader = ElementReader(&element);

            let note = reader.read_opt("note")?;
            let icon0 = reader.read("icon0")?;
            let icon1 = reader.read("icon1")?;
            let icon2 = reader.read("icon2")?;
            let icon3 = reader.read("icon3")?;
            let permit = reader.read("permit")?;
            ret.push(Self {
                note,
                icon0,
                icon1,
                icon2,
                icon3,
                permit,
            })
        }

        Ok(ret)
    }
}
