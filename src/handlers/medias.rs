use axum::{
    http::StatusCode,
    Json,
    extract::{Path, Multipart},
    BoxError,
    body::Bytes,
};
use std::env;
use std::io;
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

pub async fn upload(mut file: Multipart) -> Result<String, ErrorResponse> {
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
                println!("{:?}", file_name);
                println!("{:?}", path);

                return Ok("upload ok test".to_string());
            }
            Err(e) => {
                let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
                return Err(err);
            }
        }
    
    }
    let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), "Error when upload".to_string());
    return Err(err);
}

async fn stream_to_file<S, E>(path: PathBuf, stream: S) -> Result<(), String>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        //Ok::<_, io::Error>(())
        Ok(())
    }
    .await
    .map_err(|err: std::io::Error| err.to_string())
}