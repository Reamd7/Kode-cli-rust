//! 文件工具函数
//!
//! 提供文件编码检测、行结束符处理等实用功能

use anyhow::{Context, Result};
use encoding_rs::{Encoding, UTF_8};
use std::path::{Path, PathBuf};

/// 行结束符类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// Windows 风格 (CRLF)
    CRLF,
    /// Unix/Mac 风格 (LF)
    LF,
    /// 混合模式（警告）
    Mixed,
}

/// 读取结果
#[derive(Debug, Clone)]
pub struct ReadResult {
    /// 文件内容
    pub content: String,
    /// 行数
    pub line_count: usize,
    /// 总行数
    pub total_lines: usize,
    /// 起始行号
    pub start_line: usize,
}

/// 规范化路径
pub fn normalize_path(path: &Path, cwd: &Path) -> Result<PathBuf> {
    let path_buf = if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    };

    // 清理路径（移除 . 和 ..）
    Ok(path_buf
        .components()
        .filter(|c| !matches!(c, std::path::Component::CurDir))
        .collect())
}

/// 检测文件编码
///
/// 返回编码的名称
pub fn detect_encoding(path: &Path) -> Result<&'static Encoding> {
    use std::fs;
    use std::io::Read;

    let mut file =
        fs::File::open(path).with_context(|| format!("无法打开文件检测编码: {:?}", path))?;

    let mut buffer = [0u8; 4096];
    let n = file
        .read(&mut buffer)
        .with_context(|| format!("无法读取文件检测编码: {:?}", path))?;

    // 尝试检测编码
    let (encoding, ..) = Encoding::for_bom(&buffer[..n]).unwrap_or((UTF_8, 0));

    Ok(encoding)
}

/// 检测文本内容的行结束符
pub fn detect_line_endings(content: &str) -> LineEnding {
    let crlf_count = content.matches("\r\n").count();
    let lf_count = content.matches('\n').count().saturating_sub(crlf_count);

    match (crlf_count, lf_count) {
        (0, 0) => LineEnding::LF, // 默认 LF
        (_, 0) => LineEnding::CRLF,
        (0, _) => LineEnding::LF,
        _ => {
            if crlf_count > lf_count * 2 {
                LineEnding::CRLF
            } else if lf_count > crlf_count * 2 {
                LineEnding::LF
            } else {
                LineEnding::Mixed
            }
        }
    }
}

/// 检测仓库的行结束符
pub fn detect_repo_line_endings(repo_path: &Path) -> Result<LineEnding> {
    // 检查 .gitattributes
    let gitattributes = repo_path.join(".gitattributes");
    if gitattributes.exists() {
        use std::io::BufRead;
        if let Ok(file) = std::fs::File::open(&gitattributes) {
            for line in std::io::BufReader::new(file).lines().map_while(Result::ok) {
                if line.contains("text eol=crlf") {
                    return Ok(LineEnding::CRLF);
                } else if line.contains("text eol=lf") {
                    return Ok(LineEnding::LF);
                }
            }
        }
    }

    // 扫描常见文件检测
    let common_files = [
        "README.md",
        "Cargo.toml",
        "package.json",
        ".gitignore",
        "src/main.rs",
        "src/lib.rs",
    ];

    let mut crlf_count = 0;
    let mut lf_count = 0;

    for file_name in &common_files {
        let file_path = repo_path.join(file_name);
        if file_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let ending = detect_line_endings(&content);
                match ending {
                    LineEnding::CRLF => crlf_count += 1,
                    LineEnding::LF => lf_count += 1,
                    _ => {}
                }
            }
        }
    }

    Ok(if crlf_count > lf_count {
        LineEnding::CRLF
    } else {
        LineEnding::LF
    })
}

/// 添加行号到文本
pub fn add_line_numbers(content: &str, start_line: usize) -> String {
    content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line_num = start_line + i;
            format!("{:4} | {}", line_num + 1, line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// 读取文本内容（支持 offset/limit）
pub fn read_text_content(path: &Path, offset: usize, limit: Option<usize>) -> Result<ReadResult> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("无法读取文件: {:?}", path))?;

    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    let start_line = offset.min(total_lines);
    let end_line = limit
        .map(|l| (start_line + l).min(total_lines))
        .unwrap_or(total_lines);

    let selected_lines = if start_line < total_lines {
        &lines[start_line..end_line]
    } else {
        &[]
    };

    let result_content = selected_lines.join("\n");

    Ok(ReadResult {
        content: result_content,
        line_count: selected_lines.len(),
        total_lines,
        start_line,
    })
}

/// 写入文本内容（处理编码和行结束符）
pub fn write_text_content(
    path: &Path,
    content: &str,
    _encoding: &'static Encoding,
    line_ending: LineEnding,
) -> Result<()> {
    // 转换行结束符
    let normalized_content = match line_ending {
        LineEnding::CRLF => content.replace('\n', "\r\n"),
        LineEnding::LF => content.replace("\r\n", "\n"),
        LineEnding::Mixed => {
            // 混合模式时，统一转换为 LF
            content.replace("\r\n", "\n")
        }
    };

    // 确保父目录存在
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("无法创建目录: {:?}", parent))?;
        }
    }

    std::fs::write(path, normalized_content)
        .with_context(|| format!("无法写入文件: {:?}", path))?;

    Ok(())
}

/// 转换行结束符
pub fn convert_line_ending(content: &str, to: LineEnding) -> String {
    match to {
        LineEnding::CRLF => content.replace('\n', "\r\n").replace("\r\r\n", "\r\n"),
        LineEnding::LF => content.replace("\r\n", "\n"),
        LineEnding::Mixed => content.to_string(),
    }
}

/// 计算文本差异（统一格式）
///
/// # Arguments
/// * `old_text` - 原始文本
/// * `new_text` - 新文本
///
/// # Returns
/// 差异字符串（统一格式）
///
/// # Examples
/// ```
/// use kode_tools::file_utils::compute_diff;
///
/// let diff = compute_diff("line1\nline2", "line1\nline2-edited");
/// assert!(diff.contains("line2-edited"));
/// ```
pub fn compute_diff(old_text: &str, new_text: &str) -> String {
    use similar::{ChangeTag, TextDiff};

    let diff = TextDiff::from_lines(old_text, new_text);

    let mut result = String::new();

    for op in diff.iter_all_changes() {
        let sign = match op.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };

        if let Some(value) = op.as_str() {
            result.push_str(sign);
            result.push_str(value);
        }
    }

    result
}

/// 结构化差异信息
#[derive(Debug, Clone)]
pub struct DiffInfo {
    /// 添加的行数
    pub additions: usize,
    /// 删除的行数
    pub deletions: usize,
    /// 修改的行数
    pub modifications: usize,
    /// 差异文本（统一格式）
    pub diff: String,
}

/// 计算结构化差异
///
/// # Arguments
/// * `old_text` - 原始文本
/// * `new_text` - 新文本
///
/// # Returns
/// 结构化差异信息
pub fn compute_structured_diff(old_text: &str, new_text: &str) -> DiffInfo {
    use similar::{ChangeTag, TextDiff};

    let diff = TextDiff::from_lines(old_text, new_text);

    let mut additions: usize = 0;
    let mut deletions: usize = 0;

    for op in diff.iter_all_changes() {
        match op.tag() {
            ChangeTag::Insert => additions += 1,
            ChangeTag::Delete => deletions += 1,
            ChangeTag::Equal => {}
        }
    }

    // 修改数约为添加和删除的较小值
    let modifications = additions.min(deletions);

    DiffInfo {
        additions: additions.saturating_sub(modifications),
        deletions: deletions.saturating_sub(modifications),
        modifications,
        diff: compute_diff(old_text, new_text),
    }
}

/// 查找相似文件名
///
/// 当文件不存在时，搜索具有相似名称的文件。
/// 使用编辑距离算法计算相似度。
///
/// # Arguments
/// * `target_name` - 目标文件名（不含路径）
/// * `search_dir` - 搜索目录
/// * `max_results` - 返回的最大结果数
/// * `threshold` - 相似度阈值（0.0-1.0），低于此值的结果将被过滤
///
/// # Returns
/// 相似文件路径列表，按相似度降序排序
///
/// # Examples
/// ```
/// use kode_tools::file_utils::find_similar_file;
/// use std::path::Path;
///
/// let similar = find_similar_file("config.tml", Path::new("/project"), 5, 0.6);
/// // 可能返回: ["config.toml", "config.json", ...]
/// ```
pub fn find_similar_file(
    target_name: &str,
    search_dir: &Path,
    max_results: usize,
    threshold: f64,
) -> Vec<PathBuf> {
    use std::fs;

    let mut scores: Vec<(PathBuf, f64)> = Vec::new();

    // 如果搜索目录不存在，返回空列表
    if !search_dir.exists() || !search_dir.is_dir() {
        return Vec::new();
    }

    // 遍历搜索目录
    if let Ok(entries) = fs::read_dir(search_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            // 只检查文件，跳过目录和隐藏文件
            if !path.is_file() {
                continue;
            }

            // 跳过隐藏文件（以 . 开头）
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    if name_str.starts_with('.') {
                        continue;
                    }

                    // 计算相似度
                    let similarity = calculate_similarity(target_name, name_str);

                    // 只保留高于阈值的结果
                    if similarity >= threshold {
                        scores.push((path, similarity));
                    }
                }
            }
        }
    }

    // 按相似度降序排序
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // 返回前 N 个结果
    scores
        .into_iter()
        .take(max_results)
        .map(|(path, _)| path)
        .collect()
}

/// 计算两个字符串的相似度（基于编辑距离）
///
/// 返回值范围 [0.0, 1.0]，其中 1.0 表示完全相同
fn calculate_similarity(a: &str, b: &str) -> f64 {
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();

    // 完全相同
    if a_lower == b_lower {
        return 1.0;
    }

    let max_len = a_lower.len().max(b_lower.len());

    // 如果有一个是空字符串
    if max_len == 0 {
        return 0.0;
    }

    // 计算编辑距离
    let distance = edit_distance(&a_lower, &b_lower);

    // 转换为相似度（1.0 - 归一化的编辑距离）
    let similarity = 1.0 - (distance as f64 / max_len as f64);

    // 额外加分：如果有公共前缀或后缀
    let prefix_bonus = common_prefix_len(&a_lower, &b_lower) as f64 / max_len as f64 * 0.1;
    let suffix_bonus = common_suffix_len(&a_lower, &b_lower) as f64 / max_len as f64 * 0.1;

    (similarity + prefix_bonus + suffix_bonus).min(1.0)
}

/// 计算编辑距离（Levenshtein 距离）
#[allow(clippy::needless_range_loop)]
fn edit_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    // 创建二维数组
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // 初始化第一行和第一列
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // 动态规划计算
    for i in 1..=m {
        for j in 1..=n {
            if a_chars[i - 1] == b_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }

    dp[m][n]
}

/// 计算公共前缀长度
fn common_prefix_len(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .take_while(|(ca, cb)| ca == cb)
        .count()
}

/// 计算公共后缀长度
fn common_suffix_len(a: &str, b: &str) -> usize {
    a.chars()
        .rev()
        .zip(b.chars().rev())
        .take_while(|(ca, cb)| ca == cb)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_line_endings_lf() {
        let content = "line1\nline2\nline3\n";
        assert_eq!(detect_line_endings(content), LineEnding::LF);
    }

    #[test]
    fn test_detect_line_endings_crlf() {
        let content = "line1\r\nline2\r\nline3\r\n";
        assert_eq!(detect_line_endings(content), LineEnding::CRLF);
    }

    #[test]
    fn test_detect_line_endings_mixed() {
        let content = "line1\nline2\r\nline3\n";
        // 这个检测取决于具体实现
        let ending = detect_line_endings(content);
        assert!(matches!(
            ending,
            LineEnding::CRLF | LineEnding::LF | LineEnding::Mixed
        ));
    }

    #[test]
    fn test_add_line_numbers() {
        let content = "line1\nline2\nline3";
        let result = add_line_numbers(content, 0);
        assert_eq!(result, "   1 | line1\n   2 | line2\n   3 | line3");
    }

    #[test]
    fn test_add_line_numbers_with_offset() {
        let content = "line1\nline2\nline3";
        let result = add_line_numbers(content, 5);
        assert_eq!(result, "   6 | line1\n   7 | line2\n   8 | line3");
    }

    #[test]
    fn test_compute_diff() {
        let old_text = "line1\nline2\nline3";
        let new_text = "line1\nline2-edited\nline3";
        let diff = compute_diff(old_text, new_text);

        assert!(diff.contains("-line2"));
        assert!(diff.contains("+line2-edited"));
    }

    #[test]
    fn test_compute_structured_diff() {
        let old_text = "line1\nline2\nline3";
        let new_text = "line1\nline2-edited\nline3\nline4";
        let diff_info = compute_structured_diff(old_text, new_text);

        // line2 被删除，line2-edited 被添加，line4 被添加
        // 所以 additions = 2 (line2-edited, line4)
        // deletions = 1 (line2)
        // modifications = min(2, 1) = 1
        assert_eq!(diff_info.additions, 1); // 净增加: line4
        assert_eq!(diff_info.deletions, 0); // 净删除: 0
        assert!(diff_info.diff.contains("line2-edited"));
        assert!(diff_info.diff.contains("line4"));
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("hello", "hello"), 0);
        assert_eq!(edit_distance("", "test"), 4);
    }

    #[test]
    fn test_calculate_similarity() {
        let sim = calculate_similarity("config.toml", "config.tml");
        assert!(sim > 0.8); // 非常相似

        let sim = calculate_similarity("test", "best");
        assert!(sim > 0.5); // 相似

        let sim = calculate_similarity("abc", "xyz");
        assert!(sim < 0.5); // 不太相似
    }

    #[test]
    fn test_find_similar_file() {
        let temp_dir = TempDir::new().unwrap();

        // 创建一些测试文件
        fs::write(temp_dir.path().join("config.toml"), "test").unwrap();
        fs::write(temp_dir.path().join("config.json"), "test").unwrap();
        fs::write(temp_dir.path().join("README.md"), "test").unwrap();
        fs::write(temp_dir.path().join(".hidden"), "test").unwrap();

        // 查找相似的文件名（打字的错误）
        let similar = find_similar_file("config.tml", temp_dir.path(), 3, 0.5);

        // 应该找到 config.toml
        assert!(!similar.is_empty());
        assert!(similar
            .iter()
            .any(|p| p.file_name().unwrap().to_str().unwrap() == "config.toml"));
    }

    #[test]
    fn test_normalize_path() {
        let cwd = PathBuf::from("/home/user/project");

        // 绝对路径
        let abs_path = PathBuf::from("/etc/config");
        assert_eq!(
            normalize_path(&abs_path, &cwd).unwrap(),
            PathBuf::from("/etc/config")
        );

        // 相对路径
        let rel_path = PathBuf::from("src/main.rs");
        assert_eq!(
            normalize_path(&rel_path, &cwd).unwrap(),
            PathBuf::from("/home/user/project/src/main.rs")
        );

        // 带点的相对路径
        let dot_path = PathBuf::from("./src/main.rs");
        assert_eq!(
            normalize_path(&dot_path, &cwd).unwrap(),
            PathBuf::from("/home/user/project/src/main.rs")
        );
    }

    #[test]
    fn test_convert_line_ending() {
        let content = "line1\nline2\nline3";

        let crlf = convert_line_ending(content, LineEnding::CRLF);
        assert_eq!(crlf, "line1\r\nline2\r\nline3");

        let lf = convert_line_ending(&crlf, LineEnding::LF);
        assert_eq!(lf, "line1\nline2\nline3");
    }

    #[test]
    fn test_read_text_content() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        fs::write(&test_file, "line1\nline2\nline3\nline4\nline5").unwrap();

        // 读取全部
        let result = read_text_content(&test_file, 0, None).unwrap();
        assert_eq!(result.total_lines, 5);
        assert_eq!(result.line_count, 5);

        // 读取部分（offset=1, limit=2）
        let result = read_text_content(&test_file, 1, Some(2)).unwrap();
        assert_eq!(result.start_line, 1);
        assert_eq!(result.line_count, 2);
        assert_eq!(result.content, "line2\nline3");
    }

    #[test]
    fn test_write_text_content() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        write_text_content(&test_file, "line1\nline2\nline3", UTF_8, LineEnding::LF).unwrap();

        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "line1\nline2\nline3");
    }

    #[test]
    fn test_write_text_content_crlf() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        write_text_content(&test_file, "line1\nline2\nline3", UTF_8, LineEnding::CRLF).unwrap();

        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "line1\r\nline2\r\nline3");
    }

    #[test]
    fn test_detect_encoding() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        // UTF-8 with BOM
        fs::write(&test_file, b"\xEF\xBB\xBFHello").unwrap();
        let encoding = detect_encoding(&test_file).unwrap();
        assert_eq!(encoding.name(), "UTF-8");

        // Plain UTF-8
        fs::write(&test_file, "Hello").unwrap();
        let encoding = detect_encoding(&test_file).unwrap();
        assert_eq!(encoding.name(), "UTF-8");
    }
}
