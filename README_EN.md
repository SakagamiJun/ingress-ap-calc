# Ingress 40M AP Calculator

English | [简体中文](./README.md)

A high-performance, lightweight Ingress AP calculator written in Rust. It helps players precisely calculate the combination of actions needed to reach 40,000,000 AP (or any other target) using a Dynamic Programming algorithm.

## Features

- **Precise Calculation**: Uses Dynamic Programming (DP) to ensure the optimal path to the target AP is found.
- **High Performance**: Employs a "Greedy + DP" hybrid strategy for lightning-fast results with minimal memory usage.
- **Multi-language Support**: Supports Chinese and English with automatic system language detection.
- **Flexible Configuration**:
  - Customizable Field:Link ratio.
  - Supports Global Event Multiplier and Apex Item Multiplier.
  - Real-time toggle for specific actions.
  - Set action priorities.
- **Cross-platform**: Supports Windows, macOS, and Linux.
- **Zero Dependencies**: Built entirely with the Rust standard library.

## Installation

You can download pre-compiled binaries from the [Releases](https://github.com/your-username/ingress-ap-calc/releases) page.

Alternatively, build from source:

```bash
git clone https://github.com/your-username/ingress-ap-calc.git
cd ingress-ap-calc
cargo build --release
```

The binary will be located at `target/release/ingress-ap-calc`.

## Usage

After running the program, you will enter a REPL interface:

- **Enter a number** (e.g., `39990000`): Set current AP and start calculation.
- **Enter a plus/minus** (e.g., `+500`): Calculate based on an increment/decrement of the last AP.
- `t <ID>`: Toggle an action (e.g., `t 1` to enable/disable Scan).
- `a <N>`: Set global Apex multiplier (e.g., `a 2`).
- `g <N>`: Set global event multiplier (e.g., `g 2`).
- `ratio <F> <L>`: Set Field:Link ratio.
- `target <N>`: Change the target AP (default 40,000,000).
- `priority <ID>`: Set the ID of the action to be used preferentially.
- `lang <cn/en>`: Manually switch language.
- `q`: Quit.

## License

This project is licensed under the [MIT License](./LICENSE).
