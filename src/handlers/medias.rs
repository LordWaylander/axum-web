use axum::{
    http::StatusCode,
    Json,
    extract::{Path, Multipart},
    BoxError,
    body::Bytes,
};
use std::env;
use std::io::{Error, ErrorKind};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use futures::{Stream, TryStreamExt};
use std::path::PathBuf;

use crate::repository::medias as RepositoryMedia;
use crate::models::medias::Media;
use crate::format_responses::ErrorResponse;

pub async fn get_all_upload() -> Result<Json<Vec<Media>>, ErrorResponse> {
    let result = RepositoryMedia::get_all_medias();

    match result {
        Ok(response) => {
            if response.len() == 0 {
                let err = ErrorResponse::error(StatusCode::OK.as_u16(),"No medias found".to_string());
                Err(err)
            } else {
                
                let mut resp_json = Vec::new();

                for r in response {
                    resp_json.push(r)
                }


                Ok(Json(resp_json))
            }
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(),e.to_string() );
            Err(err)
        },
    }

}

pub async fn get_one_upload(Path(id): Path<i32>) -> Result<Json<Media>, ErrorResponse> {
    let result = RepositoryMedia::get_one_media(id);

    match result {
        Ok(response) => {
            Ok(Json(response))
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(err)
        },
    }
}

pub async fn post_upload(file: Multipart) -> Result<Json<Media>, ErrorResponse> {
    let upload = upload(file).await;

    match upload {
        Ok((file_name, path)) => {
            let payload = Media {
                file_name,
                url: format!("{}/{}", env::var("ADDRESS").unwrap(), path.to_str().unwrap().to_string()),
                path: path.to_str().unwrap().to_string()
            };

            let result = RepositoryMedia::create_media(payload);
            
            match result {
                Ok(response) => {
                    Ok(Json(response))
                },
                Err(e) => {
                    let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
                    Err(err)
                },
            }
        }
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(err)
        }
    }
}

pub async fn upload(mut file: Multipart) -> Result<(std::string::String, PathBuf), Error> {
    // https://github.com/tokio-rs/axum/blob/main/examples/stream-to-file/src/main.rs
    while let Ok(Some(field)) = file.next_field().await {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        let path = std::path::Path::new(&env::var("UPLOAD_DIR").unwrap()).join(file_name.clone());
        let f = stream_to_file(path.clone(), field).await;

        match f {
            Ok(_) => {
                return Ok((file_name, path));
            }
            Err(e) => {
                return Err(e);
            }
        }
    
    }
    Err(Error::new(ErrorKind::Other, "Error when upload".to_string()))
}

async fn stream_to_file<S, E>(path: PathBuf, stream: S) -> Result<(), Error>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    // Convert the stream into an `AsyncRead`.
    let body_with_io_error = stream.map_err(|err| Error::new(ErrorKind::Other, err));
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);

    // Create the file. `File` implements `AsyncWrite`.
    let mut file = BufWriter::new(File::create(path).await?);

    // Copy the body into the file.
    tokio::io::copy(&mut body_reader, &mut file).await?;

    Ok::<_, Error>(())
}