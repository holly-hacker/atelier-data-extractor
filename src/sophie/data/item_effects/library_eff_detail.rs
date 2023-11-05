use crate::utils::{self, ElementReader, PakIndex};

pub struct LibraryEffDetail {
    pub note: Option<String>,
    pub permit: bool,
}

impl LibraryEffDetail {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\Saves\library\LibraryEffDetail.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "LibraryEffDetail");

        for element in elements {
            let reader = ElementReader(&element);

            let note = reader.read_opt("note")?;
            let permit = reader.read("permit")?;
            ret.push(Self { note, permit })
        }

        Ok(ret)
    }
}
