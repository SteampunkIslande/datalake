use rocket::{FromForm, form::Form, fs::TempFile, post};
use std::path::Path;

#[derive(FromForm)]
pub struct Upload<'f> {
    pub file: TempFile<'f>,
    pub file_name: String,
}

#[post("/upload", format = "multipart/form-data", data = "<file>")]
pub async fn import_file_post(file: Form<Upload<'_>>) -> std::io::Result<()> {
    let Upload {
        mut file,
        file_name,
    } = file.into_inner();

    file.move_copy_to(Path::new("user-files").join(&file_name))
        .await?;

    Ok(())
}
