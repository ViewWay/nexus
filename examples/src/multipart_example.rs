// Multipart File Upload Example / 多部分文件上传示例
//
// Demonstrates Nexus's multipart form data handling:
// 演示 Nexus 的多部分表单数据处理：
// - Single file upload / 单文件上传
// - Multiple file upload / 多文件上传
// - Form fields with files / 带文件的表单字段
// - File validation / 文件验证
//
// Equivalent to: Spring MultipartFile, Multer
// 等价于：Spring MultipartFile, Multer

use nexus_http::{Request, Response, Result, StatusCode};
use nexus_multipart::{Multipart, MultipartData};
use nexus_router::Router;
use serde::{Deserialize, Serialize};
use std::fs::File;
use bytes::Bytes;
use std::io::Write;
use std::path::Path;

/// File metadata / 文件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileMetadata {
    filename: String,
    content_type: String,
    size: usize,
    path: String,
}

/// Upload response / 上传响应
#[derive(Debug, Serialize, Deserialize)]
struct UploadResponse {
    success: bool,
    message: String,
    files: Vec<FileMetadata>,
}

/// Single file upload handler / 单文件上传处理程序
async fn handle_single_file_upload(mut multipart: Multipart) -> Result<Response> {
    println!("\n--- Single File Upload / 单文件上传 ---");

    let mut uploaded_files = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        let filename = field.filename().unwrap_or("unknown").to_string();
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        println!("Field: {}", name);
        println!("Filename: {}", filename);
        println!("Content-Type: {}", content_type);

        // Validate file type / 验证文件类型
        if !is_allowed_file_type(&filename) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(
                    serde_json::json!({
                        "error": format!("File type not allowed: {}", filename)
                    })
                    .to_string()
                    .into(),
                )
                .unwrap());
        }

        // Read file data / 读取文件数据
        let data = field.bytes().await.unwrap();

        // Save file to disk / 保存文件到磁盘
        let save_path = format!("uploads/{}", filename);
        if let Err(e) = save_file(&save_path, &data) {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(
                    serde_json::json!({
                        "error": format!("Failed to save file: {}", e)
                    })
                    .to_string()
                    .into(),
                )
                .unwrap());
        }

        uploaded_files.push(FileMetadata {
            filename: filename.clone(),
            content_type,
            size: data.len(),
            path: save_path,
        });
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&UploadResponse {
                success: true,
                message: format!("Uploaded {} file(s)", uploaded_files.len()),
                files: uploaded_files,
            })
            .unwrap()
            .into(),
        )
        .unwrap())
}

/// Multiple file upload handler / 多文件上传处理程序
async fn handle_multiple_files_upload(mut multipart: Multipart) -> Result<Response> {
    println!("\n--- Multiple Files Upload / 多文件上传 ---");

    let mut uploaded_files = Vec::new();
    let mut total_size = 0;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.filename().unwrap_or("unknown").to_string();
        let content_type = field.content_type().unwrap_or("").to_string();
        let data = field.bytes().await.unwrap();

        println!("Processing file: {} ({} bytes)", filename, data.len());

        // Validate file / 验证文件
        if let Err(e) = validate_file(&filename, &data) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(
                    serde_json::json!({"error": e.to_string()})
                        .to_string()
                        .into(),
                )
                .unwrap());
        }

        // Save file / 保存文件
        let unique_filename = generate_unique_filename(&filename);
        let save_path = format!("uploads/{}", unique_filename);

        if let Err(e) = save_file(&save_path, &data) {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(
                    serde_json::json!({
                        "error": format!("Failed to save file: {}", e)
                    })
                    .to_string()
                    .into(),
                )
                .unwrap());
        }

        uploaded_files.push(FileMetadata {
            filename: unique_filename.clone(),
            content_type,
            size: data.len(),
            path: save_path,
        });

        total_size += data.len();
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&UploadResponse {
                success: true,
                message: format!(
                    "Uploaded {} file(s), total size: {} bytes",
                    uploaded_files.len(),
                    total_size
                ),
                files: uploaded_files,
            })
            .unwrap()
            .into(),
        )
        .unwrap())
}

/// File upload with form data / 带表单数据的文件上传
async fn handle_file_with_metadata(mut multipart: Multipart) -> Result<Response> {
    println!("\n--- File with Metadata / 带元数据的文件 ---");

    let mut title = String::new();
    let mut description = String::new();
    let mut uploaded_file = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "title" => {
                title = field.text().await.unwrap_or_default();
                println!("Title: {}", title);
            },
            "description" => {
                description = field.text().await.unwrap_or_default();
                println!("Description: {}", description);
            },
            "file" => {
                let filename = field.filename().unwrap_or("unknown").to_string();
                let content_type = field.content_type().unwrap_or("").to_string();
                let data = field.bytes().await.unwrap();

                println!("File: {} ({} bytes)", filename, data.len());

                let unique_filename = generate_unique_filename(&filename);
                let save_path = format!("uploads/{}", unique_filename);

                if let Err(e) = save_file(&save_path, &data) {
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(
                            serde_json::json!({
                                "error": format!("Failed to save file: {}", e)
                            })
                            .to_string()
                            .into(),
                        )
                        .unwrap());
                }

                uploaded_file = Some(FileMetadata {
                    filename: unique_filename,
                    content_type,
                    size: data.len(),
                    path: save_path,
                });
            },
            _ => {
                println!("Unknown field: {}", name);
            },
        }
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(
            serde_json::json!({
                "title": title,
                "description": description,
                "file": uploaded_file
            })
            .to_string()
            .into(),
        )
        .unwrap())
}

/// Check if file type is allowed / 检查文件类型是否允许
fn is_allowed_file_type(filename: &str) -> bool {
    let allowed_extensions = vec![
        ".jpg", ".jpeg", ".png", ".gif", ".webp", // Images / 图片
        ".pdf", ".doc", ".docx", ".txt", // Documents / 文档
        ".mp4", ".avi", ".mov", // Videos / 视频
        ".mp3", ".wav", // Audio / 音频
        ".zip", ".rar", // Archives / 压缩文件
    ];

    allowed_extensions
        .iter()
        .any(|ext| filename.to_lowercase().ends_with(ext))
}

/// Validate file / 验证文件
fn validate_file(filename: &str, data: &[u8]) -> Result<(), String> {
    // Check file type / 检查文件类型
    if !is_allowed_file_type(filename) {
        return Err(format!("File type not allowed: {}", filename));
    }

    // Check file size (max 10MB) / 检查文件大小（最大10MB）
    const MAX_SIZE: usize = 10 * 1024 * 1024;
    if data.len() > MAX_SIZE {
        return Err(format!("File too large: {} bytes (max: {})", data.len(), MAX_SIZE));
    }

    // Check for empty file / 检查空文件
    if data.is_empty() {
        return Err("File is empty".to_string());
    }

    Ok(())
}

/// Generate unique filename / 生成唯一文件名
fn generate_unique_filename(filename: &str) -> String {
    let timestamp = chrono::Utc::now().timestamp();
    let uuid = ulid::Ulid::new();

    if let Some(ext) = Path::new(filename).extension() {
        format!("{}_{}.{}", timestamp, uuid, ext.to_str().unwrap_or("bin"))
    } else {
        format!("{}_{}", timestamp, uuid)
    }
}

/// Save file to disk / 保存文件到磁盘
fn save_file(path: &str, data: &[u8]) -> std::io::Result<()> {
    // Create directory if it doesn't exist / 如果目录不存在则创建
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

/// HTTP server with file upload endpoints / 带文件上传端点的HTTP服务器
async fn file_upload_server() {
    println!("\n=== File Upload Server / 文件上传服务器 ===\n");

    let app = Router::new()
        // Single file upload / 单文件上传
        .post("/api/upload/single", |req: Request| async move {
            let content_type = req.header("content-type").unwrap_or("");
            let body_bytes = req.body().data().clone();
            let max_file_size = 10 * 1024 * 1024; // 10MB
            match Multipart::new(content_type, body_bytes, max_file_size) {
                Ok(multipart) => handle_single_file_upload(multipart).await,
                Err(e) => Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(format!("Invalid multipart request: {}", e).into())
                    .unwrap()),
            }
        })
        // Multiple files upload / 多文件上传
        .post("/api/upload/multiple", |req: Request| async move {
            let content_type = req.header("content-type").unwrap_or("");
            let body_bytes = req.body().data().clone();
            let max_file_size = 10 * 1024 * 1024; // 10MB
            match Multipart::new(content_type, body_bytes, max_file_size) {
                Ok(multipart) => handle_multiple_files_upload(multipart).await,
                Err(e) => Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(format!("Invalid multipart request: {}", e).into())
                    .unwrap()),
            }
        })
        // File with metadata / 带元数据的文件
        .post("/api/upload/metadata", |req: Request| async move {
            let content_type = req.header("content-type").unwrap_or("");
            let body_bytes = req.body().data().clone();
            let max_file_size = 10 * 1024 * 1024; // 10MB
            match Multipart::new(content_type, body_bytes, max_file_size) {
                Ok(multipart) => handle_file_with_metadata(multipart).await,
                Err(e) => Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(format!("Invalid multipart request: {}", e).into())
                    .unwrap()),
            }
        })
        // Upload form page / 上传表单页面
        .get("/upload", |_req: Request| async {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/html")
                .body(include_str!("upload_form.html").to_string().into())
                .unwrap())
        });

    println!("Server configured with file upload endpoints:");
    println!("  POST /api/upload/single - Upload single file");
    println!("  POST /api/upload/multiple - Upload multiple files");
    println!("  POST /api/upload/metadata - Upload file with metadata");
    println!("  GET  /upload - Upload form page");
    println!("\nFile upload features:");
    println!("  - File type validation");
    println!("  - File size limits (10MB max)");
    println!("  - Unique filename generation");
    println!("  - Automatic directory creation");
    println!();
}

/// File upload demonstration / 文件上传演示
fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║   Nexus Multipart File Upload Example / 文件上传示例          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    println!("\nFeatures:");
    println!("  ✓ Single file upload");
    println!("  ✓ Multiple files upload");
    println!("  ✓ File upload with metadata");
    println!("  ✓ File type validation");
    println!("  ✓ File size limits");
    println!("  ✓ Unique filename generation");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(file_upload_server());

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   File upload server ready!                                   ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}
