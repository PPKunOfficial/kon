//! Simplified Chinese translations for settings descriptions.

pub fn lookup(text: &str) -> Option<&'static str> {
    Some(match text {
        "(Linux only) choose how window control buttons are laid out in the titlebar." => {
            "（仅 Linux）选择标题栏窗口控制按钮的布局方式。"
        }
        "(Linux only) whether Zed or your compositor should draw window decorations." => {
            "（仅 Linux）选择由 Kon 还是合成器绘制窗口装饰。"
        }
        "(macOS only) render windows using the Display P3 color space." => {
            "（仅 macOS）使用 Display P3 色彩空间渲染窗口。"
        }
        "(macOS only) whether to allow Windows to tab together." => {
            "（仅 macOS）是否允许窗口合并为标签页。"
        }
        "(macOS only) which fullscreen mode the toggle fullscreen action enters." => {
            "（仅 macOS）切换全屏操作进入的全屏模式。"
        }
        "A mapping from languages to files and file extensions that should be treated as that language." => {
            "语言与文件及扩展名的映射关系。"
        }
        "Activates the Python virtual environment, if one is found, in the terminal's working directory." => {
            "如果找到 Python 虚拟环境，则在终端工作目录中激活它。"
        }
        "Additional code actions to run when formatting." => "格式化时额外运行的代码操作。",
        "Allow sending requests to Anthropic models that cannot be offered with Zero Data Retention." => {
            "允许向无法提供零数据保留的 Anthropic 模型发送请求。"
        }
        "Amount of indentation for nested items." => "嵌套条目的缩进量。",
        "Amount of time to wait before changing focus." => "更改焦点前的等待时间。",
        "An optional string to override the title of the terminal tab." => {
            "用于覆盖终端标签页标题的可选字符串。"
        }
        "Any number of settings profiles that are temporarily applied on top of your existing user settings." => {
            "可叠加在用户设置之上的任意数量的设置配置文件。"
        }
        "Automatically close files that have been deleted." => "自动关闭已删除的文件。",
        "Automatically compact the agent's context when it grows too large, summarizing earlier messages to free up room in the model's context window." => {
            "当智能体上下文过大时自动压缩，通过总结较早消息释放模型上下文空间。"
        }
        "Automatically fetch the default branch's remote and the current branch's upstream remote." => {
            "自动获取默认分支的远程和当前分支的上游远程。"
        }
        "Automatically open menus in the titlebar on hover." => "悬停时自动打开标题栏菜单。",
        "Automatically show a signature help pop-up." => "自动显示签名帮助弹窗。",
        "Border style for the minimap's scrollbar thumb." => "缩略图滚动条滑块的边框样式。",
        "Character counts at which to show wrap guides in the editor." => {
            "在编辑器中显示换行参考线的字符数。"
        }
        "Character counts at which to show wrap guides." => "显示换行参考线的字符数。",
        "Choose a static, fixed theme or dynamically select themes based on appearance and light/dark modes." => {
            "选择固定主题，或根据外观和明暗模式动态选择主题。"
        }
        "Choose whether to use the selected light or dark icon theme or to follow your OS appearance configuration." => {
            "选择使用指定的浅色/深色图标主题，或跟随系统外观。"
        }
        "Choose whether to use the selected light or dark theme or to follow your OS appearance configuration." => {
            "选择使用指定的浅色/深色主题，或跟随系统外观。"
        }
        "Collect timing data for foreground and background executor tasks so they can be inspected via `zed: open performance profiler`. May lead to increased memory usage." => {
            "收集前后台执行器任务的耗时数据，可通过 `zed: open performance profiler` 查看。可能增加内存占用。"
        }
        "Command to automatically run when Zed creates a Terminal Thread shell in the agent panel. Runs in your configured shell." => {
            "当 Kon 在智能体面板中创建终端会话时自动运行的命令。会在你配置的 Shell 中执行。"
        }
        "Commands executed in the terminal" => "在终端中执行的命令",
        "Control when to show the active encoding in the status bar." => {
            "控制何时在状态栏显示当前编码。"
        }
        "Control whether Git status is shown in the editor's gutter." => {
            "控制是否在编辑器边栏显示 Git 状态。"
        }
        "Controls automatic indentation behavior when typing." => "控制输入时的自动缩进行为。",
        "Controls how LSP completions are inserted." => "控制 LSP 补全的插入方式。",
        "Controls how words are completed." => "控制单词补全方式。",
        "Controls line number display in the editor's gutter. \"disabled\" shows absolute line numbers, \"enabled\" shows relative line numbers for each absolute line, and \"wrapped\" shows relative line numbers for every line, absolute or wrapped." => {
            "控制编辑器边栏的行号显示方式。\"disabled\" 显示绝对行号，\"enabled\" 为每个绝对行显示相对行号，\"wrapped\" 为每个换行后的行显示相对行号。"
        }
        "Controls the appearance behavior of the tab's close button." => {
            "控制标签页关闭按钮的显示行为。"
        }
        "Controls when to use system clipboard in Vim mode." => {
            "控制 Vim 模式下何时使用系统剪贴板。"
        }
        "Controls where the `editor::rewrap` action is allowed for this language." => {
            "控制该语言中允许使用 `editor::rewrap` 操作的位置。"
        }
        "Controls whether Zed may collect training data when using Zed's Edit Predictions. Data is only collected for files in projects detected as open source. The default value uses the preference previously set via the status-bar toggle, or false if no preference has been stored." => {
            "控制使用编辑预测时是否允许收集训练数据。仅为检测为开源项目的文件收集。默认沿用状态栏开关的偏好，未设置时为 false。"
        }
        "Controls whether edit predictions are shown immediately or manually." => {
            "控制编辑预测是立即显示还是手动触发。"
        }
        "Controls whether the closing characters are always skipped over and auto-removed no matter how they were inserted." => {
            "控制闭合字符是否始终跳过并自动删除，无论其如何插入。"
        }
        "Cursor shape for insert mode. Inherit uses the editor's cursor shape." => {
            "插入模式的光标形状。选择“继承”时使用编辑器光标形状。"
        }
        "Cursor shape for normal mode." => "普通模式的光标形状。",
        "Cursor shape for replace mode." => "替换模式的光标形状。",
        "Cursor shape for the editor." => "编辑器的光标形状。",
        "Cursor shape for visual mode." => "可视模式的光标形状。",
        "Custom digraph mappings for Vim mode." => "Vim 模式的自定义二合字母映射。",
        "Custom line height value (must be at least 1.0)." => "自定义行高值（至少为 1.0）。",
        "Debounce threshold in milliseconds after which changes are reflected in the Git gutter." => {
            "更改反映到 Git 边栏前的防抖阈值（毫秒）。"
        }
        "Default Prettier options, in the format as in package.json section for Prettier." => {
            "默认 Prettier 选项，格式与 package.json 中的 Prettier 配置相同。"
        }
        "Default action when clicking a changed file in the Git panel." => {
            "在 Git 面板中点击已更改文件时的默认操作。"
        }
        "Default branch name will be when init.defaultbranch is not set in Git." => {
            "当 Git 未设置 init.defaultbranch 时使用的默认分支名。"
        }
        "Default cursor shape for the terminal (bar, block, underline, or hollow)." => {
            "终端的默认光标形状（竖线、方块、下划线或空心）。"
        }
        "Default depth to expand outline items in the current file." => {
            "当前文件中展开大纲条目的默认深度。"
        }
        "Default fixed width when the agent panel is docked to the left or right and flexible sizing is disabled." => {
            "智能体面板停靠在左右两侧且禁用弹性尺寸时的默认固定宽度。"
        }
        "Default height in pixels when the debug panel is docked to the bottom." => {
            "调试面板停靠底部时的默认高度（像素）。"
        }
        "Default height when the agent panel is docked to the bottom." => {
            "智能体面板停靠底部时的默认高度。"
        }
        "Default height when the terminal is docked to the bottom (in pixels)." => {
            "终端停靠底部时的默认高度（像素）。"
        }
        "Default timeout in seconds for MCP server tool calls." => {
            "MCP 服务器工具调用的默认超时（秒）。"
        }
        "Default width in pixels when the debug panel is docked to the left or right." => {
            "调试面板停靠左右两侧时的默认宽度（像素）。"
        }
        "Default width of the Git panel in pixels." => "Git 面板的默认宽度（像素）。",
        "Default width of the Threads Sidebar. Changing this setting also updates a manually resized sidebar. Double-click the divider to reset to this width." => {
            "会话侧边栏的默认宽度。修改后会同时更新手动调整过的侧边栏。双击分隔线可重置为此宽度。"
        }
        "Default width of the collaboration panel in pixels." => "协作面板的默认宽度（像素）。",
        "Default width of the outline panel in pixels." => "大纲面板的默认宽度（像素）。",
        "Default width of the project panel in pixels." => "项目面板的默认宽度（像素）。",
        "Default width when the terminal is docked to the left or right (in pixels)." => {
            "终端停靠左右两侧时的默认宽度（像素）。"
        }
        "Delay in milliseconds before automatically requesting a prediction after typing stops. Set to 0 to request predictions immediately." => {
            "停止输入后自动请求预测的延迟（毫秒）。设为 0 表示立即请求。"
        }
        "Delay in milliseconds before drag and drop selection starts." => {
            "开始拖放选择前的延迟（毫秒）。"
        }
        "Delay in milliseconds before the which-key menu appears." => {
            "Which-key 菜单出现前的延迟（毫秒）。"
        }
        "Delay in milliseconds before tooltips appear for project panel titles." => {
            "项目面板标题提示出现前的延迟（毫秒）。"
        }
        "Delay in milliseconds of the project panel title tooltips." => {
            "项目面板标题提示的延迟（毫秒）。"
        }
        "Determines how indent guide backgrounds are colored." => "决定缩进参考线背景的着色方式。",
        "Determines how indent guides are colored." => "决定缩进参考线的着色方式。",
        "Determines how snippets are sorted relative to other completion items." => {
            "决定代码片段相对于其他补全项的排序方式。"
        }
        "Determines the stepping granularity for debug operations." => "决定调试操作的单步粒度。",
        "Direction to split horizontally." => "水平拆分方向。",
        "Direction to split vertically." => "垂直拆分方向。",
        "Directory creation" => "创建目录",
        "Disable all Git integration features in Zed." => "禁用 Kon 中的所有 Git 集成功能。",
        "Disable edit predictions in these language scopes, such as \"comment\" and \"string\". Use \"...\" to add scopes without repeating the inherited list." => {
            "在这些语言作用域中禁用编辑预测，例如 \"comment\" 和 \"string\"。使用 \"...\" 可在不重复继承列表的情况下追加作用域。"
        }
        "Display indent guides in the editor." => "在编辑器中显示缩进参考线。",
        "Display the terminal title in breadcrumbs inside the terminal pane." => {
            "在终端面板内的面包屑中显示终端标题。"
        }
        "Display the which-key menu with matching bindings while a multi-stroke binding is pending. The pending keystrokes indicator remains visible, but its binding preview popover is disabled." => {
            "在多键绑定待输入时显示匹配的 Which-key 菜单。待处理按键指示器仍会显示，但其绑定预览弹窗会被禁用。"
        }
        "Duration in milliseconds to highlight yanked text in Vim mode." => {
            "Vim 模式下高亮复制文本的持续时间（毫秒）。"
        }
        "Enable Helix mode and key bindings." => "启用 Helix 模式和键位绑定。",
        "Enable Vim mode and key bindings." => "启用 Vim 模式和键位绑定。",
        "Enable drag and drop selection." => "启用拖放选择。",
        "Enable middle-click paste on Linux." => "在 Linux 上启用中键粘贴。",
        "Enable smartcase searching in Vim mode." => "在 Vim 模式中启用智能大小写搜索。",
        "Enable to show entries in tree view list, disable to show in flat view list." => {
            "启用时以树形列表显示条目，禁用时以平铺列表显示。"
        }
        "Enables or disables formatting with Prettier for a given language." => {
            "为指定语言启用或禁用 Prettier 格式化。"
        }
        "Extra task variables to set for a particular language." => {
            "为特定语言设置的额外任务变量。"
        }
        "Fast scroll sensitivity multiplier for both horizontal and vertical scrolling." => {
            "水平和垂直滚动的快速滚动灵敏度倍率。"
        }
        "File and directory copying" => "文件和目录复制",
        "File and directory deletion" => "文件和目录删除",
        "File and directory moves/renames" => "文件和目录移动/重命名",
        "File creation and overwrite operations" => "文件创建和覆盖操作",
        "File editing operations" => "文件编辑操作",
        "Files or globs of files that will be excluded by Zed entirely. They will be skipped during file scans, file searches, and not be displayed in the project file tree. Takes precedence over \"File Scan Inclusions\"" => {
            "将被 Kon 完全排除的文件或通配符。它们会跳过文件扫描和搜索，也不会显示在项目树中。优先级高于“文件扫描包含项”。"
        }
        "Files or globs of files that will be included by Zed, even when ignored by git. This is useful for files that are not tracked by git, but are still important to your project. Note that globs that are overly broad can slow down Zed's file scanning. \"File Scan Exclusions\" takes precedence over these inclusions" => {
            "即使被 Git 忽略也会被 Kon 包含的文件或通配符。适用于未被 Git 跟踪但仍重要的文件。过宽的通配符会降低文件扫描速度。“文件扫描排除项”优先级更高。"
        }
        "Font fallbacks for terminal text. If not set, defaults to buffer font fallbacks." => {
            "终端文本的后备字体。未设置时使用编辑器的后备字体。"
        }
        "Font family for Mermaid diagrams in the agent panel and markdown preview. Falls back to the UI font family." => {
            "智能体面板和 Markdown 预览中 Mermaid 图表的字体。回退到界面字体。"
        }
        "Font family for UI elements." => "界面元素使用的字体。",
        "Font family for agent response text in the agent panel. Falls back to the regular UI font family." => {
            "智能体面板中智能体回复文本的字体。回退到常规界面字体。"
        }
        "Font family for code blocks in the markdown preview. Falls back to the editor font family." => {
            "Markdown 预览中代码块的字体。回退到编辑器字体。"
        }
        "Font family for editor text." => "编辑器文本的字体。",
        "Font family for terminal text. If not set, defaults to buffer font family." => {
            "终端文本的字体。未设置时使用编辑器字体。"
        }
        "Font family for the markdown preview. Falls back to the UI font family." => {
            "Markdown 预览的字体。回退到界面字体。"
        }
        "Font family for user messages in the agent panel. Falls back to the regular buffer font family." => {
            "智能体面板中用户消息的字体。回退到常规编辑器字体。"
        }
        "Font features for terminal text." => "终端文本的字体特性。",
        "Font size for UI elements." => "界面元素的字号。",
        "Font size for agent response text in the agent panel. Falls back to the regular UI font size." => {
            "智能体面板中智能体回复文本的字号。回退到常规界面字号。"
        }
        "Font size for editor text." => "编辑器文本的字号。",
        "Font size for terminal text. If not set, defaults to buffer font size." => {
            "终端文本的字号。未设置时使用编辑器字号。"
        }
        "Font size for the markdown preview. Falls back to the editor font size." => {
            "Markdown 预览的字号。回退到编辑器字号。"
        }
        "Font size for user messages text in the agent panel." => {
            "智能体面板中用户消息文本的字号。"
        }
        "Font weight for UI elements (100-900)." => "界面元素的字重（100-900）。",
        "Font weight for editor text (100-900)." => "编辑器文本的字重（100-900）。",
        "Font weight for headings in the markdown preview, in CSS units from 100 to 900." => {
            "Markdown 预览中标题的字重，CSS 单位 100-900。"
        }
        "Font weight for terminal text in CSS weight units (100-900)." => {
            "终端文本的字重（CSS 单位 100-900）。"
        }
        "Forces Prettier integration to use a specific parser name when formatting files with the language." => {
            "强制 Prettier 在格式化该语言文件时使用指定的解析器名称。"
        }
        "Forces Prettier integration to use specific plugins when formatting files with the language." => {
            "强制 Prettier 在格式化该语言文件时使用指定的插件。"
        }
        "Format absolute dates using locale, iso (YYYY-MM-DD HH:MM), or a strftime format such as %Y-%m-%d %H:%M." => {
            "使用系统区域格式、iso（YYYY-MM-DD HH:MM）或 strftime 格式（如 %Y-%m-%d %H:%M）格式化绝对日期。"
        }
        "GNOME-style layout string such as \"close:minimize,maximize\"." => {
            "GNOME 风格的布局字符串，例如 \"close:minimize,maximize\"。"
        }
        "Global switch to toggle hints on and off." => "全局开关，用于启用或禁用提示。",
        "Global switch to toggle inline values on and off when debugging." => {
            "调试时启用或禁用内联值的全局开关。"
        }
        "Globs to match against file paths to determine if a file is private." => {
            "用于匹配文件路径并判断文件是否为私有的通配符。"
        }
        "Globs to match files that will be considered \"hidden\" and can be hidden from the project panel." => {
            "匹配被视为“隐藏”的文件的通配符，可从项目面板中隐藏。"
        }
        "HTTP requests to URLs" => "向 URL 发起的 HTTP 请求",
        "Hide the values of variables in private files." => "隐藏私有文件中变量的值。",
        "Highlight all occurrences of selected text." => "高亮选中文本的所有出现位置。",
        "How Git hunks are displayed visually in the editor." => "编辑器中 Git 代码块的显示方式。",
        "How `zed <path>` opens directories when no flag is specified." => {
            "未指定参数时 `zed <path>` 打开目录的方式。"
        }
        "How and when the scrollbar should be displayed." => "滚动条的显示方式和时机。",
        "How entry statuses are displayed." => "条目状态的显示方式。",
        "How line endings should be handled for new files and during format and save operations." => {
            "新文件以及格式化和保存操作时行尾符的处理方式。"
        }
        "How long to wait between automatic fetches, in seconds. (min: 15)" => {
            "自动获取之间的等待时间（秒，最小 15）。"
        }
        "How many characters has to be in the completions query to automatically show the words-based completions." => {
            "补全查询至少需要多少字符才自动显示基于单词的补全。"
        }
        "How many columns a tab should occupy." => "制表符占用的列数。",
        "How many lines of context to provide in multibuffer excerpts by default." => {
            "多缓冲区摘录默认提供的上下文行数。"
        }
        "How many lines to expand the multibuffer excerpts by default." => {
            "默认展开多缓冲区摘录的行数。"
        }
        "How much to fade out unused code (0.0 - 0.9)." => "未使用代码的淡化程度（0.0 - 0.9）。",
        "How projects open from the UI by default." => "从界面打开项目时的默认方式。",
        "How thinking blocks should be displayed by default. 'Auto' fully expands during streaming, then auto-collapses when done. 'Preview' auto-expands with a height constraint during streaming. 'Always Expanded' shows full content. 'Always Collapsed' keeps them collapsed." => {
            "思考块的默认显示方式。“自动”在流式输出时完全展开，完成后自动折叠；“预览”在流式输出时按高度限制展开；“始终展开”显示完整内容；“始终折叠”保持折叠。"
        }
        "How to display diffs in the editor." => "编辑器中差异的显示方式。",
        "How to display the LSP item kind (function, method, variable, etc.) of each entry in the completions menu." => {
            "补全菜单中 LSP 条目类型（函数、方法、变量等）的显示方式。"
        }
        "How to group entries in the git panel." => "Git 面板中条目的分组方式。",
        "How to highlight the current line in the minimap." => "缩略图中当前行的高亮方式。",
        "How to highlight the current line." => "当前行的高亮方式。",
        "How to indent soft-wrapped lines." => "自动换行行的缩进方式。",
        "How to perform a buffer format." => "缓冲区格式化的执行方式。",
        "How to render LSP color previews in the editor." => "编辑器中 LSP 颜色预览的渲染方式。",
        "How to scroll the target into view when navigating to a definition or reference." => {
            "导航到定义或引用时将目标滚动到可视区域的方式。"
        }
        "How to soft-wrap long lines of text." => "长文本行的自动换行方式。",
        "How to sort entries in the git panel." => "Git 面板中条目的排序方式。",
        "Include ignored files in search results by default." => {
            "默认在搜索结果中包含被忽略的文件。"
        }
        "Key-value pairs to add to the terminal's environment." => "添加到终端环境的键值对。",
        "Layout mode for the bottom dock." => "底部停靠栏的布局模式。",
        "Left padding for centered layout." => "居中布局的左内边距。",
        "Line height for editor text." => "编辑器文本的行高。",
        "Line height for terminal text." => "终端文本的行高。",
        "Loading agent skill instructions" => "加载智能体技能说明",
        "Maximum content width in pixels. Content will be centered when the pane is wider than this value." => {
            "最大内容宽度（像素）。面板宽于此值时内容居中。"
        }
        "Maximum content width in pixels. Content will be centered when the panel is wider than this value." => {
            "最大内容宽度（像素）。面板宽于此值时内容居中。"
        }
        "Maximum directory depth to eagerly index outside of git repositories; contents of directories at this depth or deeper are indexed on demand. Repositories rooted shallower than this depth are always indexed fully. In projects that are not rooted at a git repository, repositories directly inside a root folder activate their git features immediately; deeper ones activate on first use. 0 means no limit and activates all git repositories immediately" => {
            "在 Git 仓库之外主动索引的最大目录深度；该深度及更深的目录内容按需索引。根目录浅于此深度的仓库始终完整索引。未以 Git 仓库为根的项目中，根文件夹内直接包含的仓库会立即启用 Git 功能，更深的仓库在首次使用时启用。0 表示不限制并立即启用所有 Git 仓库。"
        }
        "Maximum length of the commit message title before a warning is shown. Set to 0 to disable." => {
            "显示警告前提交标题的最大长度。设为 0 可禁用。"
        }
        "Maximum number of columns to display in the minimap." => "缩略图显示的最大列数。",
        "Maximum number of idle agent threads with loadable sessions to keep loaded. When the limit is exceeded, the least recently updated threads are unloaded." => {
            "保持加载的最大空闲智能体会话数。超出限制时卸载最近最少更新的会话。"
        }
        "Maximum number of lines to keep in scrollback history (max: 100,000; 0 disables scrolling)." => {
            "回滚历史保留的最大行数（最大 100,000；0 禁用滚动）。"
        }
        "Maximum open tabs in a pane. Will not close an unsaved tab." => {
            "面板中最大打开的标签页数。不会关闭未保存的标签页。"
        }
        "Minimum number of characters to reserve space for in the gutter." => {
            "边栏预留的最小字符数。"
        }
        "Minimum number of lines to display in the agent message editor." => {
            "智能体消息编辑器显示的最小行数。"
        }
        "Minimum time to wait before pulling diagnostics from the language server(s)." => {
            "从语言服务器拉取诊断前的最短等待时间。"
        }
        "Modifier key for adding multiple cursors." => "添加多光标使用的修饰键。",
        "Number of lines to search for modelines (set to 0 to disable)." => {
            "搜索模式行的行数（设为 0 禁用）。"
        }
        "On: format the whole buffer.\nOff: do not format.\nModifications: format only lines with unstaged changes; skips formatting when a git diff or LSP range formatting is unavailable.\nModifications If Available: same, but falls back to formatting the whole buffer." => {
            "开：格式化整个缓冲区。\n关：不格式化。\n仅修改：只格式化有未暂存更改的行；无法使用 Git 差异或 LSP 范围格式化时跳过。\n可用时仅修改：同上，但会回退为格式化整个缓冲区。"
        }
        "Opacity of inactive panels (0.0 - 1.0)." => "非活动面板的不透明度（0.0 - 1.0）。",
        "Optimize Zed's interface for assistive technology such as screen readers. When enabled, otherwise-collapsed controls stay expanded and keyboard-reachable." => {
            "为屏幕阅读器等辅助技术优化 Kon 界面。启用后，原本折叠的控件会保持展开并可通过键盘访问。"
        }
        "Padding between the end of the source line and the start of the inline blame in columns." => {
            "源代码行末尾与内联追溯开始处之间的列间距。"
        }
        "Position of the close button in a tab." => "标签页中关闭按钮的位置。",
        "Preferred debuggers for this language." => "该语言首选的调试器。",
        "Relative size of the drop target in the editor that will open dropped file as a split pane." => {
            "编辑器中拖放目标的相对大小，用于将拖入的文件作为拆分面板打开。"
        }
        "Restore previous file state when reopening." => "重新打开时恢复之前的文件状态。",
        "Right padding for centered layout." => "居中布局的右内边距。",
        "Save after inactivity period (in milliseconds)." => "无活动一段时间后保存（毫秒）。",
        "Scroll sensitivity multiplier for both horizontal and vertical scrolling." => {
            "水平和垂直滚动的灵敏度倍率。"
        }
        "Search case-sensitively by default." => "默认区分大小写搜索。",
        "Search for whole words by default." => "默认全词搜索。",
        "Select input audio device" => "选择输入音频设备",
        "Select output audio device" => "选择输出音频设备",
        "Sets the cursor blinking behavior in the terminal." => "设置终端光标的闪烁方式。",
        "Should the name or path be displayed first in the git view." => {
            "Git 视图中优先显示名称还是路径。"
        }
        "Show Git diff indicators in the scrollbar." => "在滚动条中显示 Git 差异指示器。",
        "Show Git diff information in the editor." => "在编辑器中显示 Git 差异信息。",
        "Show Git status information in the editor." => "在编辑器中显示 Git 状态信息。",
        "Show a background for inlay hints." => "为内联提示显示背景。",
        "Show a badge on the terminal panel icon with the count of open terminals." => {
            "在终端面板图标上显示已打开终端数量徽标。"
        }
        "Show a git status indicator next to file names in the project panel." => {
            "在项目面板文件名旁显示 Git 状态指示器。"
        }
        "Show agent review buttons in the editor toolbar." => {
            "在编辑器工具栏中显示智能体审查按钮。"
        }
        "Show an indicator while a multi-stroke key binding is pending. If the input has a timeout, a countdown is shown and hovering pauses it. Its binding preview popover is disabled when the which-key menu is enabled." => {
            "多键绑定待输入时显示指示器。如果输入有超时，会显示倒计时，悬停可暂停。启用 Which-key 菜单时，其绑定预览弹窗会被禁用。"
        }
        "Show author name as part of the commit information in branch picker." => {
            "在分支选择器的提交信息中显示作者姓名。"
        }
        "Show banners announcing new features in the titlebar." => "在标题栏显示新功能横幅。",
        "Show bookmarks in the gutter." => "在边栏显示书签。",
        "Show breadcrumbs." => "显示面包屑导航。",
        "Show breakpoints in the gutter." => "在边栏显示断点。",
        "Show buffer search result indicators in the scrollbar." => {
            "在滚动条中显示缓冲区搜索结果指示器。"
        }
        "Show code action button at start of buffer line." => "在缓冲区行首显示代码操作按钮。",
        "Show code action buttons in the editor toolbar." => "在编辑器工具栏中显示代码操作按钮。",
        "Show code folding controls in the gutter." => "在边栏显示代码折叠控件。",
        "Show commit summary as part of the inline blame." => "在内联追溯中显示提交摘要。",
        "Show cursor positions in the scrollbar." => "在滚动条中显示光标位置。",
        "Show error and warning count badges next to file names in the project panel." => {
            "在项目面板文件名旁显示错误和警告数量徽标。"
        }
        "Show file icons in the file finder." => "在文件查找器中显示文件图标。",
        "Show file icons in the outline panel." => "在大纲面板中显示文件图标。",
        "Show file icons in the project panel." => "在项目面板中显示文件图标。",
        "Show file icons next to the Git status icon." => "在 Git 状态图标旁显示文件图标。",
        "Show git status indicators on the branch icon in the titlebar." => {
            "在标题栏分支图标上显示 Git 状态指示器。"
        }
        "Show indent guides in the project panel." => "在项目面板中显示缩进参考线。",
        "Show line numbers in the gutter." => "在边栏显示行号。",
        "Show opened editors as preview tabs." => "将打开的编辑器显示为预览标签页。",
        "Show padding for zoomed panes." => "为缩放的面板显示内边距。",
        "Show pinned tabs in a separate row above unpinned tabs." => {
            "在未固定标签页上方单独一行显示固定标签页。"
        }
        "Show quick action buttons (e.g., search, selection, editor controls, etc.)." => {
            "显示快速操作按钮（如搜索、选择、编辑器控件等）。"
        }
        "Show relative or absolute dates in commit history." => "在提交历史中显示相对或绝对日期。",
        "Show relative or absolute dates in graph rows." => "在图表行中显示相对或绝对日期。",
        "Show relative or absolute dates in the blame gutter." => {
            "在追溯边栏中显示相对或绝对日期。"
        }
        "Show relative or absolute dates in the branch picker." => {
            "在分支选择器中显示相对或绝对日期。"
        }
        "Show relative or absolute dates in the stash picker." => {
            "在暂存选择器中显示相对或绝对日期。"
        }
        "Show relative or absolute dates inline and in the status bar." => {
            "在内联和状态栏中显示相对或绝对日期。"
        }
        "Show runnable buttons in the gutter." => "在边栏显示可运行按钮。",
        "Show selected symbol occurrences in the scrollbar." => {
            "在滚动条中显示选中符号的出现位置。"
        }
        "Show selected text occurrences in the scrollbar." => "在滚动条中显示选中文本的出现位置。",
        "Show the Git file status on a tab item." => "在标签页上显示 Git 文件状态。",
        "Show the Git panel button in the status bar." => "在状态栏显示 Git 面板按钮。",
        "Show the Git status in the outline panel." => "在大纲面板中显示 Git 状态。",
        "Show the Git status in the project panel." => "在项目面板中显示 Git 状态。",
        "Show the active language button in the status bar." => "在状态栏显示当前语言按钮。",
        "Show the active line endings button in the status bar." => "在状态栏显示当前行尾符按钮。",
        "Show the avatar of the author of the commit." => "显示提交作者的图像。",
        "Show the branch name button in the titlebar." => "在标题栏显示分支名称按钮。",
        "Show the collaboration panel button in the status bar." => "在状态栏显示协作面板按钮。",
        "Show the cursor position button in the status bar." => "在状态栏显示光标位置按钮。",
        "Show the debugger button in the status bar." => "在状态栏显示调试器按钮。",
        "Show the edit prediction button in the status bar." => "在状态栏显示编辑预测按钮。",
        "Show the file icon for a tab." => "显示标签页的文件图标。",
        "Show the informational hover box when moving the mouse over symbols in the editor." => {
            "鼠标悬停在编辑器符号上时显示信息提示框。"
        }
        "Show the menus in the titlebar." => "在标题栏显示菜单。",
        "Show the name of the active file in the status bar." => "在状态栏显示当前文件名。",
        "Show the navigation history buttons in the tab bar." => "在标签栏显示导航历史按钮。",
        "Show the outline panel button in the status bar." => "在状态栏显示大纲面板按钮。",
        "Show the project diagnostics button in the status bar." => "在状态栏显示项目诊断按钮。",
        "Show the project host and name in the titlebar." => "在标题栏显示项目主机和名称。",
        "Show the project panel button in the status bar." => "在状态栏显示项目面板按钮。",
        "Show the project search button in the status bar." => "在状态栏显示项目搜索按钮。",
        "Show the scrollbar in the project panel." => "在项目面板中显示滚动条。",
        "Show the selections menu in the editor toolbar." => "在编辑器工具栏中显示选区菜单。",
        "Show the signature help pop-up after completions or bracket pairs are inserted." => {
            "插入补全或括号对后显示签名帮助弹窗。"
        }
        "Show the tab bar buttons (New, Split Pane, Zoom)." => {
            "显示标签栏按钮（新建、拆分面板、缩放）。"
        }
        "Show the tab bar in the editor." => "在编辑器中显示标签栏。",
        "Show the terminal button in the status bar." => "在状态栏显示终端按钮。",
        "Show the worktree name button in the titlebar." => "在标题栏显示工作树名称按钮。",
        "Show voting thumbs up/down icon buttons for feedback on agent edits." => {
            "显示用于评价智能体编辑的赞/踩图标按钮。"
        }
        "Show wrap guides (vertical rulers)." => "显示换行参考线（垂直标尺）。",
        "Show wrap guides in the editor." => "在编辑器中显示换行参考线。",
        "Size of the border surrounding the active pane." => "活动面板边框的大小。",
        "Sort order for entries in the project panel." => "项目面板中条目的排序顺序。",
        "Spacing between worktree entries in the project panel." => {
            "项目面板中工作树条目之间的间距。"
        }
        "Start searching as you type in project search, without pressing Enter." => {
            "项目搜索中输入即开始搜索，无需按 Enter。"
        }
        "String substituted for `${separator}` in the window title format. Include any surrounding whitespace in the value." => {
            "窗口标题格式中替换 `${separator}` 的字符串。请包含周围的空白字符。"
        }
        "The API URL to use for Codestral." => "Codestral 使用的 API 地址。",
        "The Codestral model id to use." => "使用的 Codestral 模型 ID。",
        "The Ollama model to use for edit predictions." => "编辑预测使用的 Ollama 模型。",
        "The OpenType features to enable for rendering in UI elements." => {
            "界面元素渲染时启用的 OpenType 特性。"
        }
        "The OpenType features to enable for rendering in text buffers." => {
            "文本缓冲区渲染时启用的 OpenType 特性。"
        }
        "The URL of the Zed server to connect to." => "要连接的 Kon 服务器地址。",
        "The URL of your OpenAI-compatible server's completions API." => {
            "OpenAI 兼容服务器补全 API 的地址。"
        }
        "The amount of padding between the end of the source line and the start of the inline diagnostic." => {
            "源代码行末尾与内联诊断开始处之间的间距。"
        }
        "The arguments to pass to the shell program." => "传递给 Shell 程序的参数。",
        "The base URL of your Ollama server." => "Ollama 服务器的基础地址。",
        "The column at which to soft-wrap lines, for buffers where soft-wrap is enabled." => {
            "启用自动换行的缓冲区中软换行的列位置。"
        }
        "The custom set of icons Zed will associate with files and directories." => {
            "Kon 为文件和目录关联的自定义图标集。"
        }
        "The debounce delay before querying highlights from the language." => {
            "从语言查询高亮前的防抖延迟。"
        }
        "The default mode when Vim starts." => "Vim 启动时的默认模式。",
        "The delay after which the inline blame information is shown." => {
            "显示内联追溯信息的延迟。"
        }
        "The delay in milliseconds to show inline diagnostics after the last diagnostic update." => {
            "最后一次诊断更新后显示内联诊断的延迟（毫秒）。"
        }
        "The directory path to use (will be shell expanded)." => {
            "使用的目录路径（会进行 Shell 展开）。"
        }
        "The dock position of the debug panel." => "调试面板的停靠位置。",
        "The font fallbacks to use for rendering in text buffers." => {
            "文本缓冲区渲染使用的后备字体。"
        }
        "The font fallbacks to use for rendering in the UI." => "界面渲染使用的后备字体。",
        "The icon theme to use when mode is set to dark, or when mode is set to system and it is in dark mode." => {
            "模式为深色，或模式为跟随系统且系统处于深色模式时使用的图标主题。"
        }
        "The icon theme to use when mode is set to light, or when mode is set to system and it is in light mode." => {
            "模式为浅色，或模式为跟随系统且系统处于浅色模式时使用的图标主题。"
        }
        "The list of language servers to use (or disable) for this language." => {
            "该语言使用（或禁用）的语言服务器列表。"
        }
        "The maximum number of tokens to generate." => "生成的最大 Token 数。",
        "The minimum APCA perceptual contrast between foreground and background colors (0-106)." => {
            "前景色和背景色之间的最小 APCA 感知对比度（0-106）。"
        }
        "The minimum APCA perceptual contrast to maintain when rendering text over highlight backgrounds." => {
            "在高亮背景上渲染文本时保持的最小 APCA 感知对比度。"
        }
        "The minimum column at which to display inline diagnostics." => "显示内联诊断的最小列。",
        "The minimum column number at which to show the inline blame information." => {
            "显示内联追溯信息的最小列号。"
        }
        "The minimum width (in columns) at which the split diff view is used. When the editor is narrower, the diff view automatically switches to unified mode. Set to 0 to disable." => {
            "使用拆分差异视图的最小宽度（列）。编辑器更窄时自动切换为统一视图。设为 0 禁用。"
        }
        "The model string to pass to the OpenAI-compatible server." => {
            "传递给 OpenAI 兼容服务器的模型字符串。"
        }
        "The multiplier for scrolling in the terminal with the mouse wheel" => {
            "终端中鼠标滚轮滚动的倍率"
        }
        "The name of a base set of key bindings to use." => "使用的基础键位绑定集名称。",
        "The name of your selected icon theme." => "所选图标主题的名称。",
        "The name of your selected theme." => "所选主题的名称。",
        "The number of characters to keep on either side when scrolling with the mouse." => {
            "鼠标滚动时两侧保留的字符数。"
        }
        "The number of lines to keep above/below the cursor when auto-scrolling." => {
            "自动滚动时在光标上下保留的行数。"
        }
        "The prompt format to use when requesting predictions. Set to Infer to have the format inferred based on the model name." => {
            "请求预测时使用的提示词格式。设为 Infer 时根据模型名称推断格式。"
        }
        "The proxy to use for network requests." => "网络请求使用的代理。",
        "The shell program to run." => "要运行的 Shell 程序。",
        "The shell program to use." => "使用的 Shell 程序。",
        "The text rendering mode to use." => "使用的文本渲染模式。",
        "The theme to use when mode is set to dark, or when mode is set to system and it is in dark mode." => {
            "模式为深色，或模式为跟随系统且系统处于深色模式时使用的主题。"
        }
        "The theme to use when mode is set to light, or when mode is set to system and it is in light mode." => {
            "模式为浅色，或模式为跟随系统且系统处于浅色模式时使用的主题。"
        }
        "The unit for image file sizes." => "图片文件大小的单位。",
        "The width of the active indent guide in pixels, between 1 and 10." => {
            "活动缩进参考线的宽度（像素，1-10）。"
        }
        "The width of the indent guides in pixels, between 1 and 10." => {
            "缩进参考线的宽度（像素，1-10）。"
        }
        "Time in milliseconds until timeout error when connecting to a TCP debug adapter." => {
            "连接 TCP 调试适配器时的超时时间（毫秒）。"
        }
        "Time to wait in milliseconds before hiding the hover popover after the mouse moves away." => {
            "鼠标移开后隐藏悬停弹窗的等待时间（毫秒）。"
        }
        "Time to wait in milliseconds before showing the informational hover box." => {
            "显示信息提示框的等待时间（毫秒）。"
        }
        "Toggle relative line numbers in Vim mode." => "在 Vim 模式中切换相对行号。",
        "Toggles inlay hints (hides or shows) when the user presses the modifiers specified." => {
            "按下指定修饰键时切换内联提示的显示。"
        }
        "Use LSP tasks over Zed language extension tasks." => {
            "优先使用 LSP 任务而不是语言扩展任务。"
        }
        "Use gitignored files when searching." => "搜索时使用被 Git 忽略的文件。",
        "Use native OS dialogs for 'Open' and 'Save As'." => "“打开”和“另存为”使用系统原生对话框。",
        "Use native OS dialogs for confirmations." => "确认操作使用系统原生对话框。",
        "Use regex search by default in Vim search." => "Vim 搜索默认使用正则表达式。",
        "Use regex search by default." => "默认使用正则表达式搜索。",
        "Visible character used to render space characters when show_whitespaces is enabled (default: \"•\")" => {
            "启用显示空白字符时用于渲染空格的可视字符（默认：“•”）。"
        }
        "Visible character used to render tab characters when show_whitespaces is enabled (default: \"→\")" => {
            "启用显示空白字符时用于渲染制表符的可视字符（默认：“→”）。"
        }
        "Web search queries" => "Web 搜索查询",
        "What shell to use when opening a terminal." => "打开终端时使用的 Shell。",
        "What to do after closing the current tab." => "关闭当前标签页后的操作。",
        "What to do when multibuffer is double-clicked in some of its excerpts." => {
            "在多缓冲区的摘录中双击时的操作。"
        }
        "What to do when the last window is closed." => "关闭最后一个窗口时的操作。",
        "What to do when using the 'close active item' action with no tabs." => {
            "在没有标签页时使用“关闭活动项”操作的行为。"
        }
        "What to restore from the previous session when opening Zed." => {
            "打开 Kon 时从上次会话恢复的内容。"
        }
        "What to show for directories in the git panel." => "Git 面板中目录的显示内容。",
        "What to show for directories in the outline panel." => "大纲面板中目录的显示内容。",
        "What to show for directories in the project panel." => "项目面板中目录的显示内容。",
        "What to show when opening a new window; Readme opens a project's README when no items are open, leaving the pane empty if none is found." => {
            "打开新窗口时显示的内容；Readme 在没有打开项时显示项目 README，找不到时留空。"
        }
        "What working directory to use when launching the terminal." => {
            "启动终端时使用的工作目录。"
        }
        "When auto compaction runs. A percentage string like \"90%\" is measured against the context window. A positive integer is the number of used tokens to compact after. A negative integer is the number of tokens remaining in the context window before compacting." => {
            "自动压缩的运行时机。百分比字符串（如 \"90%\"）相对上下文窗口计算；正整数表示已使用多少 Token 后压缩；负整数表示上下文窗口剩余多少 Token 时压缩。"
        }
        "When enabled, agent edits will also be displayed in single-file buffers for review." => {
            "启用后，智能体编辑也会显示在单文件缓冲区中供审查。"
        }
        "When enabled, the :substitute command replaces all matches in a line by default. The 'g' flag then toggles this behavior." => {
            "启用后，Vim 的 :substitute 命令默认替换行内所有匹配项，'g' 标志可切换此行为。"
        }
        "When enabled, use folding ranges from the language server instead of indent-based folding." => {
            "启用后，使用语言服务器的折叠范围而不是基于缩进的折叠。"
        }
        "When enabled, use the language server's document symbols for outlines and breadcrumbs instead of tree-sitter." => {
            "启用后，使用语言服务器的文档符号而不是 tree-sitter 来生成大纲和面包屑。"
        }
        "When false, forcefully disables the horizontal scrollbar." => {
            "为 false 时强制禁用水平滚动条。"
        }
        "When false, forcefully disables the vertical scrollbar." => {
            "为 false 时强制禁用垂直滚动条。"
        }
        "When fetching LSP completions, determines how long to wait for a response of a particular server (set to 0 to wait indefinitely)." => {
            "获取 LSP 补全时，等待特定服务器响应的时长（设为 0 表示无限等待）。"
        }
        "When opening Zed, avoid Restricted Mode by auto-trusting all projects, enabling use of all features without having to give permission to each new project." => {
            "打开 Kon 时自动信任所有项目以避免受限模式，无需为每个新项目授权即可使用全部功能。"
        }
        "When to auto save buffer changes." => "自动保存缓冲区更改的时机。",
        "When to hide the mouse cursor." => "隐藏鼠标光标的时机。",
        "When to play a sound when the agent has either completed its response, or needs user input." => {
            "智能体完成回复或需要用户输入时播放声音的时机。"
        }
        "When to populate a new search's query based on the text under the cursor." => {
            "根据光标下文本填充新搜索查询的时机。"
        }
        "When to scan content of linked directories" => "扫描链接目录内容的时机",
        "When to show edit predictions previews in buffer. The eager mode displays them inline, while the subtle mode displays them only when holding a modifier key." => {
            "编辑预测预览在缓冲区中的显示时机。eager 模式内联显示，subtle 模式仅在按住修饰键时显示。"
        }
        "When to show indent guides in the outline panel." => "在大纲面板中显示缩进参考线的时机。",
        "When to show the minimap in the editor." => "在编辑器中显示缩略图的时机。",
        "When to show the minimap thumb." => "显示缩略图滑块的时机。",
        "When to show the scrollbar in the completion menu." => "补全菜单中显示滚动条的时机。",
        "When to show the scrollbar in the editor." => "编辑器中显示滚动条的时机。",
        "When to show the scrollbar in the terminal." => "终端中显示滚动条的时机。",
        "Where to dock the Git panel." => "Git 面板的停靠位置。",
        "Where to dock the agent panel." => "智能体面板的停靠位置。",
        "Where to dock the collaboration panel." => "协作面板的停靠位置。",
        "Where to dock the outline panel." => "大纲面板的停靠位置。",
        "Where to dock the project panel." => "项目面板的停靠位置。",
        "Where to dock the terminal panel." => "终端面板的停靠位置。",
        "Where to render Git blame when it is enabled." => "启用 Git 追溯时的渲染位置。",
        "Where to show LSP results that can contain multiple locations (Go to Definition, Go to Implementation, Find All References)." => {
            "显示可能包含多个位置的 LSP 结果（转到定义、转到实现、查找所有引用）的位置。"
        }
        "Where to show notifications when the agent has completed its response or needs confirmation before running a tool action." => {
            "智能体完成回复或运行工具操作前需要确认时显示通知的位置。"
        }
        "Where to show the minimap in the editor." => "编辑器中缩略图的显示位置。",
        "Whether alternate scroll mode is active by default (converts mouse scroll to arrow keys in apps like Vim)." => {
            "是否默认启用备用滚动模式（在 Vim 等应用中将鼠标滚动转换为方向键）。"
        }
        "Whether and how to display code lenses from language servers." => {
            "是否以及如何显示语言服务器的代码镜头。"
        }
        "Whether breakpoints should be reused across Zed sessions." => {
            "是否在 Kon 会话之间复用断点。"
        }
        "Whether clicking the stop button on a running terminal tool should also cancel the agent's generation. Note that this only applies to the stop button, not to ctrl+c inside the terminal." => {
            "点击运行中终端工具的停止按钮时是否同时取消智能体生成。仅适用于停止按钮，不适用于终端内的 Ctrl+C。"
        }
        "Whether cmd-click (ctrl-click on Linux and Windows) opens hyperlinks even when the terminal application has enabled mouse reporting. When disabled, these clicks are forwarded to the application; links can still be opened with shift-cmd-click." => {
            "即使终端应用启用了鼠标报告，Cmd+点击（Linux/Windows 上为 Ctrl+点击）是否打开超链接。禁用后这些点击会转发给应用；仍可用 Shift+Cmd+点击打开链接。"
        }
        "Whether edit predictions are shown in normal mode. By default, edit predictions are only shown in insert and replace modes." => {
            "是否在普通模式下显示编辑预测。默认仅在插入和替换模式下显示。"
        }
        "Whether git features show changes relative to HEAD (uncommitted changes) or to the default branch (all changes on the current branch)." => {
            "Git 功能显示相对于 HEAD（未提交更改）还是默认分支（当前分支的所有更改）的变更。"
        }
        "Whether indentation of pasted content should be adjusted based on the context." => {
            "是否根据上下文调整粘贴内容的缩进。"
        }
        "Whether invoking a panel's ToggleFocus action while it's already focused closes the panel, instead of just moving focus back to the editor." => {
            "面板已聚焦时再次调用其 ToggleFocus 操作是否关闭面板，而不是仅将焦点移回编辑器。"
        }
        "Whether newly opened commit views expand every changed file." => {
            "新打开的提交视图是否展开所有已更改文件。"
        }
        "Whether newly opened commit views show the full commit description." => {
            "新打开的提交视图是否显示完整提交描述。"
        }
        "Whether newly opened file diffs show the full file instead of changes only." => {
            "新打开的文件差异是否显示完整文件而不是仅显示更改。"
        }
        "Whether opening a folder in an existing window automatically opens the Threads Sidebar." => {
            "在现有窗口中打开文件夹时是否自动打开会话侧边栏。"
        }
        "Whether or not to automatically check for updates." => "是否自动检查更新。",
        "Whether or not to debounce inlay hints updates after buffer edits (set to 0 to disable debouncing)." => {
            "缓冲区编辑后是否对内联提示更新进行防抖（设为 0 禁用防抖）。"
        }
        "Whether or not to debounce inlay hints updates after buffer scrolls (set to 0 to disable debouncing)." => {
            "缓冲区滚动后是否对内联提示更新进行防抖（设为 0 禁用防抖）。"
        }
        "Whether or not to ensure there's a single newline at the end of a buffer when saving it." => {
            "保存时是否确保缓冲区末尾只有一个换行。"
        }
        "Whether or not to remove any trailing whitespace from lines of a buffer before saving it." => {
            "保存前是否删除缓冲区行尾空白。"
        }
        "Whether or not to restore unsaved buffers on restart." => "重启时是否恢复未保存的缓冲区。",
        "Whether or not to show Git blame data for the currently focused line." => {
            "是否显示当前聚焦行的 Git 追溯数据。"
        }
        "Whether other hints should be shown." => "是否显示其他提示。",
        "Whether parameter hints should be shown." => "是否显示参数提示。",
        "Whether selecting text in the terminal automatically copies to the system clipboard." => {
            "在终端中选择文本时是否自动复制到系统剪贴板。"
        }
        "Whether tasks are enabled for this language." => "是否为此语言启用任务。",
        "Whether the agent panel should use flexible (proportional) sizing when docked to the left or right. When enabled, the default width does not control the panel width, and resetting the panel restores the default proportion." => {
            "智能体面板停靠左右两侧时是否使用弹性（按比例）尺寸。启用后默认宽度不再控制面板宽度，重置面板会恢复默认比例。"
        }
        "Whether the commit message editor is shown in the Git panel by default." => {
            "Git 面板默认是否显示提交消息编辑器。"
        }
        "Whether the cursor blinks in the editor." => "编辑器中光标是否闪烁。",
        "Whether the cursor smoothly animates when moving around the editor." => {
            "光标在编辑器中移动时是否平滑动画。"
        }
        "Whether the editor search results will loop." => "编辑器搜索结果是否循环。",
        "Whether the editor will scroll beyond the last line." => {
            "编辑器是否允许滚动超过最后一行。"
        }
        "Whether the file finder should skip focus for the active file in search results." => {
            "文件查找器是否在搜索结果中跳过活动文件的聚焦。"
        }
        "Whether the git panel should open on startup." => "Git 面板是否在启动时打开。",
        "Whether the hover popover sticks when the mouse moves toward it, allowing interaction with its contents." => {
            "鼠标移向悬停弹窗时弹窗是否保持打开，以便与其内容交互。"
        }
        "Whether the microphone should be muted when joining a channel or a call." => {
            "加入频道或通话时是否将麦克风静音。"
        }
        "Whether the option key behaves as the meta key." => "Option 键是否作为 Meta 键使用。",
        "Whether the project panel should open on startup." => "项目面板是否在启动时打开。",
        "Whether the terminal panel should open on startup." => "终端面板是否在启动时打开。",
        "Whether the terminal panel should use flexible (proportional) sizing when docked to the left or right." => {
            "终端面板停靠左右两侧时是否使用弹性（按比例）尺寸。"
        }
        "Whether the text selection should have rounded corners." => "文本选区是否使用圆角。",
        "Whether to align detail text in code completions context menus left or right." => {
            "代码补全菜单中详情文本的对齐方式（左或右）。"
        }
        "Whether to allow horizontal scrolling in the project panel. When disabled, the view is always locked to the leftmost position and long file names are clipped." => {
            "是否允许项目面板水平滚动。禁用时视图始终锁定在最左侧，过长的文件名会被裁剪。"
        }
        "Whether to always use cmd-enter (or ctrl-enter on Linux or Windows) to send messages." => {
            "是否始终使用 Cmd+Enter（Linux/Windows 上为 Ctrl+Enter）发送消息。"
        }
        "Whether to automatically close JSX tags." => "是否自动闭合 JSX 标签。",
        "Whether to automatically enable case-sensitive search based on the search query." => {
            "是否根据搜索查询自动启用区分大小写搜索。"
        }
        "Whether to automatically generate a title for agent threads using the summary model. When disabled, titles are only created when manually requested." => {
            "是否使用摘要模型自动生成智能体会话标题。禁用时仅在手动请求时创建标题。"
        }
        "Whether to automatically open Markdown files in the preview." => {
            "是否自动以预览方式打开 Markdown 文件。"
        }
        "Whether to automatically open files after pasting or duplicating them." => {
            "粘贴或复制文件后是否自动打开。"
        }
        "Whether to automatically open files dropped from external sources." => {
            "是否自动打开从外部拖入的文件。"
        }
        "Whether to automatically open newly created files in the editor." => {
            "是否在编辑器中自动打开新建的文件。"
        }
        "Whether to automatically surround text with characters for you. For example, when you select text and type '(', Zed will automatically surround text with ()." => {
            "是否自动用字符包围文本。例如选中文本输入 '(' 时，Kon 会自动用 () 包围。"
        }
        "Whether to automatically type closing characters for you. For example, when you type '(', Zed will automatically add a closing ')' at the correct position." => {
            "是否自动输入闭合字符。例如输入 '(' 时，Kon 会在正确位置自动添加 ')'。"
        }
        "Whether to center the current match in the editor" => "是否在编辑器中将当前匹配项居中",
        "Whether to change focus to a pane when the mouse hovers over it." => {
            "鼠标悬停在面板上时是否将焦点切换到该面板。"
        }
        "Whether to collapse untracked files in the diff panel." => {
            "是否在差异面板中折叠未跟踪文件。"
        }
        "Whether to colorize brackets in the editor." => "是否在编辑器中为括号着色。",
        "Whether to constrain the agent panel content to a maximum width, centering it when the panel is wider, for optimal readability." => {
            "是否将智能体面板内容限制在最大宽度内，并在面板更宽时居中，以获得最佳可读性。"
        }
        "Whether to constrain the markdown preview content to a maximum width, centering it when the pane is wider, for optimal readability." => {
            "是否将 Markdown 预览内容限制在最大宽度内，并在面板更宽时居中，以获得最佳可读性。"
        }
        "Whether to disable all AI features in Zed." => "是否禁用 Kon 中的所有 AI 功能。",
        "Whether to display inline and alongside documentation for items in the completions menu." => {
            "是否在补全菜单中同时内联显示和并排显示条目文档。"
        }
        "Whether to enable automatic language detection in unsaved buffers." => {
            "是否在未保存的缓冲区中启用自动语言检测。"
        }
        "Whether to enable drag-and-drop operations in the project panel." => {
            "是否在项目面板中启用拖放操作。"
        }
        "Whether to enable word diff highlighting in the editor. When enabled, changed words within modified lines are highlighted to show exactly what changed." => {
            "是否在编辑器中启用单词级差异高亮。启用后会高亮修改行中具体变化的单词。"
        }
        "Whether to fetch LSP completions or not." => "是否获取 LSP 补全。",
        "Whether to fold directories automatically and show compact folders when a directory has only one subdirectory inside." => {
            "目录只包含一个子目录时是否自动折叠并显示紧凑文件夹。"
        }
        "Whether to fold directories automatically when a directory contains only one subdirectory." => {
            "目录只包含一个子目录时是否自动折叠。"
        }
        "Whether to follow-up empty Go to definition responses from the language server." => {
            "是否跟进语言服务器返回的空“转到定义”响应。"
        }
        "Whether to format DAP messages when adding them to debug adapter logger." => {
            "将 DAP 消息添加到调试适配器日志时是否格式化。"
        }
        "Whether to have edit cards in the agent panel expanded, showing a Preview of the diff." => {
            "智能体面板中的编辑卡片是否默认展开并显示差异预览。"
        }
        "Whether to have terminal cards in the agent panel expanded, showing the whole command output." => {
            "智能体面板中的终端卡片是否默认展开并显示完整命令输出。"
        }
        "Whether to hide symbols, excerpts and search matches in the outline panel when a multi-buffer view is active." => {
            "多缓冲区视图激活时，是否在大纲面板中隐藏符号、摘录和搜索匹配项。"
        }
        "Whether to hide the gitignore entries in the project panel." => {
            "是否在项目面板中隐藏 .gitignore 条目。"
        }
        "Whether to hide the hidden entries in the project panel." => {
            "是否在项目面板中隐藏隐藏条目。"
        }
        "Whether to hide the root entry when only one folder is open in the window." => {
            "窗口中只打开一个文件夹时是否隐藏根条目。"
        }
        "Whether to indent lines using tab characters, as opposed to multiple spaces." => {
            "是否使用制表符而不是多个空格进行缩进。"
        }
        "Whether to keep tabs in preview mode when code navigation is used to navigate away from them. If `enable_preview_file_from_code_navigation` or `enable_preview_multibuffer_from_code_navigation` is also true, the new tab may replace the existing one." => {
            "使用代码导航离开标签页时是否保持预览模式。如果同时启用 `enable_preview_file_from_code_navigation` 或 `enable_preview_multibuffer_from_code_navigation`，新标签页可能替换现有标签页。"
        }
        "Whether to keep the system awake while agent threads are running." => {
            "智能体会话运行期间是否保持系统唤醒。"
        }
        "Whether to keep the text selection after copying it to the clipboard." => {
            "复制到剪贴板后是否保留文本选区。"
        }
        "Whether to log messages between active debug adapters and Zed." => {
            "是否记录活动调试适配器与 Kon 之间的消息。"
        }
        "Whether to open tabs in preview mode when code navigation is used to open a multibuffer." => {
            "代码导航打开多缓冲区时是否以预览模式打开标签页。"
        }
        "Whether to open tabs in preview mode when code navigation is used to open a single file." => {
            "代码导航打开单个文件时是否以预览模式打开标签页。"
        }
        "Whether to open tabs in preview mode when opened from a multibuffer." => {
            "从多缓冲区打开标签页时是否使用预览模式。"
        }
        "Whether to open tabs in preview mode when opened from the project panel with a single click or the Open action." => {
            "从项目面板单击或使用“打开”操作打开标签页时是否使用预览模式。"
        }
        "Whether to open tabs in preview mode when selected from the file finder." => {
            "从文件查找器选择标签页时是否使用预览模式。"
        }
        "Whether to perform linked edits of associated ranges, if the LS supports it. For example, when editing opening <html> tag, the contents of the closing </html> tag will be edited as well." => {
            "语言服务器支持时，是否对关联范围执行联动编辑。例如编辑开头的 <html> 标签时，也会同步修改闭合的 </html> 标签内容。"
        }
        "Whether to play a sound when the BEL character (`\\a`, `0x07`) is printed" => {
            "输出 BEL 字符（`\\a`，`0x07`）时是否播放声音"
        }
        "Whether to pop the completions menu while typing in an editor without explicitly requesting it." => {
            "在编辑器中输入时是否自动弹出补全菜单（无需显式请求）。"
        }
        "Whether to pre-fill the file finder's query with the text selected in the focused item (e.g. an editor or a terminal)." => {
            "是否用当前聚焦项（如编辑器或终端）中选中的文本预填文件查找器查询。"
        }
        "Whether to pull for language server-powered diagnostics or not." => {
            "是否拉取语言服务器提供的诊断。"
        }
        "Whether to reduce non-essential motion, such as loading spinners, by rendering them in a static state." => {
            "是否将加载动画等非必要动态效果以静态方式渲染，以减少动态效果。"
        }
        "Whether to reveal entries in the project panel automatically when a corresponding project entry becomes active." => {
            "对应项目条目激活时是否自动在项目面板中显示该条目。"
        }
        "Whether to reveal when a corresponding outline entry becomes active." => {
            "对应大纲条目激活时是否自动显示。"
        }
        "Whether to scroll when clicking near the edge of the visible text area." => {
            "在可见文本区域边缘附近点击时是否滚动。"
        }
        "Whether to show a badge on the git panel icon with the count of uncommitted changes." => {
            "是否在 Git 面板图标上显示未提交更改数量徽标。"
        }
        "Whether to show diagnostics inline or not." => "是否内联显示诊断。",
        "Whether to show folder names with bold text in the project panel." => {
            "是否在项目面板中以粗体显示文件夹名称。"
        }
        "Whether to show tabs and spaces in the editor." => "是否在编辑器中显示制表符和空格。",
        "Whether to show the addition/deletion change count next to each file in the Git panel." => {
            "是否在 Git 面板中每个文件旁显示新增/删除行数。"
        }
        "Whether to show the agent panel button in the status bar." => {
            "是否在状态栏显示智能体面板按钮。"
        }
        "Whether to show the merge conflict indicator in the status bar that offers to resolve conflicts using the agent." => {
            "是否在状态栏显示合并冲突指示器，以便使用智能体解决冲突。"
        }
        "Whether to show the stage and restore buttons on diff hunks." => {
            "是否在差异代码块上显示暂存和恢复按钮。"
        }
        "Whether to show turn statistics like elapsed time during generation and final turn duration." => {
            "是否显示回合统计，如生成耗时和最终回合时长。"
        }
        "Whether to show warnings or not by default." => "默认是否显示警告。",
        "Whether to sort file and folder names case-sensitively in the project panel." => {
            "项目面板中是否区分大小写排序文件和文件夹名称。"
        }
        "Whether to start a new line with a comment when a previous line is a comment as well." => {
            "上一行是注释时，新行是否也以注释开头。"
        }
        "Whether to stick parent directories at top of the project panel." => {
            "是否将父目录固定在项目面板顶部。"
        }
        "Whether to stick scopes to the top of the editor" => "是否将作用域固定在编辑器顶部",
        "Whether to suggest installing extensions based on the files you open." => {
            "是否根据打开的文件推荐安装扩展。"
        }
        "Whether to use additional LSP queries to format (and amend) the code after every \"trigger\" symbol input, defined by LSP server capabilities" => {
            "是否在每次输入“触发”符号后使用额外的 LSP 查询来格式化（并修正）代码，具体由 LSP 服务器能力定义。"
        }
        "Whether to use command history ranking for sorting in the command palette." => {
            "命令面板是否按命令历史排名排序。"
        }
        "Whether to use language servers to provide code intelligence." => {
            "是否使用语言服务器提供代码智能。"
        }
        "Whether to zoom the editor font size with the mouse wheel while holding the primary modifier key." => {
            "按住主修饰键时是否用鼠标滚轮缩放编辑器字号。"
        }
        "Whether type hints should be shown." => "是否显示类型提示。",
        "Whether your current project should be shared when joining an empty channel." => {
            "加入空频道时是否共享当前项目。"
        }
        "Which diagnostic indicators to show in the scrollbar." => "在滚动条中显示哪些诊断指示器。",
        "Which files containing diagnostic errors/warnings to mark in the project panel." => {
            "在项目面板中标记哪些包含诊断错误/警告的文件。"
        }
        "Which files containing diagnostic errors/warnings to mark in the tabs." => {
            "在标签页中标记哪些包含诊断错误/警告的文件。"
        }
        "Which level to use to filter out diagnostics displayed in the editor." => {
            "使用哪个级别过滤编辑器中显示诊断。"
        }
        "Which settings should be activated only in Preview build of Zed." => {
            "哪些设置仅在 Kon 的预览版本中生效。"
        }
        "Which side of the window the Threads Sidebar appears on." => {
            "会话侧边栏显示在窗口的哪一侧。"
        }
        "Width in pixels of the git diff indicators." => "Git 差异指示器的宽度（像素）。",
        "Width of the git diff indicators in the gutter. Default scales with the buffer font size." => {
            "边栏中 Git 差异指示器的宽度。默认随编辑器字号缩放。"
        }
        "Window title template. Available variables are `${projectName}`, `${fileName}`, `${filePath}`, `${relativePath}`, `${fileStem}`, `${remoteName}`, `${remoteHost}`, `${appName}`, `${branch}`, and `${separator}`. `${separator}` is omitted when adjacent variables are empty, but literal text is preserved. The collaboration indicator, when present, is appended after the rendered template. If the template renders to nothing, the default template is used instead." => {
            "窗口标题模板。可用变量：`${projectName}`、`${fileName}`、`${filePath}`、`${relativePath}`、`${fileStem}`、`${remoteName}`、`${remoteHost}`、`${appName}`、`${branch}` 和 `${separator}`。相邻变量为空时会省略 `${separator}`，但保留字面文本。协作指示器会在渲染后的模板后追加。如果模板渲染为空，则使用默认模板。"
        }
        "when enabled, zed will prefer already-open buffers." => {
            "启用后，Kon 会优先使用已打开的缓冲区。"
        }
        _ => return None,
    })
}
