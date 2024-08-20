use anyhow::Result;
use reqwest::multipart::{self, Form};
use std::path::Path;

#[async_trait::async_trait]
pub(crate) trait ToMultipart {
    async fn to_multipart(self) -> Result<Form>;
}

pub async fn file_to_multipart<P: AsRef<Path>>(key: String, path: P, form: Form) -> Result<Form> {
    // 使用tokio读取bytes然后转换为reqwest::multipart::Form
    let bytes = tokio::fs::read(path.as_ref()).await?;
    let file_name = path.as_ref().file_name().and_then(|f| f.to_str());
    let file_mime_str = mime_guess::from_path(path.as_ref())
        .first()
        .and_then(|m| Some(m.to_string()));
    let mut file_part = multipart::Part::bytes(bytes);
    if let Some(file_name) = file_name {
        file_part = file_part.file_name(file_name.to_string());
    }
    if let Some(file_mime_str) = file_mime_str {
        file_part = file_part.mime_str(&file_mime_str)?;
    }
    let form = form.part(key, file_part);
    Ok(form)
}
