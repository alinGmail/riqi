## 重构建议
  ### 1. 主循环问题 (src/main.rs:179-190)
  问题: 每次循环都spawn新的事件监听任务，造成任务泄漏

  loop {
      terminal.draw(|frame| { ... })?;
      tokio::spawn(async move { ... }); // 每次循环都spawn!
  }

  建议: 在循环外启动一次事件监听任务

  ### 2. 输入处理逻辑 (src/main.rs:194-251)
  问题: 大量if语句处理键盘输入，难以维护和扩展 建议:

  - 提取为独立的KeyHandler或InputHandler结构
  - 使用match表达式或Command模式
  - 考虑添加可配置的键绑定

  ### 3. 状态管理分散
  问题: calendar在run函数中，select_day在RiqiState中，容易不同步 建议:

  - 将calendar移入RiqiState
  - 添加方法如update_calendar_if_needed()统一管理

  ### 4. 假期更新逻辑重复 (src/main.rs:130-136, 254-261)
  问题: 初始化和更新时重复调用load_holidays，参数完全相同 建议: 提取为riqi_state.reload_holidays()方法

  ### 5. 硬编码主题路径 (src/main.rs:138)
  let theme = load_theme_from_file("resources/theme/ningmen.toml")

  建议: 主题路径应该从配置或命令行参数获取

  ### 6. 错误处理不一致
  - load_holidays使用unwrap_or返回空Map
  - load_theme_from_file使用expect直接panic
  建议: 统一错误处理策略，考虑返回Result并优雅降级

  ### 7. 配置优先级链 (src/config/config_init.rs)
  问题: get_country、get_language等函数重复相同的优先级判断模式 建议: 泛型函数处理配置优先级：

  fn resolve_config<T>(args: Option<T>, file: Option<T>, system: Option<T>) -> Option<T>

  ### 8. 任务去重逻辑有bug (src/holiday/update.rs:64-69)
  let executed = executed_tasks.contains("update_meta");

  问题: 检查的是"update_meta"但函数名是update_holiday_data，导致去重失效 建议: 使用函数参数构造唯一key或使用枚举

  ### 9. 日志配置硬编码 (src/main.rs:46-56)
  问题: 日志文件路径、级别都是硬编码 建议:

  - 日志配置应该从环境变量或配置文件读取
  - 支持不同日志级别（开发/生产环境）

  ### 10. 缺少生命周期规范
  问题: 组件如MonthComponent持有多个引用，但关系不清晰 建议: 考虑使用Rc/Arc共享配置，或重新设计数据所有权

  ### 11. 测试覆盖不足
  - src/main.rs没有单元测试
  - 输入处理逻辑没有测试
  建议: 将业务逻辑从UI循环中分离，便于测试

  ### 12. 异步模型混乱
  - 使用tokio runtime但大部分是同步代码
  - 只有下载和事件处理是异步
  建议: 评估是否真的需要#[tokio::main]，或者更充分利用异步

  ### 优先级排序
  1. 高优先级: 1(任务泄漏)、8(bug)
  2. 中优先级: 2(可维护性)、3(状态管理)、4(代码重复)
  3. 低优先级: 其他优化项
