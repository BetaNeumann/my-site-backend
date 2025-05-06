pub trait Record {
    const TABLE_NAME: &'static str;
}

#[macro_export]
macro_rules! record {
    ($table_name:literal, $name:ident { $($field:ident: $type:ty),* }) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            $(pub $field: $type),*
        }

        impl crate::traits::Record for $name {
            const TABLE_NAME: &'static str = $table_name;
        }

        paste::paste! {
            #[derive(Debug, serde::Serialize, serde::Deserialize)]
            pub struct [< $name WithId >] {
                pub id: surrealdb::RecordId,
                $(pub $field: $type),*
            }

            impl crate::traits::Record for [< $name WithId >] {
                const TABLE_NAME: &'static str = $table_name;
            }
        }
    };
}
