use crate::models::medias::{Media, NewMedia, UpdateMedia};
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

pub fn create_media(payload: NewMedia) -> Result<Media, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Media, Error> = connection.transaction(|connection| {
        diesel::insert_into(medias::table)
        .values(&payload)
        .execute(connection)?;

        let media = medias::table
        .order(medias::id.desc())
        .select(Media::as_select())
        .get_result(connection)?;

        Ok(media)
    }); 

    return result;
}

pub fn update_media(payload: UpdateMedia) -> Result<Media, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Media, Error> = connection.transaction(|connection| {

        diesel::update(medias::table.find(payload.id))
            .set(&payload)
            .execute(connection)?;

            let media = medias::table
            .find(payload.id)
            .select(Media::as_select())
            .get_result(connection)?;

        Ok(media)
    });

    return result;
}

pub fn delete_media(other_id: i32) -> Result<Media, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Media, Error> = connection.transaction(|connection| {

        let media = medias::table
        .find(&other_id)
        .select(Media::as_select())
        .get_result(connection)?;

        diesel::delete(medias::table)
        .filter(id.eq(&other_id))
        .execute(connection)?;
        
        Ok(media)
    });

    return result;
}