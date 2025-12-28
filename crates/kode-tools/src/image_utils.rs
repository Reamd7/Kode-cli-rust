//! 图片处理工具
//!
//! 提供图片尺寸调整、压缩和格式转换功能。

use crate::tool::ToolResult;
use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use std::path::Path;

#[cfg(feature = "image")]
use image_crate::{imageops::FilterType, ImageFormat};

/// 图片最大宽度
const MAX_WIDTH: u32 = 2000;

/// 图片最大高度
const MAX_HEIGHT: u32 = 2000;

/// 图片最大大小 (3.75MB)
const MAX_IMAGE_SIZE: usize = 3_750_000;

// JPEG 压缩质量（保留供未来使用）
// const JPEG_QUALITY: u8 = 80;

/// 处理图片文件
///
/// 读取图片并进行必要的处理：
/// - 如果尺寸超过 MAX_WIDTH 或 MAX_HEIGHT，则缩小
/// - 如果处理后大小仍超过 MAX_IMAGE_SIZE，则压缩为 JPEG
///
/// 如果未启用 `image` feature，则回退到直接编码为 Base64
///
/// # Arguments
/// * `path` - 图片文件路径
///
/// # Returns
/// 返回包含 Base64 编码和元数据的 ToolResult
///
/// # Examples
/// ```no_run
/// use kode_tools::image_utils::process_image;
/// use std::path::Path;
///
/// # fn main() -> anyhow::Result<()> {
/// let result = process_image(Path::new("/path/to/image.png"))?;
/// assert!(result.output.len() > 0);
/// # Ok(())
/// # }
/// ```
pub fn process_image(path: &Path) -> Result<ToolResult> {
    #[cfg(feature = "image")]
    {
        process_image_impl(path)
    }

    #[cfg(not(feature = "image"))]
    {
        // Fallback: 如果没有 image feature，直接返回 Base64
        create_image_response(path)
    }
}

/// 实际的图片处理实现（需要 image feature）
#[cfg(feature = "image")]
fn process_image_impl(path: &Path) -> Result<ToolResult> {
    use std::fs;

    // 1. 读取原始图片文件
    let original_bytes =
        fs::read(path).with_context(|| format!("无法读取图片文件: {}", path.display()))?;

    // 2. 如果原始文件已经在限制内，直接返回
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

    if original_bytes.len() <= MAX_IMAGE_SIZE {
        // 检查尺寸是否也在限制内
        if let Ok(img) = image_crate::load_from_memory(&original_bytes) {
            let (width, height) = (img.width(), img.height());
            if width <= MAX_WIDTH && height <= MAX_HEIGHT {
                // 尺寸和大小都符合，直接返回原始图片
                let base64 = general_purpose::STANDARD.encode(&original_bytes);
                let metadata = serde_json::json!({
                    "size": original_bytes.len(),
                    "format": ext,
                    "width": width,
                    "height": height,
                });
                return Ok(ToolResult::with_metadata(base64, metadata));
            }
        }
    }

    // 3. 加载图片并处理
    let img = image_crate::load_from_memory(&original_bytes).with_context(|| "无法解析图片格式")?;

    let (mut width, mut height) = (img.width(), img.height());

    // 4. 计算缩放后的尺寸（保持宽高比）
    let needs_resize = width > MAX_WIDTH || height > MAX_HEIGHT;

    if needs_resize {
        if width > MAX_WIDTH {
            height = (height * MAX_WIDTH / width).max(1);
            width = MAX_WIDTH;
        }

        if height > MAX_HEIGHT {
            width = (width * MAX_HEIGHT / height).max(1);
            height = MAX_HEIGHT;
        }
    }

    // 5. 执行缩放（如果需要）
    let processed_img = if needs_resize {
        img.resize(width, height, FilterType::Lanczos3)
    } else {
        img
    };

    // 6. 编码为 buffer
    let mut buffer = Vec::new();
    let format = ImageFormat::from_extension(ext).unwrap_or(ImageFormat::Png);

    // 尝试保持原始格式
    let encode_result = if needs_resize || original_bytes.len() > MAX_IMAGE_SIZE {
        // 如果调整了大小或原始文件过大，使用 JPEG 格式以获得更好的压缩
        processed_img.write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Jpeg)
    } else {
        processed_img.write_to(&mut std::io::Cursor::new(&mut buffer), format)
    };

    if encode_result.is_err() || buffer.len() > MAX_IMAGE_SIZE {
        // 7. 如果编码失败或仍然过大，使用 JPEG 压缩
        buffer.clear();
        processed_img
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Jpeg)
            .context("JPEG 编码失败")?;
    }

    // 8. 返回处理后的图片
    let base64 = general_purpose::STANDARD.encode(&buffer);
    let output_format = ext;

    let metadata = serde_json::json!({
        "size": buffer.len(),
        "format": output_format,
        "width": width,
        "height": height,
        "original_size": original_bytes.len(),
        "resized": needs_resize,
    });

    Ok(ToolResult::with_metadata(base64, metadata))
}

/// 创建图片响应（不处理，直接编码）
///
/// 这是 process_image 的简化版本，直接将图片编码为 Base64
/// 当未启用 `image` feature 时使用
///
/// # Arguments
/// * `path` - 图片文件路径
///
/// # Returns
/// 返回包含 Base64 编码的 ToolResult
pub fn create_image_response(path: &Path) -> Result<ToolResult> {
    use std::fs;

    let bytes = fs::read(path)?;
    let base64 = general_purpose::STANDARD.encode(&bytes);

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown");

    let metadata = serde_json::json!({
        "size": bytes.len(),
        "format": ext,
    });

    Ok(ToolResult::with_metadata(base64, metadata))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_constants() {
        assert_eq!(MAX_WIDTH, 2000);
        assert_eq!(MAX_HEIGHT, 2000);
        assert_eq!(MAX_IMAGE_SIZE, 3_750_000);
        // JPEG_QUALITY is commented out for future use
        // assert_eq!(JPEG_QUALITY, 80);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_process_image_without_file() {
        // 测试不存在的文件
        let result = process_image(Path::new("/nonexistent/file.png"));
        assert!(result.is_err());
    }
}
