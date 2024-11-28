use crate::models::medias::Media;
use crate::schema::medias;
use crate::schema::medias::dsl::*;
use crate::database;
use diesel::prelude::*;
use diesel::result::Error;

pub fn get_all_medias() -> Result<Vec<Media>, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Vec<Media>, Error> = connection.transaction(|connection| {
        let medias_vector = medias::table
        .order(medias::id.asc())
        .select(Media::as_select())
        .get_results::<Media>(connection)?;

        Ok(medias_vector)
    });

    return result;

}

pub fn get_one_media(other_id: i32) -> Result<Media, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Media, Error> = connection.transaction(|connection| {
        let media = medias::table
        .find(other_id)
        .select(Media::as_select())
        .get_result::<Media>(connection)?;

        Ok(media)
    });

    return result;
}