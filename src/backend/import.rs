use std::path::Path;

use rocket::{FromForm, form::Form, fs::TempFile, post};

#[derive(FromForm)]
pub struct Upload<'f> {
    file: TempFile<'f>,
    file_name: String,
}

#[post("/upload", format = "multipart/form-data", data = "<file>")]
pub async fn import_file_post(mut file: Form<Upload<'_>>) -> std::io::Result<()> {
    let file_name = &file.file_name;
    file.file
        .move_copy_to(Path::new("user-files").join(file_name));
    Ok(())
}
