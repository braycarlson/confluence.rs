#[derive(Clone, Copy, Debug)]
pub struct ContentMetadataCurrentuserFavourited;

impl ContentMetadataCurrentuserFavourited {
    pub const FAVOURITED_DATE: crate::Field<Self, crate::Text> =
        crate::Field::new("favouritedDate");

    pub const IS_FAVOURITE: crate::Field<Self, crate::Boolean> = crate::Field::new("isFavourite");
}
