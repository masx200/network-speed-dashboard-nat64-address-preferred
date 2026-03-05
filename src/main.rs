// 纯 HTTP/3 测试工具 - 使用 h3 库
mod main_h3_only;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(main_h3_only::run()?)
}
