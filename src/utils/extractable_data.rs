use serde::Serialize;
use typescript_type_def::TypeDef;

use super::PakIndex;

pub trait ExtractableData<TCtx>: TypeDef + Serialize + Sized {
    const FILE_NAME: &'static str;

    fn read(pak_index: &mut PakIndex, ctx: &TCtx) -> anyhow::Result<Self>;
}

#[macro_export]
macro_rules! generate_data_functions {
    (context $context:ty: $($type:ty,)+) => {
        pub fn write_to_files(
            pak_index: &mut $crate::utils::PakIndex,
            ctx: &$context,
            output_directory: &std::path::Path,
        ) -> anyhow::Result<()> {
            use $crate::utils::ExtractableData;
            $(
                info!("Writing {} data", stringify!($type));
                let data = <$type>::read(pak_index, ctx).context(concat!("read ", stringify!($type)))?;
                $crate::extract::write_data_to_file(
                    &output_directory.join(format!("{}.json", <$type>::FILE_NAME)),
                    &data,
                )
                .context(concat!("write ", stringify!($type)))?;
            )+
            Ok(())
        }

        pub fn write_typedefs(output_directory: &std::path::Path) -> anyhow::Result<()> {
            use $crate::utils::ExtractableData;
            $(
                info!("Writing typedefs for {}", stringify!($type));
                $crate::typedefs::gen_typedefs::<$type>(
                    output_directory,
                    &format!("{}.d.ts", <$type>::FILE_NAME),
                )
                .context(concat!("write typedefs for ", stringify!($type)))?;
            )+
            Ok(())
        }
    };
}
