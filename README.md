# 🚀 Rust 异步并发网络爬虫

高性能异步并发网络爬虫，支持批量抓取网页、接口数据统计。

## ✨ 核心特性

- ⚡ **异步并发** - 使用 Tokio 运行时，单机轻松实现数千并发
- ⏱️ **超时控制** - 每个请求独立的超时机制（默认 5 秒）
- 📦 **批量请求** - 支持批量 URL 处理
- 🛡️ **错误隔离** - 单个请求失败不影响其他请求
- 📊 **性能统计** - 实时显示耗时、成功率、数据量、速度

## 🎯 性能表现

实测数据（10 个网页并发抓取）：

```
⏱️  总耗时: 1.41 秒
✅ 成功率: 100% (10/10)
📦 总数据量: 978.86 KB
⚡ 平均速度: 7.11 请求/秒
```

### 性能对比

| 语言/方案 | 耗时 | 性能对比 |
|----------|------|---------|
| Rust 异步 | **1.4s** | 🚀 基准 |
| Python 协程 | ~5-10s | 🐌 慢 3-7 倍 |
| Python 同步 | ~15-20s | 🐌 慢 10-15 倍 |

## 🛠️ 技术栈

| 组件 | 库 | 版本 | 用途 |
|------|------|------|------|
| 异步运行时 | [tokio](https://tokio.rs/) | 1.0+ | 提供异步执行环境 |
| HTTP 客户端 | [reqwest](https://docs.rs/reqwest/) | 0.11+ | 发起 HTTP 请求 |
| 异步工具 | [futures](https://docs.rs/futures/) | 0.3+ | 并发控制（join_all） |
| 错误处理 | [anyhow](https://docs.rs/anyhow/) | 1.0+ | 简化错误处理 |

## 📦 安装

确保已安装 Rust 工具链，然后克隆项目：

```bash
git clone git@github.com:Sogrey/rust-web-crawler.git
cd rust-web-crawler
```

## 🚀 快速开始

### 开发模式运行

```bash
cargo run
```

### 生产模式运行（推荐）

```bash
cargo run --release
```

## 📖 使用示例

### 基本用法

```rust
use reqwest::Client;
use futures::future::join_all;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let urls = vec![
        "https://rust-lang.org",
        "https://github.com",
        // 添加更多 URL...
    ];

    // 并发发起所有请求
    let tasks: Vec<_> = urls
        .iter()
        .map(|url| fetch_url(&client, url))
        .collect();

    let results = join_all(tasks).await;
    
    // 处理结果...
    Ok(())
}

async fn fetch_url(client: &Client, url: &str) -> anyhow::Result<usize> {
    let body = client
        .get(url)
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;
    Ok(body.len())
}
```

### 输出示例

```
🚀 异步并发网络爬虫启动...

📋 准备抓取 10 个网页...

✅ [1/10] https://rust-lang.org - 18601 字节
✅ [2/10] https://github.com - 566006 字节
...

📊 ========== 统计信息 ==========
⏱️  总耗时: 1.41 秒
✅ 成功: 10/10 (100.0%)
📦 总数据量: 1002356 字节 (978.86 KB)
⚡ 平均速度: 7.11 请求/秒

✨ 爬虫任务完成！
```

## 🏗️ 项目结构

```
rust-web-crawler/
├── Cargo.toml              # 项目依赖配置
├── src/
│   └── main.rs             # 主程序入口
├── PROJECT_PLAN.md         # 项目规划文档
└── README.md               # 本文档
```

## 🔧 扩展方向

- [ ] 添加并发数限制（Semaphore）
- [ ] 实现请求重试机制
- [ ] 支持 HTTPS 代理
- [ ] 添加请求速率限制
- [ ] 实现断点续爬
- [ ] 数据持久化存储
- [ ] 支持 JavaScript 渲染（Headless Browser）

## 🎓 适用场景

- ✅ 批量抓取静态网页
- ✅ API 接口数据采集
- ✅ 网站健康检查
- ✅ 数据监控统计
- ✅ 竞品价格监控

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

[MIT License](LICENSE)

## 🔗 相关资源

- [Tokio 官方文档](https://tokio.rs/tokio/tutorial)
- [Reqwest 文档](https://docs.rs/reqwest/)
- [Rust 异步编程指南](https://rust-lang.github.io/async-book/)

---

**⭐ 如果这个项目对你有帮助，请给一个 Star！**
