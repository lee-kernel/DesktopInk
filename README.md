# Desktop Ink · 桌面文字

使用 Rust 和 Windows 原生 Win32 API 编写的透明桌面文字工具，无第三方 Rust 依赖。

## 下载

- 稳定版本：在 [Releases](https://github.com/lee-kernel/DesktopInk/releases) 下载 `DesktopInk-windows-x64.zip`，解压后运行 `DesktopInk.exe`。附有 SHA256 校验文件。
- 最新构建：进入 [Windows build](https://github.com/lee-kernel/DesktopInk/actions/workflows/windows-build.yml)，打开成功的运行，在 Artifacts 下载 `DesktopInk-windows-x64`（需要登录 GitHub，保留 30 天）。下载后解压构建产物，再解压其中的软件压缩包。

提交到 main、提交拉取请求或手动运行工作流时，会自动编译 Windows x64 版本并上传下载产物。发布 GitHub Release 时，会自动编译对应版本并将压缩包和校验文件附到该 Release。发布流程不需要额外配置个人访问令牌。

## 使用

双击 `DesktopInk.exe`。左侧选择文字，右侧编辑后点击“应用并保存”。

- 点击“添加”创建独立的文字浮层；“删除”移除选中的文字。
- 解锁时用鼠标左键拖动文字；透明背景不接收点击，请从文字笔画处拖动。
- “锁定 / 点击穿透”作用于全部文字。锁定后仍可通过管理窗口编辑和解锁。
- 支持多行文字、本机已安装字体下拉选择（也可输入名称）、8–200 像素字号、十六进制颜色、浮层宽高。字体列表在程序启动时读取，新安装字体请重启程序后选择。
- 点击字体或背景色块打开 Windows 颜色选择器，可从色板选择或自定义颜色，也可以输入 #RRGGBB。
- 勾选“背景完全透明”只显示文字；取消后显示所选背景色。选择背景颜色会自动取消此勾选。
- “整体透明度”滑块范围为 0%–90%，同时作用于文字和背景；0% 为不透明，90% 最透明。点击“应用并保存”生效。
- “浮层置顶”关闭时文字可被其他应用覆盖，开启时显示在普通应用窗口上方。设置按每段文字独立保存，同样需要点击“应用并保存”。系统安全桌面及其他置顶窗口不受此开关控制。
- 超过指定宽高的文字会被裁切，可以在管理窗口增加宽高。
- “移回主屏”将选中的文字放到屏幕坐标 (80, 100)，可用于找回移出屏幕的文字。
- 最小化管理窗口，文字继续显示；Ctrl+Shift+D 恢复管理窗口（快捷键被其他程序占用时不可用）。
- 点击“－”最小化到任务栏；点击 × 或按 Alt+F4 隐藏到右下角托盘，桌面文字仍保留。
- 点击或双击托盘图标恢复管理窗口；右键托盘图标可选择“打开管理窗口”或“退出程序”。如果图标被 Windows 折叠，请查看托盘的隐藏图标区域。
- Ctrl+Shift+D 也可恢复管理窗口。托盘添加失败时会退回最小化到任务栏，避免窗口无法找回；Explorer 重启后会尝试恢复托盘图标。
- 点击“退出程序”才真正退出并移除文字浮层；再次启动会恢复已保存内容和位置。

配置位于程序旁的 `desktop-ink.settings`，需要该目录可写。文字内容以 UTF-8 十六进制编码保存，不是加密。请把程序放在自己的可写目录。

## 开机自启动

勾选左侧“登录 Windows 时自动启动”即可立即启用，取消勾选立即关闭；无需点击“应用并保存”。默认不会自动启用。

该功能只设置当前 Windows 用户的 Run 启动项，登录后以 `--background` 模式运行，隐藏管理窗口、保留托盘图标并恢复已保存文字。Windows 系统启动管理策略可能影响执行时间，尚未实际重启电脑验证。

如果移动了程序位置，请在新位置重新启用自启动，以更新路径。系统机制参考 [Microsoft Run 启动项说明](https://learn.microsoft.com/en-us/windows/win32/setupapi/run-and-runonce-registry-keys)。

## 显示方式

文字是无边框透明窗口，不修改壁纸。默认可被普通应用窗口覆盖，也可按需启用“浮层置顶”。窗口没有挂入 Explorer 桌面窗口，Windows 的“显示桌面”操作可能隐藏浮层。

采用固定像素布局并声明 DPI 感知，尚未实现跨屏 DPI 动态缩放。支持中文和多行文字，但不是富文本编辑器。本版不包含壁纸写入或动态系统信息。

透明背景使用 Win32 分层窗口颜色键，参考 [Microsoft SetLayeredWindowAttributes 文档](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setlayeredwindowattributes)。

## 编译

需要 Windows x64、Rust MSVC 工具链、Microsoft C++ 链接工具及 Windows SDK 的 rc.exe（用于嵌入图标）。

```powershell
cargo build --release
```

或运行随附 `build.ps1`。该脚本读取现有用户级 Rust 环境变量，仅作用于当前 PowerShell 进程，不修改系统设置。

## 已验证

在本机使用 Rust 1.99.0 编译通过，程序退出码为 0 的自动检查覆盖：Unicode 配置往返、无效配置拒绝、原生窗口和控件创建、文字增删、多行中文编辑、字号和颜色转换、锁定与穿透样式、位置保存、绘制调用、配置恢复、空列表恢复、本机字体枚举与选择应用、关闭后浮层仍显示、管理窗口恢复、明确退出及嵌入图标资源加载。

自动检查不等同于人工视觉验收。可在临时目录中的程序副本上使用 `--self-test` 或 `--smoke-test` 重跑检查；结果会写到该副本旁。

0.1.3 额外验证颜色选择窗口打开并接受颜色、字体/背景颜色、透明度滑块、浮层 Alpha 与透明背景模式、90% 透明度保存恢复、置顶/取消置顶、旧配置读取，以及自启动注册的启用、读取与关闭。自启动测试使用独立临时注册表项，测试后删除，不改变真实 Run 启动项。

0.1.4 调整管理窗口布局：缩短左侧选项标题并将说明分行显示，使用微软雅黑界面字体，压缩编辑区并按客户区尺寸计算窗口大小。自动检查确认全部有编号的控件位于客户区内，两个左侧选项的文字测量宽度不超过可用空间。

## 图标来源

应用图标采用 [Lucide 的 Type 图标](https://lucide.dev/icons/type)，添加蓝色圆角底色，并生成 16–256 像素的多尺寸 ICO。原始 SVG、转换脚本和完整授权文本均保存在 `assets/` 中。

图标来源：[Lucide 官方仓库](https://github.com/lucide-icons/lucide/blob/main/icons/type.svg)。此图标源于 Feather；完整许可证包含 Lucide ISC 和 Feather MIT 授权，见 `assets/LUCIDE-LICENSE.txt`。
