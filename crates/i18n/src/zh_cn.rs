//! Simplified Chinese translations.
//!
//! The match arms are keyed by the English source string, so a missing entry
//! simply falls back to English.

pub fn lookup(text: &str) -> Option<&'static str> {
    Some(match text {
        // Menus
        "File" => "文件",
        "Edit" => "编辑",
        "Selection" => "选择",
        "View" => "视图",
        "Go" => "转到",
        "Run" => "运行",
        "Window" => "窗口",
        "Help" => "帮助",
        "About Kon" => "关于 Kon",
        "Check for Updates" => "检查更新",
        "Settings" => "设置",
        "Open Settings" => "打开设置",
        "Open Settings File" => "打开设置文件",
        "Open Project Settings" => "打开项目设置",
        "Open Project Settings File" => "打开项目设置文件",
        "Open Default Settings" => "打开默认设置",
        "Open Keymap" => "打开快捷键映射",
        "Open Keymap File" => "打开快捷键映射文件",
        "Open Default Key Bindings" => "打开默认键位绑定",
        "Select Theme..." => "选择主题...",
        "Select Icon Theme..." => "选择图标主题...",
        "Services" => "服务",
        "Extensions" => "扩展",
        "Install CLI" => "安装命令行工具",
        "Hide Kon" => "隐藏 Kon",
        "Hide Others" => "隐藏其他",
        "Show All" => "全部显示",
        "Quit Kon" => "退出 Kon",
        "New" => "新建",
        "New Window" => "新建窗口",
        "Open File..." => "打开文件...",
        "Open Folder..." => "打开文件夹...",
        "Open…" => "打开...",
        "Open Recent…" => "打开最近...",
        "Open Remote…" => "打开远程...",
        "Add Folder to Project…" => "将文件夹添加到项目...",
        "Save" => "保存",
        "Save As…" => "另存为...",
        "Save All" => "全部保存",
        "Close Editor" => "关闭编辑器",
        "Close Project" => "关闭项目",
        "Close Window" => "关闭窗口",
        "Undo" => "撤销",
        "Redo" => "重做",
        "Cut" => "剪切",
        "Copy" => "复制",
        "Copy and Trim" => "复制并去除空白",
        "Paste" => "粘贴",
        "Find" => "查找",
        "Find in Project" => "在项目中查找",
        "Toggle Line Comment" => "切换行注释",
        "Select All" => "全选",
        "Expand Selection" => "扩大选择",
        "Shrink Selection" => "缩小选择",
        "Select Next Sibling" => "选择下一个同级节点",
        "Select Previous Sibling" => "选择上一个同级节点",
        "Add Cursor Above" => "在上方添加光标",
        "Add Cursor Below" => "在下方添加光标",
        "Select Next Occurrence" => "选择下一个匹配项",
        "Select Previous Occurrence" => "选择上一个匹配项",
        "Select All Occurrences" => "选择所有匹配项",
        "Move Line Up" => "上移行",
        "Move Line Down" => "下移行",
        "Duplicate Selection" => "复制选中内容",
        "Back" => "后退",
        "Forward" => "前进",
        "Command Palette..." => "命令面板...",
        "Execute a command..." => "执行命令...",
        "Go to File..." => "转到文件...",
        "Go to Symbol in Editor..." => "转到编辑器中的符号...",
        "Go to Line/Column..." => "转到行/列...",
        "Go to Definition" => "转到定义",
        "Go to Declaration" => "转到声明",
        "Go to Type Definition" => "转到类型定义",
        "Find All References" => "查找所有引用",
        "Show Incoming Calls" => "显示传入调用",
        "Show Outgoing Calls" => "显示传出调用",
        "Next Problem" => "下一个问题",
        "Previous Problem" => "上一个问题",
        "Spawn Task" => "运行任务",
        "Start Debugger" => "启动调试器",
        "Edit tasks.json…" => "编辑 tasks.json...",
        "Edit debug.json…" => "编辑 debug.json...",
        "Continue" => "继续",
        "Step Over" => "单步跳过",
        "Step Into" => "单步进入",
        "Step Out" => "单步跳出",
        "Toggle Breakpoint" => "切换断点",
        "Edit Breakpoint" => "编辑断点",
        "Clear All Breakpoints" => "清除所有断点",
        "Minimize" => "最小化",
        "Zoom" => "缩放",
        "Zoom In" => "放大",
        "Zoom Out" => "缩小",
        "Reset Zoom" => "重置缩放",
        "Reset All Zoom" => "重置所有缩放",
        "Toggle Left Dock" => "切换左侧停靠栏",
        "Toggle Right Dock" => "切换右侧停靠栏",
        "Toggle Bottom Dock" => "切换底部停靠栏",
        "Toggle All Docks" => "切换所有停靠栏",
        "Editor Layout" => "编辑器布局",
        "Split Up" => "向上拆分",
        "Split Down" => "向下拆分",
        "Split Left" => "向左拆分",
        "Split Right" => "向右拆分",
        "Project Panel" => "项目面板",
        "Outline Panel" => "大纲面板",
        "Collab Panel" => "协作面板",
        "Terminal Panel" => "终端面板",
        "Debugger Panel" => "调试器面板",
        "Agent Panel" => "智能体面板",
        "Git Panel" => "Git 面板",
        "Diagnostics" => "诊断",
        "Toggle GPUI Inspector" => "切换 GPUI 检查器",
        "View Release Notes Locally" => "在本地查看发行说明",
        "View Dependency Licenses" => "查看依赖许可证",
        "Show Welcome" => "显示欢迎页",
        "File Bug Report..." => "报告 Bug...",
        "Request Feature..." => "请求新功能...",
        "Email Us..." => "给我们发邮件...",
        "Documentation" => "文档",
        "Zed Repository" => "Zed 代码仓库",
        "Zed Twitter" => "Zed Twitter",
        "Join the Team" => "加入团队",
        "Open Files" => "打开文件",
        "Open Recent" => "打开最近",
        "Open Remote" => "打开远程",
        "Toggle Terminal" => "切换终端",
        "Toggle Project Panel" => "切换项目面板",
        "Toggle Agent Panel" => "切换智能体面板",
        "Toggle Git Panel" => "切换 Git 面板",
        "Toggle Outline Panel" => "切换大纲面板",
        "Toggle Collaboration Panel" => "切换协作面板",
        "Toggle Debugger Panel" => "切换调试器面板",
        "Format Document" => "格式化文档",
        "Go to Line" => "转到行",
        "Rename" => "重命名",
        "Delete" => "删除",
        "Copy Path" => "复制路径",
        "Reveal in Finder" => "在访达中显示",
        "Reveal in File Explorer" => "在文件资源管理器中显示",
        "Reveal in File Manager" => "在文件管理器中显示",
        "Open in Terminal" => "在终端中打开",

        // Welcome page
        "Get Started" => "开始使用",
        "New File" => "新建文件",
        "Open Project" => "打开项目",
        "Clone Repository" => "克隆仓库",
        "Open Command Palette" => "打开命令面板",
        "Configure" => "配置",
        "Customize Keymaps" => "自定义快捷键",
        "Explore Extensions" => "浏览扩展",
        "Collaborate with Agents" => "与智能体协作",
        "Open Agent Panel" => "打开智能体面板",
        "Recent Projects" => "最近项目",
        "Welcome back to Kon" => "欢迎回到 Kon",
        "Welcome to Kon" => "欢迎使用 Kon",
        "The editor for what's next" => "面向未来的编辑器",
        "Return to Onboarding" => "返回引导",
        "Run multiple threads at once, mix and match any ACP-compatible agent, and keep work conflict-free with worktrees." => {
            "同时运行多个线程，自由组合任意兼容 ACP 的智能体，并使用工作树避免工作冲突。"
        }

        // Settings pages
        "General" => "通用",
        "Appearance" => "外观",
        "Keymap" => "快捷键",
        "Editor" => "编辑器",
        "Languages & Tools" => "语言与工具",
        "Search & Files" => "搜索与文件",
        "Window & Layout" => "窗口与布局",
        "Panels" => "面板",
        "Debugger" => "调试器",
        "Terminal" => "终端",
        "Version Control" => "版本控制",
        "Collaboration" => "协作",
        "AI" => "AI",
        "Network" => "网络",
        "Developer" => "开发者",

        // Settings section headers
        "Advanced Settings" => "高级设置",
        "Agent Configuration" => "智能体配置",
        "Agent Panel Font" => "智能体面板字体",
        "Autoclose" => "自动闭合",
        "Auto Save" => "自动保存",
        "Auto Update" => "自动更新",
        "Base Keymap" => "基础快捷键",
        "Behavior Settings" => "行为设置",
        "Branch Picker" => "分支选择器",
        "Buffer Font" => "编辑器字体",
        "Calls" => "通话",
        "Collaboration Panel" => "协作面板",
        "Command Palette" => "命令面板",
        "Completions" => "补全",
        "Cursor" => "光标",
        "Debuggers" => "调试器",
        "Display Settings" => "显示设置",
        "Drag And Drop Selection" => "拖放选择",
        "Edit Predictions" => "编辑预测",
        "Environment" => "环境",
        "Feature Flags" => "功能开关",
        "File Finder" => "文件查找器",
        "File Scan" => "文件扫描",
        "File Types" => "文件类型",
        "Font" => "字体",
        "Formatting" => "格式化",
        "General Settings" => "通用设置",
        "Language" => "语言",
        "The language used by the user interface." => "用户界面使用的语言。",
        "Git Blame View" => "Git 追溯视图",
        "Git Commit View" => "Git 提交视图",
        "Git Graph" => "Git 图表",
        "Git Gutter" => "Git 边栏",
        "Git Hunks" => "Git 代码块",
        "Git Integration" => "Git 集成",
        "Guides" => "参考线",
        "Gutter" => "边栏",
        "Highlighting" => "高亮",
        "Hover Popover" => "悬停弹窗",
        "Indentation" => "缩进",
        "Indent Guides" => "缩进参考线",
        "Inlay Hints" => "内联提示",
        "Inline Diagnostics" => "内联诊断",
        "Inline Git Blame" => "内联 Git 追溯",
        "Instrumentation" => "性能监测",
        "Keybindings" => "键位绑定",
        "Languages" => "语言",
        "Layout" => "布局",
        "Layout Settings" => "布局设置",
        "LSP Completions" => "LSP 补全",
        "LSP Highlights" => "LSP 高亮",
        "LSP Pull Diagnostics" => "LSP 拉取诊断",
        "Markdown Preview Font" => "Markdown 预览字体",
        "Mermaid Font" => "Mermaid 字体",
        "Minimap" => "缩略图",
        "Miscellaneous" => "杂项",
        "Modal Editing" => "模态编辑",
        "Multibuffer" => "多缓冲区",
        "Pane Modifiers" => "面板修饰键",
        "Pane Split Direction" => "面板拆分方向",
        "Preview Tabs" => "预览标签页",
        "Privacy" => "隐私",
        "Scoped Settings" => "作用域设置",
        "Scrollbar" => "滚动条",
        "Scrolling" => "滚动",
        "Search" => "搜索",
        "Security" => "安全",
        "Signature Help" => "签名帮助",
        "Stash Picker" => "暂存选择器",
        "Status Bar" => "状态栏",
        "Tab Bar" => "标签栏",
        "Tab Settings" => "标签页设置",
        "Tasks" => "任务",
        "Text Rendering" => "文本渲染",
        "Theme" => "主题",
        "Title Bar" => "标题栏",
        "Toolbar" => "工具栏",
        "UI Font" => "界面字体",
        "Which-key Menu" => "Which-key 菜单",
        "Whitespace" => "空白字符",
        "Workspace Restoration" => "工作区恢复",
        "Wrapping" => "自动换行",

        // Onboarding
        "Light" => "浅色",
        "Dark" => "深色",
        "System" => "跟随系统",
        "Import Settings" => "导入设置",
        "Automatically pull your settings from other editors" => "自动从其他编辑器迁移设置",
        "Install" => "安装",
        "Agent Setup" => "智能体设置",
        "Install your favorite agents and start your first thread." => {
            "安装你常用的智能体并开始第一个对话。"
        }
        "Vim Mode" => "Vim 模式",
        "Coming from Neovim? Use our first-class implementation of Vim Mode" => {
            "从 Neovim 过来？试试我们原生支持的 Vim 模式"
        }
        "Trust All Projects By Default" => "默认信任所有项目",
        "Automatically mark all new projects as trusted to unlock all Zed's features" => {
            "自动将所有新项目标记为受信任，以启用所有功能"
        }
        "Kon can only allow services like language servers, project settings, and MCP servers to run after you mark a new project as trusted." => {
            "只有将新项目标记为受信任后，Kon 才允许语言服务器、项目设置和 MCP 服务器等服务运行。"
        }
        "Finish Setup" => "完成设置",

        // Title bar and status messages
        "Disconnected" => "已断开",
        "Updating..." => "正在更新...",
        "Please restart Kon to Collaborate" => "请重启 Kon 以使用协作",
        "Please update Kon to Collaborate" => "请更新 Kon 以使用协作",
        "Restart to update Kon" => "重启以更新 Kon",
        "Remote Project" => "远程项目",
        "Dev Container" => "开发容器",
        "Mock Remote Project" => "模拟远程项目",
        "You're in Restricted Mode" => "你处于受限模式",
        "Mark this project as trusted and unlock all features" => {
            "将此项目标记为受信任并解锁所有功能"
        }
        "Click to Follow" => "点击以跟随",
        "Worktree" => "工作树",
        "Create Branch" => "创建分支",
        "User menu" => "用户菜单",
        "Organization" => "组织",
        "Panel Layout" => "面板布局",
        "Classic" => "经典",
        "Agentic" => "智能体",
        "Sign Out" => "退出登录",
        "Connecting to: {}" => "正在连接到：{}",
        "Connected to: {}" => "已连接到：{}",
        "Connection attempt to {} missed. Retrying..." => {
            "与 {} 的连接尝试超时。正在重试..."
        }
        "Lost connection to {}. Reconnecting..." => "与 {} 的连接已断开。正在重新连接...",
        "Disconnected from {}" => "已断开与 {} 的连接",
        "Application menu" => "应用菜单",
        "Open Application Menu" => "打开应用菜单",
        "Leave Call" => "离开通话",

        "None" => "无",
        "On" => "开",
        "Off" => "关",
        "Kon (Default)" => "Kon（默认）",
        "Open Recent Project" => "打开最近项目",
        "Onboarding" => "引导",
        "Today" => "今天",
        "Yesterday" => "昨天",
        "This Week" => "本周",
        "Past Week" => "上周",
        "Older" => "更早",
        "Search all threads…" => "搜索所有会话...",
        "Cancel Restore" => "取消恢复",
        "Delete Thread" => "删除会话",
        "Archive Thread" => "归档会话",
        "Archived Thread Opened" => "已打开归档会话",
        "Clear Search" => "清除搜索",
        "Start New Agent Thread" => "新建智能体会话",
        "Import Threads" => "导入会话",
        "Show All Threads" => "显示所有会话",
        "Show Only Archived Threads" => "仅显示归档会话",
        "No threads match your search." => "没有匹配的会话。",
        "No threads yet." => "还没有会话。",
        "This Window" => "当前窗口",
        "No recent projects found" => "未找到最近项目",
        "No matches" => "无匹配项",
        "Choose from Local Folders" => "从本地文件夹选择",
        "Select" => "选择",
        "1 thread" => "1 个会话",
        "{} threads" => "{} 个会话",
        "Associate the \"{}\" thread with..." => "将会话“{}”关联到...",
        // Project panel
        "Open Markdown Preview" => "打开 Markdown 预览",
        "Open SVG Preview" => "打开 SVG 预览",
        "Search Inside" => "在文件夹中搜索",
        "New Folder" => "新建文件夹",
        "Open in Default App" => "在默认应用中打开",
        "Find in Folder…" => "在文件夹中查找...",
        "Unfold Directory" => "展开目录",
        "Fold Directory" => "折叠目录",
        "Compare Marked Files" => "比较已标记的文件",
        "Duplicate" => "创建副本",
        "Copy Relative Path" => "复制相对路径",
        "Download..." => "下载...",
        "Restore File" => "还原文件",
        "Add to .gitignore" => "添加到 .gitignore",
        "Add to .git/info/exclude" => "添加到 .git/info/exclude",
        "View History" => "查看历史",
        "Open File Permalink" => "打开文件永久链接",
        "Copy File Permalink" => "复制文件永久链接",
        "Trash" => "移到回收站",
        "Remove from Project" => "从项目中移除",
        "Expand All" => "全部展开",
        "Collapse All" => "全部折叠",
        "Add Folders to Project…" => "将文件夹添加到项目...",
        "File or directory name cannot be empty." => "文件或目录名称不能为空。",
        "File or directory name contains leading or trailing whitespace." => {
            "文件或目录名称包含前导或尾随空白。"
        }
        "Do you want to trash {}?" => "要将 {} 移到回收站吗？",
        "Do you want to trash the following {} files?\n{}" => {
            "要将以下 {} 个文件移到回收站吗？\n{}"
        }
        "Are you sure you want to permanently delete {}?" => "确定要永久删除 {} 吗？",
        "Are you sure you want to permanently delete the following {} files?\n{}" => {
            "确定要永久删除以下 {} 个文件吗？\n{}"
        }
        "This cannot be undone." => "此操作无法撤销。",
        ".. 1 file not shown" => ".. 还有 1 个文件未显示",
        ".. {} files not shown" => ".. 还有 {} 个文件未显示",
        "\n\nIt has unsaved changes, which will be lost." => {
            "\n\n它有未保存的更改，这些更改将丢失。"
        }
        "\n\n1 of these has unsaved changes, which will be lost." => {
            "\n\n其中 1 个有未保存的更改，这些更改将丢失。"
        }
        "\n\n{} of these have unsaved changes, which will be lost." => {
            "\n\n其中有 {} 个有未保存的更改，这些更改将丢失。"
        }
        "Restore" => "还原",
        "Cancel" => "取消",
        "A file or folder with name {} already exists in the destination folder. Do you want to replace it?" => {
            "目标文件夹中已存在名为 {} 的文件或文件夹。要替换它吗？"
        }
        "Symbolic Link" => "符号链接",
        "{} entries" => "{} 项",

        // Tab bar and pane
        "Unlock Tab" => "解锁标签页",
        "This will make this tab editable" => "这将使此标签页可编辑",
        "Locked Tab" => "已锁定标签页",
        "This tab is read-only" => "此标签页为只读",
        "Read-Only Tab" => "只读标签页",
        "Close" => "关闭",
        "Close Others" => "关闭其他标签页",
        "Close Multibuffers" => "关闭多缓冲区",
        "Close Left" => "关闭左侧标签页",
        "Close Right" => "关闭右侧标签页",
        "Close Clean" => "关闭未修改的标签页",
        "Close All" => "关闭全部",
        "Unpin Tab" => "取消固定标签页",
        "Pin Tab" => "固定标签页",
        "Make Tab Read-Only" => "将标签页设为只读",
        "Make Tab Editable" => "将标签页设为可编辑",
        "Reveal In Project Panel" => "在项目面板中显示",
        "Go Back" => "后退",
        "Go Forward" => "前进",
        "New…" => "新建...",
        "Open File" => "打开文件",
        "Search Project" => "搜索项目",
        "Search Symbols" => "搜索符号",
        "New Terminal" => "新建终端",
        "New Center Terminal" => "在中央新建终端",
        "Split Pane" => "拆分面板",
        "Do you want to save changes to the following files?" => "要保存以下文件的更改吗？",
        "Save all" => "全部保存",
        "Discard all" => "全部放弃",
        "Unable to save file: {}" => "无法保存文件：{}",
        "Close Without Saving" => "不保存并关闭",
        "Overwrite" => "覆盖",
        "Discard Edits" => "放弃编辑",
        "Don't Save" => "不保存",
        "{} contains unsaved edits. Do you want to save it?" => {
            "{} 包含未保存的编辑。要保存吗？"
        }
        "This buffer contains unsaved edits. Do you want to save it?" => {
            "此缓冲区包含未保存的编辑。要保存吗？"
        }
        "This file has changed on disk since you started editing it. Do you want to overwrite it?" => {
            "此文件在你开始编辑后已在磁盘上更改。要覆盖它吗？"
        }
        "This file has been deleted on disk since you started editing it. Do you want to recreate it?" => {
            "此文件在你开始编辑后已在磁盘上删除。要重新创建它吗？"
        }

        // Editor
        "Run to Cursor" => "运行到光标处",
        "Evaluate Selection" => "求值所选内容",
        "Go to Implementation" => "转到实现",
        "Rename Symbol" => "重命名符号",
        "Format Buffer" => "格式化缓冲区",
        "Format Selections" => "格式化所选内容",
        "Show Code Actions" => "显示代码操作",
        "Add to Agent Thread" => "添加到智能体会话",
        "Copy Permalink to Line" => "复制行永久链接",
        "View File History" => "查看文件历史",
        "Edit Log Breakpoint" => "编辑日志断点",
        "Set Log Breakpoint" => "设置日志断点",
        "Edit Condition Breakpoint" => "编辑条件断点",
        "Set Condition Breakpoint" => "设置条件断点",
        "Edit Hit Condition Breakpoint" => "编辑命中条件断点",
        "Set Hit Condition Breakpoint" => "设置命中条件断点",
        "Unset Breakpoint" => "取消断点",
        "Set Breakpoint" => "设置断点",
        "Close Git Blame" => "关闭 Git 追溯",
        "Open Git Blame" => "打开 Git 追溯",
        "Remove Bookmark" => "移除书签",
        "Add Bookmark" => "添加书签",
        "Disable" => "禁用",
        "Enable" => "启用",
        "Clear Run Status" => "清除运行状态",

        // Status bar
        "Status bar" => "状态栏",
        "Open threads sidebar" => "打开会话侧边栏",
        "Open Threads Sidebar" => "打开会话侧边栏",
        "Hide Button" => "隐藏按钮",

        // Settings window
        "Copy Link" => "复制链接",
        "User" => "用户",
        "Project" => "项目",
        "Settings File" => "设置文件",
        "View Other Projects" => "查看其他项目",
        "Search Settings" => "搜索设置",
        "Clear" => "清除",
        "Focus Content" => "聚焦内容区",
        "Focus Navbar" => "聚焦导航栏",
        "Settings Navigation" => "设置导航",
        "Scope" => "作用域",
        "Change Scope" => "更改作用域",
        "No Results" => "无结果",
        "No settings match \"{}\"" => "没有与“{}”匹配的设置",
        "Fix in settings.json" => "在 settings.json 中修复",
        "Failed to load your settings. Some values may be incorrect and changes may be lost." => {
            "加载设置失败。部分值可能不正确，更改可能会丢失。"
        }
        "Your settings are out of date, and need to be updated." => "你的设置已过期，需要更新。",
        "They can be automatically migrated to the latest version." => {
            "它们可以自动迁移到最新版本。"
        }
        "They must be manually migrated to the latest version." => {
            "它们必须手动迁移到最新版本。"
        }
        "Your settings file is out of date, automatic migration failed" => {
            "你的设置文件已过期，自动迁移失败"
        }
        "Restricted Mode" => "受限模式",
        "This project is in restricted mode. Some project settings may not apply." => {
            "此项目处于受限模式。部分项目设置可能不会生效。"
        }
        "Manage Trust" => "管理信任",
        "Kon — Settings" => "Kon — 设置",
        "Agent" => "智能体",

        // Agent panel
        "Edit Thread Title" => "编辑会话标题",
        "No model is configured for summarizing thread titles." => {
            "未配置用于总结会话标题的模型。"
        }
        "Title generation failed. Click to retry." => "标题生成失败。点击重试。",
        "Toggle Agent Menu" => "切换智能体菜单",
        "Current Thread" => "当前会话",
        "Regenerate Thread Title" => "重新生成会话标题",
        "Open Thread as Markdown" => "以 Markdown 打开会话",
        "Add Server…" => "添加服务器...",
        "Install New Servers…" => "安装新服务器...",
        "Context" => "上下文",
        "Open Global Rules" => "打开全局规则",
        "Open Project Rules" => "打开项目规则",
        "Profiles" => "配置档案",
        "Toggle Threads Sidebar" => "切换会话侧边栏",
        "Reauthenticate" => "重新认证",
        "Log Out" => "退出登录",
        "Reload Agent" => "重新加载智能体",
        "Selected Agent" => "已选智能体",
        "Disable Full Screen" => "退出全屏",
        "Enable Full Screen" => "进入全屏",
        "Add More Agents" => "添加更多智能体",
        "No active native thread to copy" => "没有可复制的活动原生会话",
        "Thread copied to clipboard (base64 encoded)" => {
            "会话已复制到剪贴板（base64 编码）"
        }
        "Open a project to load a thread" => "打开项目以加载会话",
        "No clipboard content available" => "剪贴板中没有内容",
        "Clipboard does not contain text" => "剪贴板不包含文本",
        "Failed to decode clipboard content (expected base64)" => {
            "无法解码剪贴板内容（应为 base64）"
        }
        "Failed to parse thread data from clipboard" => "无法从剪贴板解析会话数据",
        "Thread loaded from clipboard" => "已从剪贴板加载会话",
        "No active thread" => "没有活动会话",
        "Thread metadata store not available" => "会话元数据存储不可用",
        "No metadata found for active thread" => "未找到活动会话的元数据",
        "All Sidebar Thread Metadata" => "所有侧边栏会话元数据",

        // Git panel
        "Stage All" => "暂存全部",
        "Unstage All" => "取消暂存全部",
        "Restore All Changes" => "还原所有更改",
        "Stash All" => "贮藏全部",
        "Stash Tracked" => "贮藏已跟踪文件",
        "Stash Staged" => "贮藏已暂存文件",
        "Stash Pop" => "弹出贮藏",
        "View Stash" => "查看贮藏",
        "Discard Tracked Changes" => "放弃已跟踪文件的更改",
        "Trash Untracked Files" => "将未跟踪文件移到回收站",
        "Stage" => "暂存",
        "Unstage" => "取消暂存",
        "Conflicts" => "冲突",
        "Tracked" => "已跟踪",
        "Untracked" => "未跟踪",
        "Staged" => "已暂存",
        "Unstaged" => "未暂存",
        "Branch Changes" => "分支更改",
        "unstaged changes" => "未暂存的更改",
        "changes" => "更改",
        "Are you sure you want to restore {name}?" => "确定要还原 {name} 吗？",
        "Trash {name}?" => "要将 {name} 移到回收站吗？",
        "Are you sure you want to discard {changes} to {name}?" => {
            "确定要放弃 {name} 的{changes}吗？"
        }
        "Discard Changes" => "放弃更改",
        "Discard" => "放弃",
        "Discard {changes} to {count} files?" => "放弃 {count} 个文件中的{changes}？",
        "Trash {count} files?" => "要将 {count} 个文件移到回收站吗？",
        "Discard {changes} to {count} files and trash {trashed} files?" => {
            "放弃 {count} 个文件中的{changes}，并将 {trashed} 个文件移到回收站？"
        }
        "Discard and Trash" => "放弃并移到回收站",
        "Restore Tracked Files" => "还原已跟踪文件",
        "Discard changes to these files?" => "放弃这些文件的更改？",
        "View Options" => "视图选项",
        "Cancel Commit Message Generation" => "取消生成提交信息",
        "Generating Commit…" => "正在生成提交…",
        "No Changes to Commit" => "没有可提交的更改",
        "Generate Commit Message" => "生成提交信息",
        "Amend" => "修补提交",
        "Signoff" => "签署",
        "Skip Hooks" => "跳过钩子",
        "Generating commit message..." => "正在生成提交信息...",
        "You must resolve conflicts before committing" => "必须先解决冲突才能提交",
        "No changes to commit" => "没有可提交的更改",
        "Commit in progress" => "提交进行中",
        "No commit message" => "没有提交信息",
        "You do not have write access to this project" => "你没有此项目的写入权限",
        "Amend Tracked" => "修补已跟踪提交",
        "Commit" => "提交",
        "Commit Tracked" => "提交已跟踪文件",
        "Toggle {} Tab" => "切换{}标签页",
        "Changes" => "更改",
        "No repository found" => "未找到仓库",
        "Failed to load commit history" => "加载提交历史失败",
        "Loading Commit History…" => "正在加载提交历史…",
        "No commits yet" => "还没有提交",
        "View Branch Diff" => "查看分支差异",
        "Trust Directory" => "信任目录",
        "Learn More" => "了解更多",
        "No Git Repositories" => "没有 Git 仓库",
        "Initialize Repository" => "初始化仓库",
        "All conflicts marked as resolved" => "所有冲突已标记为已解决",
        "No staged changes yet" => "还没有已暂存的更改",
        "No unstaged changes" => "没有未暂存的更改",
        "No changes" => "没有更改",
        "Files" => "文件",
        "Unstage Folder" => "取消暂存文件夹",
        "Stage Folder" => "暂存文件夹",
        "Unstage File" => "取消暂存文件",
        "Stage File" => "暂存文件",
        "Unstage {count} {label}" => "取消暂存 {count} 个{label}",
        "Stage {count} {label}" => "暂存 {count} 个{label}",
        "Trash Folder" => "将文件夹移到回收站",
        "Trash File" => "将文件移到回收站",
        "Trash {count} {label}" => "将 {count} 个{label}移到回收站",
        "Restore {count} {label}" => "还原 {count} 个{label}",
        "Discard Changes to {count} {label}" => "放弃 {count} 个{label}的更改",
        "Copy Paths" => "复制路径",
        "Copy Relative Paths" => "复制相对路径",
        "Open Diffs" => "打开差异",
        "Open File Diffs" => "打开文件差异",
        "View Files" => "查看文件",
        "View File" => "查看文件",
        "View File Histories" => "查看文件历史",
        "Open Diff" => "打开差异",
        "Open File Diff" => "打开文件差异",

        // Terminal
        "Failed to spawn terminal" => "无法启动终端",
        "Edit Settings" => "编辑设置",

        // Search
        "Find in Results" => "在结果中查找",
        "Search…" => "搜索...",
        "Replace with…" => "替换为...",
        "Search all files…" => "搜索所有文件...",
        "Replace in project…" => "在项目中替换...",
        "Include: e.g. src/**/*.rs" => "包含：例如 src/**/*.rs",
        "Exclude: e.g. vendor/*, *.lock" => "排除：例如 vendor/*, *.lock",
        "Loading project…" => "正在加载项目…",
        "Searching…" => "正在搜索…",
        "Search All Files" => "搜索所有文件",
        "No results found in this project for the provided query" => {
            "在此项目中未找到与查询匹配的结果"
        }
        "Start typing to search. For more options:" => "开始输入即可搜索。更多选项：",
        "Hit enter to search. For more options:" => "按回车开始搜索。更多选项：",
        "Include/exclude specific paths" => "包含/排除特定路径",
        "Find and replace" => "查找并替换",
        "Match with regex" => "使用正则表达式匹配",
        "Match case" => "区分大小写",
        "Match whole words" => "全字匹配",

        // File finder
        "Search project files..." => "搜索项目文件...",
        "Exclude Ignored Files" => "排除已忽略的文件",
        "Include Ignored Files" => "包含已忽略的文件",
        "Create file " => "创建文件 ",
        "Open multiple" => "打开多个",
        "Split…" => "拆分...",

        // Extensions and themes
        "Search extensions..." => "搜索扩展...",
        "Retry" => "重试",
        "Uninstall" => "卸载",
        "Upgrade" => "升级",
        "Rebuild" => "重新构建",
        "View Documentation" => "查看文档",
        "Install Dev Extension" => "安装开发扩展",
        "View Theme Docs" => "查看主题文档",
        "View Icon Theme Docs" => "查看图标主题文档",
        "No dev extensions found" => "未找到开发扩展",
        "Copy Extension ID" => "复制扩展 ID",
        "Copy Author Info" => "复制作者信息",
        "Install Another Version..." => "安装其他版本...",
        "Enable Vim mode" => "启用 Vim 模式",

        // Agent conversation
        "Message the {}, @ to include context, / for commands" => {
            "向 {} 发送消息，@ 添加上下文，/ 使用命令"
        }
        "Message {} — @ to include context, / for commands" => {
            "向 {} 发送消息 — @ 添加上下文，/ 使用命令"
        }
        "Message {} — @ to include context" => "向 {} 发送消息 — @ 添加上下文",
        "Choose one of the following authentication options:" => "请选择以下身份验证方式之一：",
        "Try to restart the agent" => "尝试重新启动智能体",
        "Discard Saved Submissions" => "丢弃已保存的提交内容",
        "Discard local copies only; does not cancel agent work" => {
            "仅丢弃本地副本，不会取消智能体正在进行的工作"
        }

        // Restricted mode
        "Unrecognized Project" => "未识别的项目",
        "Unrecognized Projects ({})" => "未识别的项目（{}）",
        "Untrusted projects are opened in Restricted Mode to protect your system." => {
            "不受信任的项目会以受限模式打开，以保护你的系统。"
        }
        "Review .zed/settings.json for any extensions or commands configured by this project." => {
            "请检查 .zed/settings.json 中该项目配置的扩展或命令。"
        }
        "Restricted Mode prevents:" => "受限模式会阻止：",
        "Project settings from being applied" => "应用项目设置",
        "Language servers from running" => "运行语言服务器",
        "MCP Server integrations from installing" => "安装 MCP 服务器集成",
        "Trust all projects in" => "信任此位置的所有项目：",
        "Stay in Restricted Mode" => "保持受限模式",
        "Trust and Continue" => "信任并继续",
        "Trust all single files" => "信任所有单文件项目",
        "Trust all projects in the {} folder" => "信任 {} 文件夹中的所有项目",
        "Trust all projects in the parent folders" => "信任父文件夹中的所有项目",

        // Agent settings pages
        "Agents connected through the Agent Client Protocol." => {
            "通过 Agent Client Protocol 连接的智能体。"
        }
        "No external agents added yet. Click \"Add Agent\" to get started." => {
            "尚未添加外部智能体。点击“添加智能体”开始。"
        }
        "No active project found. Open a workspace to manage external agents." => {
            "未找到活动项目。打开工作区以管理外部智能体。"
        }
        "Add Agent" => "添加智能体",
        "Add" => "添加",
        "Configured Servers" => "已配置的服务器",
        "Manage servers connected directly or via extensions." => {
            "管理直接连接或通过扩展连接的服务器。"
        }
        "No MCP servers added yet. Click \"Add Server\" to get started." => {
            "尚未添加 MCP 服务器。点击“添加服务器”开始。"
        }
        "No active project found. Open a workspace to manage MCP servers." => {
            "未找到活动项目。打开工作区以管理 MCP 服务器。"
        }
        "Authenticate to connect this server" => "进行身份验证以连接此服务器",
        "Authenticate" => "身份验证",
        "A client secret is required to connect this server" => "连接此服务器需要客户端密钥",
        "Authenticating…" => "正在验证…",
        "Add Server" => "添加服务器",
        "Add Provider" => "添加提供商",
        "Provider" => "提供商",
        "API Key" => "API 密钥",
        "Visit the" => "访问",
        "to generate an API key." => "以生成 API 密钥。",
        "Or set the {} env var and restart Zed for it to take effect." => {
            "或设置 {} 环境变量并重启 Zed 使其生效。"
        }
        "Configure Provider" => "配置提供商",
        "Models" => "模型",
        "Add Model" => "添加模型",
        "Default reasoning effort" => "默认推理强度",
        "Save Provider" => "保存提供商",
        "Add skill content…" => "添加技能内容…",
        "Import from URL" => "从 URL 导入",
        "(optional)" => "（可选）",
        "Paste a GitHub .md URL to fetch it and fill out the form. For private files, Zed retries using GITHUB_TOKEN, if set." => {
            "粘贴 GitHub 上的 .md 链接以获取内容并自动填写表单。对于私有文件，如果设置了 GITHUB_TOKEN，Zed 会使用它重试。"
        }
        "Fetching and parsing…" => "正在获取并解析…",
        "Front-matter" => "前置元数据",
        "Skill Content" => "技能内容",
        "Learn more about sandboxing" => "了解有关沙盒的更多信息",
        "Dismiss" => "忽略",
        "Nothing configured" => "未配置任何内容",

        // Notifications
        "Copy Description" => "复制描述",
        "Suppress" => "不再显示",
        "Click to close" => "点击关闭",
        "Suppress with shift-click" => "按住 Shift 点击不再显示",
        "More Options" => "更多选项",
        "Copy Message" => "复制消息",
        "Click to Close" => "点击关闭",
        "Shift-click to Suppress" => "按住 Shift 点击不再显示",

        // Recent projects and remote connections
        "Open Local Folders" => "打开本地文件夹",
        "Open Remote Folder" => "打开远程文件夹",
        "Remove Folder" => "移除文件夹",
        "Remove from Window" => "从窗口中移除",
        "Remove" => "移除",
        "Activate" => "激活",
        "Search projects…" => "搜索项目…",
        "Select Dev Container Configuration" => "选择开发容器配置",
        "Search remote projects…" => "搜索远程项目…",
        "Delete Remote Project" => "删除远程项目",
        "Reconnect" => "重新连接",
        "WSL:" => "WSL：",
        "Error Creating Dev Container:" => "创建开发容器时出错：",
        "Creating Dev Container" => "正在创建开发容器",
        "Remove Distro" => "移除发行版",
        "Copy Server Address" => "复制服务器地址",
        "Remove Server" => "移除服务器",
        "Add a nickname for this server" => "为此服务器添加昵称",
        "Search project symbols..." => "搜索项目符号...",
        "Search buffer symbols…" => "搜索缓冲区符号…",
        "Toggle Panel With" => "切换面板快捷键",
        "Clear Filter" => "清除过滤",
        "Searching:" => "搜索：",

        // Agent conversation
        "Dismiss Notice" => "关闭提示",
        "Dismiss notice: {}" => "关闭提示：{}",
        "Change Model" => "更改模型",
        "Cycle Favorite Models" => "循环切换常用模型",
        "Search this thread…" => "在此会话中搜索…",
        "Copy into an empty composer. The saved submission is kept until discarded." => {
            "复制到空白输入框中。已保存的提交内容会保留，直到被丢弃。"
        }
        "Review" => "审查",
        "Subagents Awaiting Permission:" => "等待授权的子智能体：",
        "Scroll to Subagent" => "滚动到子智能体",
        "Clear All" => "全部清除",
        "Plan" => "计划",
        "Clear Plan" => "清除计划",
        "Current:" => "当前：",
        "Edits" => "编辑",
        "Stop Subagent" => "停止子智能体",
        "Minimize Subagent" => "最小化子智能体",
        "Make Subagent Full Screen" => "将子智能体设为全屏",
        "Change Thinking Effort" => "更改思考强度",
        "Cycle Thinking Effort" => "循环切换思考强度",
        "Loading Added Context…" => "正在加载已添加的上下文…",
        "Queue and Send" => "排队并发送",
        "Send Immediately" => "立即发送",
        "Input:" => "输入：",
        "Output:" => "输出：",
        "Cost" => "费用",
        "Rules" => "规则",
        "Unavailable Editing" => "无法编辑",
        "Subagent Output" => "子智能体输出",
        "Everything below this line was sent as output from this subagent to the main agent." => {
            "此行以下的所有内容都是该子智能体发送给主智能体的输出。"
        }
        "Scroll to User Message" => "滚动到用户消息",
        "Scroll to Top" => "滚动到顶部",
        "Thanks for your feedback!" => "感谢你的反馈！",
        "Awaiting Confirmation" => "等待确认",
        "Open Session" => "打开会话",
        "Unavailable Session" => "会话不可用",
        "Run Command" => "运行命令",
        "Paused" => "已暂停",
        "Stop Tool Call" => "停止工具调用",
        "Network access" => "网络访问",
        "Write Access" => "写入权限",
        "Runs without the OS sandbox" => "在没有操作系统沙盒的情况下运行",
        "Reason" => "原因",
        "Configure unicode confusables warning" => "配置 Unicode 易混淆字符警告",
        "I understand and wish to proceed" => "我了解并希望继续",
        "This command can write to a file on a Windows drive" => {
            "此命令可以写入 Windows 驱动器上的文件"
        }
        "Configure Windows-drive warning" => "配置 Windows 驱动器警告",
        "Don't show this warning again" => "不再显示此警告",
        "Couldn't create a sandbox" => "无法创建沙盒",
        "Go to File" => "转到文件",
        "No text preview available" => "没有可用的文本预览",
        "Subagent Cancelled" => "子智能体已取消",
        "Subagent Failed" => "子智能体失败",
        "Subagent Status Unknown" => "子智能体状态未知",
        "New Thread" => "新建会话",
        "Thinking" => "思考中",
        "Reject" => "拒绝",
        "Allow" => "允许",
        "Deny" => "拒绝",
        "Accept" => "接受",
        "You have sandboxing disabled in settings." => "你在设置中禁用了沙盒。",
        "Sandboxing is disabled for this thread" => "此会话已禁用沙盒",
        "Sandboxing" => "沙盒",

        // Debugger
        "Start Debug Session" => "启动调试会话",
        "Edit debug.json" => "编辑 debug.json",
        "Open Documentation" => "打开文档",
        "Open Debug Adapter Logs" => "打开调试适配器日志",
        "Close Panel" => "关闭面板",
        "Pause Program" => "暂停程序",
        "Continue Program" => "继续程序",
        "Continue Thread" => "继续线程",
        "Step In" => "单步进入",
        "Rerun Session" => "重新运行会话",
        "Terminate Thread" => "终止线程",
        "Terminate All Threads" => "终止所有线程",
        "Detach" => "分离",
        "Step Back in Session History" => "在会话历史中后退",
        "Current State" => "当前状态",
        "Debug Panel" => "调试器面板",
        "New Session" => "新建会话",
        "Debugger Docs" => "调试器文档",
        "Debugger Extensions" => "调试器扩展",
        "Breakpoints" => "断点",
        "No Breakpoints Set" => "未设置断点",
        "Select a Debug Session" => "选择调试会话",
        "Select the process you want to attach the debugger to" => {
            "选择要附加调试器的进程"
        }
        "Debugger:" => "调试器：",
        "Debug" => "调试",
        "Attach" => "附加",
        "Launch" => "启动",
        "Start" => "开始",

        // Git diff and history views
        "Use Both" => "两者都使用",
        "Toggle Staged" => "切换暂存状态",
        "Go to Previous Hunk" => "转到上一个代码块",
        "Go to Next Hunk" => "转到下一个代码块",
        "Stage and Go to Next Hunk" => "暂存并转到下一个代码块",
        "Unstage and Go to Next Hunk" => "取消暂存并转到下一个代码块",
        "Unstage All Changes" => "取消暂存所有更改",
        "Stage All Changes" => "暂存所有更改",
        "Uncommitted Changes" => "未提交的更改",
        "No uncommitted changes" => "没有未提交的更改",
        "Send Review to Agent ({})" => "将审查发送给智能体（{}）",
        "Send all review comments to the Agent panel" => "将所有审查评论发送到智能体面板",
        "Search commits…" => "搜索提交…",
        "Custom Commands" => "自定义命令",
        "Columns" => "列",
        "Copy Ref Name" => "复制引用名称",
        "View Changes" => "查看更改",
        "Toggle Folder" => "展开/折叠文件夹",
        "Copy Tag" => "复制标签",
        "Message" => "消息",
        "Refs" => "引用",
        "Graph" => "图表",
        "Description" => "描述",
        "Date" => "日期",
        "Author" => "作者",
        "Match Case Sensitivity" => "匹配大小写",
        "Select Previous Match" => "选择上一个匹配项",
        "Select Next Match" => "选择下一个匹配项",
        "Show Flat View" => "显示平铺视图",
        "Show Tree View" => "显示树形视图",
        "Close Details Panel" => "关闭详情面板",
        "Show Commit Details" => "显示提交详情",
        "Hide Commit Details" => "隐藏提交详情",
        "Email Copied!" => "邮箱已复制！",
        "Copy Email" => "复制邮箱",
        "Commit SHA Copied!" => "提交 SHA 已复制！",
        "Copy Commit SHA" => "复制提交 SHA",
        "Open In Browser" => "在浏览器中打开",
        "View Commit" => "查看提交",
        "Hide Commit Message" => "隐藏提交信息",
        "Show Commit Message" => "显示提交信息",
        "Unknown" => "未知",

        // Editor chrome and workspace dialogs
        "Show Symbol Outline" => "显示符号大纲",
        "Right-Click to Copy Path" => "右键复制路径",
        "Could not open file" => "无法打开文件",
        "Do you want to leave the current call?" => "要离开当前通话吗？",
        "Close window and hang up" => "关闭窗口并挂断",
        "Do you want to save all changes in the following files?" => {
            "要保存以下文件中的所有更改吗？"
        }
        "Are you sure you want to restart?" => "确定要重启吗？",
        "Restart" => "重启",
        "Failed to load the database file." => "加载数据库文件失败。",
        "Cannot drop files on a remote project" => "无法将文件拖放到远程项目",
        "Cannot drop local files on a remote SSH/Docker project" => {
            "无法将本地文件拖放到远程 SSH/Docker 项目"
        }
        "Could not translate the dropped paths into WSL paths" => {
            "无法将拖放的路径转换为 WSL 路径"
        }
        "Failed to join project" => "加入项目失败",
        "Paste Text" => "粘贴文本",
        "Inline Assist" => "内联助手",
        "Match Whole Words" => "全字匹配",
        "Also search files ignored by configuration" => "同时搜索被配置忽略的文件",
        "Use Regular Expressions" => "使用正则表达式",
        "One Match Per Line" => "每行一个匹配项",
        "Search Backwards" => "向后搜索",
        "Toggle Filters" => "切换过滤器",
        "Toggle Replace" => "切换替换",
        "Expand All Search Results" => "展开所有搜索结果",
        "Collapse All Search Results" => "折叠所有搜索结果",
        "Replace Next Match" => "替换下一个匹配项",
        "Replace All Matches" => "替换所有匹配项",
        "Only Search Open Files" => "仅搜索打开的文件",
        "Search agents..." => "搜索智能体...",
        "Missing registry entry." => "缺少注册表条目。",
        "Not supported on this platform" => "此平台不支持",
        "Customize" => "自定义",
        "Agent Profiles" => "智能体配置档案",
        "Custom Profiles" => "自定义配置档案",
        "Add New Profile" => "添加新配置档案",
        "Fork Profile" => "派生配置档案",
        "Configure Default Model" => "配置默认模型",
        "Configure Built-in Tools" => "配置内置工具",
        "Configure MCP Tools" => "配置 MCP 工具",
        "Delete Profile" => "删除配置档案",

        // Extension details and suggestions
        "Themes" => "主题",
        "Icon Themes" => "图标主题",
        "Grammars" => "语法",
        "Agent Servers" => "智能体服务器",
        "Slash Commands" => "斜杠命令",
        "Indexed Docs Providers" => "索引文档提供商",
        "Snippets" => "代码片段",
        "Debug Adapters" => "调试适配器",
        "Downloads: {}" => "下载量：{}",
        "Visit Extension Repository" => "访问扩展仓库",
        "Overridden by dev extension." => "已被开发扩展覆盖。",
        "Select extension version..." => "选择扩展版本...",
        "Incompatible" => "不兼容",
        "Learn more" => "了解更多",
        "Don't show again" => "不再显示",
        "Do you want to install the recommended '{}' extension for '{}' files?" => {
            "要安装推荐的“{}”扩展以支持“{}”文件吗？"
        }
        "Yes, install extension" => "是，安装扩展",
        "No, don't install it" => "否，不安装",
        "Disable Suggestions" => "禁用建议",

        // MCP server configuration modal
        "Enter client secret (leave empty for public clients)" => {
            "输入客户端密钥（公共客户端可留空）"
        }
        "Configure MCP Server" => "配置 MCP 服务器",
        "Configure {}" => "配置 {}",
        "Check the server docs for required arguments and environment variables." => {
            "查看服务器文档以了解必需的参数和环境变量。"
        }
        "Open Repository" => "打开仓库",
        "Enter your OAuth client secret, or leave empty for public clients" => {
            "输入你的 OAuth 客户端密钥，公共客户端可留空"
        }
        "Submit" => "提交",
        "Connecting Server…" => "正在连接服务器…",

        _ => return None,
    })
}
