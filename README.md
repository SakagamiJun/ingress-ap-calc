# Ingress 40M AP Calculator

[English](./README_EN.md) | 简体中文

这是一个用 Rust 编写的高性能、轻量级 Ingress AP 计算器。它可以帮助玩家通过动态规划算法，精确计算达到 40,000,000 AP（或其他目标值）所需的动作组合。

## 特性

- **精确计算**：利用动态规划（DP）算法，确保能找到达到目标 AP 的最优路径。
- **高性能**：采用“贪心 + DP”混合策略，极速求解，内存占用极低。
- **多语言支持**：支持中文和英文，自动识别系统语言。
- **灵活配置**：
  - 自定义 Field:Link 比例。
  - 支持全局活动倍率和 Apex 道具倍率。
  - 实时开关特定动作。
  - 设置动作优先级。
- **跨平台**：支持 Windows, macOS, Linux。
- **零依赖**：仅使用 Rust 标准库编写。

## 安装

你可以直接从 [Releases](https://github.com/your-username/ingress-ap-calc/releases) 页面下载预编译的二进制文件。

或者从源代码编译：

```bash
git clone https://github.com/your-username/ingress-ap-calc.git
cd ingress-ap-calc
cargo build --release
```

编译后的文件位于 `target/release/ingress-ap-calc`。

## 使用说明

运行程序后，你将看到一个 REPL 交互界面：

- **输入数值** (如 `39990000`)：设置当前 AP 值并开始计算。
- **输入加减** (如 `+500`)：基于上次输入的 AP 进行增量计算。
- `t <ID>`：开关某个动作（如 `t 1` 开启/关闭扫描）。
- `a <N>`：设置全局 Apex 倍率（如 `a 2`）。
- `g <N>`：设置全局活动倍率（如 `g 2`）。
- `ratio <F> <L>`：设置 Field:Link 的比例。
- `target <N>`：修改目标 AP 值（默认 40,000,000）。
- `priority <ID>`：设置优先使用的动作 ID。
- `lang <cn/en>`：手动切换语言。
- `q`：退出程序。

## 开源协议

本项目采用 [MIT License](./LICENSE) 开源。
