[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/STsxwOT3)

# Smart Car Lab One - 指令执行器

## 项目简介
本项目实现了智能小车的核心指令执行逻辑，支持北(N)、东(E)、南(S)、西(W)四个方向的移动与转向。

## 核心功能
- **移动 (M)**：根据当前朝向向前移动一个单位坐标。
- **左转 (L)**：逆时针旋转 90 度。
- **右转 (R)**：顺时针旋转 90 度。
- **鲁棒性**：自动忽略指令字符串中的空格及非法字符。

## 测试说明
本项目包含13项覆盖性测试，涵盖了基础位移、全路径转向、复合路径以及异常输入验证。
运行测试指令：
```bash
cargo test