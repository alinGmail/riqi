
这是一个命令行日期工具

ui 库使用 Ratatui

目标：

- [ ] 设计数据结果
- [ ] 输出当前月的日期

- [ ] 加入命令行的库
- [ ] 加入ui库

-[ ] Ratatui

- [] rattui 的block 是如何套嵌的？

rattui 的颜色问题

use ratatui::style::Color;

let red = Color::Rgb(255, 0, 0);

嵌套 Block 的关键技巧
使用 inner() 方法：

Block::inner() 方法可以获取去掉边框和标题后的内部区域

这是嵌套布局的基础

合理设置 margin：

通过 Layout::margin() 为嵌套 Block 留出空间

样式继承与覆盖：

内层 Block 可以继承外层样式，也可以覆盖

边框处理：

使用 Borders 的各个变体控制哪些边显示边框

例如 Borders::LEFT | Borders::RIGHT 只显示左右边框

性能考虑：

避免过度嵌套，通常 3-4 层足够大多数场景

复杂的嵌套可以考虑拆分为多个函数或组件

## layout 的计算

竖直布局的问题？

只考虑够的情况
