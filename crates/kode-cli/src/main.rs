//! Kode CLI 主程序

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("Kode CLI v{}", env!("CARGO_PKG_VERSION"));
    println!("项目骨架搭建完成！");

    Ok(())
}
