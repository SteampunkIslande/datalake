use rocket::{FromForm, State, form::Form, fs::TempFile, post};

use crate::DatalakeConfig;

#[derive(FromForm)]
pub struct Upload<'f> {
    pub file: TempFile<'f>,
    pub file_name: String,
    pub description: Option<String>,
}

#[post("/upload", format = "multipart/form-data", data = "<file>")]
pub async fn import_file_post(
    file: Form<Upload<'_>>,
    config: &State<DatalakeConfig>,
) -> std::io::Result<()> {
    let Upload {
        mut file,
        file_name,
        description,
    } = file.into_inner();

    let destination_file = config.datalake_path.join("user-files").join(&file_name);

    file.move_copy_to(destination_file).await?;

    Ok(())
}
