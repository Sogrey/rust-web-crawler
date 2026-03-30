// 异步并发网络爬虫 - 高性能批量请求示例
use anyhow::Result;
use futures::future::join_all;
use reqwest::Client;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 异步并发网络爬虫启动...\n");

    // 创建 HTTP 客户端（自动管理连接池）
    let client = Client::builder()
        .timeout(Duration::from_secs(10)) // 全局超时
        .build()?;

    // 目标 URL 列表（可扩展为从文件读取）
    let urls = vec![
        "https://rust-lang.org",
        "https://github.com",
        "https://crates.io",
        "https://docs.rs",
        "https://blog.rust-lang.org",
        "https://users.rust-lang.org",
        "https://reddit.com/r/rust",
        "https://stackoverflow.com/questions/tagged/rust",
        "https://hyper.rs",
        "https://tokio.rs",
    ];

    println!("📋 准备抓取 {} 个网页...\n", urls.len());

    // 记录开始时间
    let start = Instant::now();

    // 并发发起所有请求（无阻塞）
    let tasks: Vec<_> = urls
        .iter()
        .map(|url| fetch_url(&client, url))
        .collect();

    // 等待所有请求完成
    let results = join_all(tasks).await;

    // 统计结果
    let mut success_count = 0;
    let mut total_bytes = 0;
    let mut failed_urls = Vec::new();

    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(size) => {
                success_count += 1;
                total_bytes += size;
                println!("✅ [{}/{}] {} - {} 字节", 
                    success_count, urls.len(), urls[i], size);
            }
            Err(e) => {
                failed_urls.push(urls[i]);
                println!("❌ [{}/{}] {} - 错误: {}", 
                    i + 1, urls.len(), urls[i], e);
            }
        }
    }

    // 输出统计信息
    let duration = start.elapsed();
    println!("\n📊 ========== 统计信息 ==========");
    println!("⏱️  总耗时: {:.2} 秒", duration.as_secs_f64());
    println!("✅ 成功: {}/{} ({:.1}%)", 
        success_count, urls.len(), 
        (success_count as f64 / urls.len() as f64) * 100.0);
    println!("📦 总数据量: {} 字节 ({:.2} KB)", 
        total_bytes, total_bytes as f64 / 1024.0);
    println!("⚡ 平均速度: {:.2} 请求/秒", 
        urls.len() as f64 / duration.as_secs_f64());
    println!("💾 平均大小: {:.2} KB/页面", 
        (total_bytes as f64 / 1024.0) / success_count as f64);

    if !failed_urls.is_empty() {
        println!("\n❌ 失败的 URL:");
        for url in failed_urls {
            println!("   - {}", url);
        }
    }

    println!("\n✨ 爬虫任务完成！");
    Ok(())
}

/// 异步抓取单个 URL
async fn fetch_url(client: &Client, url: &str) -> Result<usize> {
    let response = client
        .get(url)
        .timeout(Duration::from_secs(5)) // 单个请求超时
        .send()
        .await?;

    // 读取响应体（确保数据完整）
    let body = response.text().await?;
    
    // 返回实际读取的字节数
    Ok(body.len())
}
