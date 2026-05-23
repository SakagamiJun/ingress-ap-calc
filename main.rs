use std::collections::HashMap;
use std::env;
use std::io;

// --- 颜色常量 ---
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_GRAY: &str = "\x1b[90m";

// --- 1. 国际化基础结构 ---

#[derive(Clone, Copy, PartialEq)]
enum Lang {
    CN,
    EN,
}

// 动作的唯一标识符
#[derive(Clone, Copy, Debug, PartialEq)]
enum ActionKey {
    Scan,
    CreateField,
    Capture,
    DestroyField,
    Deploy8th,
    CreateLink,
    DestroyLink,
    DeployMod,
    DeployRes,
    HackEnemy,
    DestroyRes,
    UpgradeRes,
}

// 界面文本 Key
enum UiKey {
    Title,
    HeaderID,
    HeaderName,
    HeaderBase,
    HeaderState,
    HeaderMult,
    PromptLine1,
    PromptLine2,
    PromptLine3,
    PromptLine4,
    PromptLine5,
    PromptLine6,
    PromptLine7,
    PromptLine8,
    PromptLine9,
    PromptLineQ,
    InputArrow,
    MsgGoalReached,
    MsgDiff,
    ErrNoActions,
    ErrInvalidId,
    ErrInvalidNum,
    ErrInvalidCmd,
    StatusOn,
    StatusOff,
    ResultSuccess,
    ResultFail,
    ResultListPrefix,
}

// --- 2. 文本提供器 ---

struct TextProvider;

impl TextProvider {
    fn action_name(key: ActionKey, lang: Lang) -> &'static str {
        match (lang, key) {
            (Lang::CN, ActionKey::Scan) => "扫描 Portal (Scan)",
            (Lang::EN, ActionKey::Scan) => "Scan Portal",
            (Lang::CN, ActionKey::CreateField) => "建立 Control Field",
            (Lang::EN, ActionKey::CreateField) => "Create Control Field",
            (Lang::CN, ActionKey::Capture) => "占领中立 Portal",
            (Lang::EN, ActionKey::Capture) => "Capture Neutral Portal",
            (Lang::CN, ActionKey::DestroyField) => "摧毁 Control Field",
            (Lang::EN, ActionKey::DestroyField) => "Destroy Control Field",
            (Lang::CN, ActionKey::Deploy8th) => "部署第8个脚 (8th Res)",
            (Lang::EN, ActionKey::Deploy8th) => "Deploy 8th Resonator",
            (Lang::CN, ActionKey::CreateLink) => "建立 Link",
            (Lang::EN, ActionKey::CreateLink) => "Create Link",
            (Lang::CN, ActionKey::DestroyLink) => "摧毁 Link",
            (Lang::EN, ActionKey::DestroyLink) => "Destroy Link",
            (Lang::CN, ActionKey::DeployMod) => "部署/安装 Mod",
            (Lang::EN, ActionKey::DeployMod) => "Deploy Mod",
            (Lang::CN, ActionKey::DeployRes) => "部署 Resonator",
            (Lang::EN, ActionKey::DeployRes) => "Deploy Resonator",
            (Lang::CN, ActionKey::HackEnemy) => "入侵敌方 Portal",
            (Lang::EN, ActionKey::HackEnemy) => "Hack Enemy Portal",
            (Lang::CN, ActionKey::DestroyRes) => "摧毁 Resonator",
            (Lang::EN, ActionKey::DestroyRes) => "Destroy Resonator",
            (Lang::CN, ActionKey::UpgradeRes) => "升级 Resonator",
            (Lang::EN, ActionKey::UpgradeRes) => "Upgrade Resonator",
        }
    }

    fn ui(key: UiKey, lang: Lang) -> &'static str {
        match (lang, key) {
            (Lang::CN, UiKey::Title) => "Ingress 40,000,000 AP 计算器 (Ultimate Enhanced)",
            (Lang::EN, UiKey::Title) => "Ingress 40M AP Calculator (Ultimate Enhanced)",

            (Lang::CN, UiKey::HeaderID) => "ID",
            (Lang::EN, UiKey::HeaderID) => "ID",
            (Lang::CN, UiKey::HeaderName) => "动作名称",
            (Lang::EN, UiKey::HeaderName) => "Action Name",
            (Lang::CN, UiKey::HeaderBase) => "基础/组合 AP",
            (Lang::EN, UiKey::HeaderBase) => "Base/Combo AP",
            (Lang::CN, UiKey::HeaderState) => "状态",
            (Lang::EN, UiKey::HeaderState) => "State",
            (Lang::CN, UiKey::HeaderMult) => "倍率",
            (Lang::EN, UiKey::HeaderMult) => "Mult",

            (Lang::CN, UiKey::PromptLine1) => "- [数值] (e.g. 39990000)      : 设置当前 AP 并计算",
            (Lang::EN, UiKey::PromptLine1) => "- [Number] (e.g. 39990000)    : Set AP and calc",
            (Lang::CN, UiKey::PromptLine2) => {
                "- [+/-]  (e.g. +500)          : 基于上次 AP ({}) 计算"
            }
            (Lang::EN, UiKey::PromptLine2) => {
                "- [+/-]  (e.g. +500)          : Calc based on last AP ({})"
            }
            (Lang::CN, UiKey::PromptLine3) => "- t <ID> (e.g. t 1)           : 开关某个动作",
            (Lang::EN, UiKey::PromptLine3) => "- t <ID> (e.g. t 1)           : Toggle action",
            (Lang::CN, UiKey::PromptLine4) => {
                "- a <N> (e.g. a 2)            : 设置全局Apex倍率 (1为不使用)"
            }
            (Lang::EN, UiKey::PromptLine4) => {
                "- a <N> (e.g. a 2)            : Set global Apex multiplier (1=off)"
            }
            (Lang::CN, UiKey::PromptLine5) => {
                "- lang <en/cn>                : 切换语言 (Switch Language)"
            }
            (Lang::EN, UiKey::PromptLine5) => "- lang <en/cn>                : Switch Language",
            (Lang::CN, UiKey::PromptLine6) => {
                "- ratio <F> <L> (当前: {})    : 设置 Field:Link 比例"
            }
            (Lang::EN, UiKey::PromptLine6) => {
                "- ratio <F> <L> (Curr: {})    : Set Field:Link ratio"
            }
            (Lang::CN, UiKey::PromptLine7) => {
                "- g <N> (e.g. g 2)            : 设置全局倍率 (活动倍率)"
            }
            (Lang::EN, UiKey::PromptLine7) => {
                "- g <N> (e.g. g 2)            : Set global multiplier (event)"
            }
            (Lang::CN, UiKey::PromptLine8) => {
                "- target <N> (e.g. target 50000000) : 设置目标AP值"
            }
            (Lang::EN, UiKey::PromptLine8) => {
                "- target <N> (e.g. target 50000000) : Set target AP value"
            }
            (Lang::CN, UiKey::PromptLine9) => {
                "- priority <ID> (e.g. priority 1)   : 设置优先动作ID"
            }
            (Lang::EN, UiKey::PromptLine9) => {
                "- priority <ID> (e.g. priority 1)   : Set priority action ID"
            }
            (Lang::CN, UiKey::PromptLineQ) => "- q                           : 退出",
            (Lang::EN, UiKey::PromptLineQ) => "- q                           : Quit",

            (Lang::CN, UiKey::InputArrow) => "请输入指令或 AP > ",
            (Lang::EN, UiKey::InputArrow) => "Enter command or AP > ",

            (Lang::CN, UiKey::MsgGoalReached) => "当前 AP: {}。已达成目标！",
            (Lang::EN, UiKey::MsgGoalReached) => "Current AP: {}. Target reached!",
            (Lang::CN, UiKey::MsgDiff) => "当前 AP: {} (距离目标还差: {}{}{})",
            (Lang::EN, UiKey::MsgDiff) => "Current AP: {} (Diff to target: {}{}{})",

            (Lang::CN, UiKey::ErrNoActions) => "[错误] 没有启用的动作！",
            (Lang::EN, UiKey::ErrNoActions) => "[Error] No enabled actions!",
            (Lang::CN, UiKey::ErrInvalidId) => "无效的 ID",
            (Lang::EN, UiKey::ErrInvalidId) => "Invalid ID",
            (Lang::CN, UiKey::ErrInvalidNum) => "无效的数字格式",
            (Lang::EN, UiKey::ErrInvalidNum) => "Invalid number format",
            (Lang::CN, UiKey::ErrInvalidCmd) => "指令格式错误",
            (Lang::EN, UiKey::ErrInvalidCmd) => "Invalid command format",

            (Lang::CN, UiKey::StatusOn) => "[开]",
            (Lang::EN, UiKey::StatusOn) => "[ON]",
            (Lang::CN, UiKey::StatusOff) => "[关]",
            (Lang::EN, UiKey::StatusOff) => "[OFF]",

            (Lang::CN, UiKey::ResultSuccess) => "[成功] 方案如下 (剩余 {}{}{} AP 的优化):",
            (Lang::EN, UiKey::ResultSuccess) => {
                "[Success] Solution found (Optimizing last {}{}{} AP):"
            }
            (Lang::CN, UiKey::ResultFail) => "[注意] 无法精确到达。最近的 10 个 AP 节点：",
            (Lang::EN, UiKey::ResultFail) => "[Note] Exact match failed. Closest 10 AP nodes:",
            (Lang::CN, UiKey::ResultListPrefix) => "次",
            (Lang::EN, UiKey::ResultListPrefix) => "times",
        }
    }
}

// --- 3. 核心数据结构 ---

#[derive(Clone, Debug)]
struct ActionConfig {
    id: usize,
    key: ActionKey,
    base_ap: i32,
    enabled: bool,
    global_multiplier: u32, // 全局倍率 (活动倍率)
    apex_multiplier: u32,   // Apex道具倍率 (可选)
}

#[derive(Clone, Debug)]
struct SolverAction {
    name: String,
    ap: i32,
    bundle_info: Option<String>,
}

// --- 4. 主程序 ---

fn main() {
    let mut current_lang = detect_system_lang();
    let mut ratio_field: u32 = 1;
    let mut ratio_link: u32 = 2;
    let mut target_ap: i32 = 40_000_000;
    let mut priority_action_id: Option<usize> = None;

    let mut configs = vec![
        ActionConfig {
            id: 1,
            key: ActionKey::Scan,
            base_ap: 500,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 2,
            key: ActionKey::CreateField,
            base_ap: 1250,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 3,
            key: ActionKey::Capture,
            base_ap: 800,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 4,
            key: ActionKey::DestroyField,
            base_ap: 750,
            enabled: false,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 5,
            key: ActionKey::Deploy8th,
            base_ap: 375,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 6,
            key: ActionKey::CreateLink,
            base_ap: 313,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 7,
            key: ActionKey::DestroyLink,
            base_ap: 187,
            enabled: false,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 8,
            key: ActionKey::DeployMod,
            base_ap: 125,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 9,
            key: ActionKey::HackEnemy,
            base_ap: 100,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 10,
            key: ActionKey::DestroyRes,
            base_ap: 75,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 11,
            key: ActionKey::UpgradeRes,
            base_ap: 65,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
        ActionConfig {
            id: 12,
            key: ActionKey::DeployRes,
            base_ap: 125,
            enabled: true,
            global_multiplier: 1,
            apex_multiplier: 1,
        },
    ];

    let mut last_ap: i32 = 0;

    print_header(current_lang);
    print_config_list(&configs, current_lang, ratio_field, ratio_link, priority_action_id);

    loop {
        println!();
        println!("{}", TextProvider::ui(UiKey::PromptLine1, current_lang));
        // 修复2：使用 replacen 解决计算显示重复的问题
        let line2_fmt = TextProvider::ui(UiKey::PromptLine2, current_lang);
        println!("{}", line2_fmt.replacen("{}", &last_ap.to_string(), 1));

        println!("{}", TextProvider::ui(UiKey::PromptLine3, current_lang));
        println!("{}", TextProvider::ui(UiKey::PromptLine4, current_lang));
        println!("{}", TextProvider::ui(UiKey::PromptLine7, current_lang));
        println!("{}", TextProvider::ui(UiKey::PromptLine8, current_lang));
        println!("{}", TextProvider::ui(UiKey::PromptLine9, current_lang));
        println!("{}", TextProvider::ui(UiKey::PromptLine5, current_lang));

        let line6_fmt = TextProvider::ui(UiKey::PromptLine6, current_lang);
        let ratio_str = format!("{}:{}", ratio_field, ratio_link);
        println!("{}", line6_fmt.replacen("{}", &ratio_str, 1));

        println!("{}", TextProvider::ui(UiKey::PromptLineQ, current_lang));

        print!("{}", TextProvider::ui(UiKey::InputArrow, current_lang));
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Read error");
        let input = input.trim();

        if input.is_empty() {
            continue;
        }
        if input == "q" {
            break;
        }

        let lower_input = input.to_lowercase();

        if lower_input.starts_with("ratio ") {
            let clean_input = lower_input.replace(":", " ");
            let parts: Vec<&str> = clean_input.split_whitespace().collect();
            if parts.len() >= 3 {
                if let (Ok(f), Ok(l)) = (parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                    if f > 0 && l > 0 {
                        ratio_field = f;
                        ratio_link = l;
                        print_config_list(&configs, current_lang, ratio_field, ratio_link, priority_action_id);
                    }
                }
            } else if parts.len() >= 2 {
                if let Ok(l) = parts[1].parse::<u32>() {
                    if l > 0 {
                        ratio_field = 1;
                        ratio_link = l;
                        print_config_list(&configs, current_lang, ratio_field, ratio_link, priority_action_id);
                    }
                }
            }
            continue;
        }

        if lower_input.starts_with("t ")
            || lower_input.starts_with("a ")
            || lower_input.starts_with("g ")
        {
            handle_config_command(input, &mut configs, current_lang);
            print_config_list(&configs, current_lang, ratio_field, ratio_link, priority_action_id);
            continue;
        }

        if lower_input.starts_with("lang ") {
            let parts: Vec<&str> = lower_input.split_whitespace().collect();
            if parts.len() >= 2 {
                if parts[1] == "cn" || parts[1] == "zh" {
                    current_lang = Lang::CN;
                } else if parts[1] == "en" {
                    current_lang = Lang::EN;
                }
                print_header(current_lang);
                print_config_list(&configs, current_lang, ratio_field, ratio_link, priority_action_id);
            }
            continue;
        }

        if lower_input.starts_with("target ") {
            let parts: Vec<&str> = lower_input.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(new_target) = parts[1].parse::<i32>() {
                    if new_target > 0 {
                        target_ap = new_target;
                        println!("目标AP已设置为: {}", target_ap);
                    } else {
                        println!("目标AP必须大于0");
                    }
                } else {
                    println!("{}", TextProvider::ui(UiKey::ErrInvalidNum, current_lang));
                }
            } else {
                println!("{}", TextProvider::ui(UiKey::ErrInvalidCmd, current_lang));
            }
            continue;
        }

        if lower_input.starts_with("priority ") {
            let parts: Vec<&str> = lower_input.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(action_id) = parts[1].parse::<usize>() {
                    // 检查动作ID是否存在
                    if configs.iter().any(|c| c.id == action_id) {
                        // 如果当前优先动作ID与输入相同，则取消优先；否则设置为新的优先动作
                        if priority_action_id == Some(action_id) {
                            priority_action_id = None;
                            println!("优先动作ID已取消");
                        } else {
                            priority_action_id = Some(action_id);
                            println!("优先动作ID已设置为: {}", action_id);
                        }
                    } else {
                        println!("无效的动作ID: {}", action_id);
                    }
                } else {
                    println!("{}", TextProvider::ui(UiKey::ErrInvalidNum, current_lang));
                }
            } else {
                println!("{}", TextProvider::ui(UiKey::ErrInvalidCmd, current_lang));
            }
            print_config_list(&configs, current_lang, ratio_field, ratio_link, priority_action_id);
            continue;
        }

        let current_ap_opt = parse_ap_input(input, last_ap, current_lang);
        match current_ap_opt {
            Ok(new_ap) => {
                last_ap = new_ap;
                if new_ap >= target_ap {
                    let msg = TextProvider::ui(UiKey::MsgGoalReached, current_lang);
                    println!(
                        "--> {}{}{}",
                        COLOR_GREEN,
                        msg.replacen("{}", &new_ap.to_string(), 1),
                        COLOR_RESET
                    );
                } else {
                    let diff = target_ap - new_ap;
                    let msg = TextProvider::ui(UiKey::MsgDiff, current_lang);
                    // 修复2：使用 replacen 解决计算显示重复的问题
                    let output = msg
                        .replacen("{}", &new_ap.to_string(), 1)
                        .replacen("{}", COLOR_YELLOW, 1)
                        .replacen("{}", &diff.to_string(), 1)
                        .replacen("{}", COLOR_RESET, 1);
                    println!("--> {}", output);

                    let (actions_no_apex, actions_with_apex) =
                        generate_solver_actions(&configs, current_lang, ratio_field, ratio_link);

                    if actions_no_apex.is_empty() {
                        println!(
                            "{}{}{}",
                            COLOR_RED,
                            TextProvider::ui(UiKey::ErrNoActions, current_lang),
                            COLOR_RESET
                        );
                    } else {
                        // 检查是否有Apex倍率
                        let has_apex = configs.iter().any(|c| c.apex_multiplier > 1);

                        if has_apex && !actions_with_apex.is_empty() {
                            // 使用混合方案：部分动作用Apex，部分不用
                            solve_mixed(diff, &actions_no_apex, &actions_with_apex, current_lang, &configs, priority_action_id);
                        } else {
                            // 没有Apex，使用纯普通方案
                            solve_ap(diff, &actions_no_apex, current_lang);
                        }
                    }
                }
            }
            Err(e) => println!("{}[!] {}{}", COLOR_RED, e, COLOR_RESET),
        }
    }
}

// --- 辅助函数 ---

fn detect_system_lang() -> Lang {
    let lang_var = env::var("LANG")
        .unwrap_or_else(|_| "en".to_string())
        .to_lowercase();
    if lang_var.contains("zh") || lang_var.contains("cn") {
        Lang::CN
    } else {
        Lang::EN
    }
}

fn print_header(lang: Lang) {
    println!("========================================");
    println!("{}", TextProvider::ui(UiKey::Title, lang));
    println!("========================================");
}

// [核心修改] 终极对齐方案：全手动填充，拒绝 format! 宏的宽度对齐
fn print_config_list(configs: &Vec<ActionConfig>, lang: Lang, r_field: u32, r_link: u32, priority_action_id: Option<usize>) {
    let link_base = configs
        .iter()
        .find(|c| c.key == ActionKey::CreateLink)
        .map(|c| c.base_ap)
        .unwrap_or(313);
    let d_link_base = configs
        .iter()
        .find(|c| c.key == ActionKey::DestroyLink)
        .map(|c| c.base_ap)
        .unwrap_or(187);

    let h_id = TextProvider::ui(UiKey::HeaderID, lang);
    let h_name = TextProvider::ui(UiKey::HeaderName, lang);
    let h_base = TextProvider::ui(UiKey::HeaderBase, lang);
    let h_state = TextProvider::ui(UiKey::HeaderState, lang);
    let h_mult = TextProvider::ui(UiKey::HeaderMult, lang);

    // 设定每列的视觉宽度常量
    let w_name = 42;
    let w_base = 14;
    let w_state = 8;
    let w_mult = 8;

    // 打印表头 (这里可以用格式化宏，因为表头没有颜色代码)
    println!(
        "\n{:<4} {:<w_name$} {:<w_base$} {:<w_state$} {:<w_mult$}",
        h_id,
        h_name,
        h_base,
        h_state,
        h_mult,
        w_name = w_name,
        w_base = w_base,
        w_state = w_state,
        w_mult = w_mult
    );
    println!("{:-<92}", "-");

    for c in configs {
        let raw_name = TextProvider::action_name(c.key, lang);
        let mut display_name = raw_name.to_string();
        let mut effective_ap = c.base_ap;

        if c.key == ActionKey::CreateField {
            display_name = format!("{} ({}F + {} Link)", raw_name, r_field, r_link);
            effective_ap = (c.base_ap * r_field as i32) + (link_base * r_link as i32);
        } else if c.key == ActionKey::DestroyField {
            display_name = format!("{} ({}F + {} Link)", raw_name, r_field, r_link);
            effective_ap = (c.base_ap * r_field as i32) + (d_link_base * r_link as i32);
        }

        // 如果是优先动作，添加标记
        if Some(c.id) == priority_action_id {
            display_name = format!("{} [优先]", display_name);
        }

        // 1. 处理名称列 (手动填充)
        let name_padded = pad_str_visual(&display_name, w_name);

        // 2. 处理 AP 列 (核心修复：先转换字符串 -> 先填充 -> 后上色)
        let ap_raw = effective_ap.to_string();
        let ap_padded = pad_str_visual(&ap_raw, w_base); // 手动补齐空格到 14
        let ap_colored = if c.enabled {
            ap_padded // 没颜色，直接用
        } else {
            format!("{}{}{}", COLOR_GRAY, ap_padded, COLOR_RESET) // 有颜色，但包裹的是已经补齐的字符串
        };

        // 3. 处理状态列
        let status_raw = if c.enabled {
            TextProvider::ui(UiKey::StatusOn, lang)
        } else {
            TextProvider::ui(UiKey::StatusOff, lang)
        };
        let status_padded = pad_str_visual(status_raw, w_state); // 手动补齐到 8
        let status_colored = if c.enabled {
            format!("{}{}{}", COLOR_GREEN, status_padded, COLOR_RESET)
        } else {
            format!("{}{}{}", COLOR_RED, status_padded, COLOR_RESET)
        };

        // 4. 处理倍率列 (显示全局倍率和Apex倍率)
        let mult_raw = if c.global_multiplier > 1 || c.apex_multiplier > 1 {
            if c.apex_multiplier > 1 {
                format!("G:{} A:{}", c.global_multiplier, c.apex_multiplier)
            } else {
                format!("G:{}", c.global_multiplier)
            }
        } else {
            "--".to_string()
        };
        let mult_padded = pad_str_visual(&mult_raw, w_mult); // 手动补齐到 8
        let mult_colored = if c.global_multiplier > 1 || c.apex_multiplier > 1 {
            format!("{}{}{}", COLOR_YELLOW, mult_padded, COLOR_RESET)
        } else {
            format!("{}{}{}", COLOR_GRAY, mult_padded, COLOR_RESET)
        };

        // 5. 打印
        // 注意：这里不再使用任何 :<14 这样的宽度修饰符，只用 {}
        // 因为我们在上面已经把每一列的内容填充到了正确的视觉宽度
        println!(
            "{:<4} {} {} {} {}",
            c.id, name_padded, ap_colored, status_colored, mult_colored
        );
    }
    println!("{:-<92}", "-");
}

fn pad_str_visual(s: &str, target_width: usize) -> String {
    let mut visual_len = 0;
    for c in s.chars() {
        if c.len_utf8() > 1 {
            visual_len += 2;
        } else {
            visual_len += 1;
        }
    }
    let mut res = s.to_string();
    if visual_len < target_width {
        for _ in 0..(target_width - visual_len) {
            res.push(' ');
        }
    }
    res
}

fn handle_config_command(input: &str, configs: &mut Vec<ActionConfig>, lang: Lang) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 2 {
        println!("{}", TextProvider::ui(UiKey::ErrInvalidCmd, lang));
        return;
    }
    let mode = parts[0];

    // 全局倍率命令 g <N>
    if mode == "g" {
        if let Ok(val) = parts[1].parse::<u32>() {
            if val >= 1 {
                for cfg in configs.iter_mut() {
                    cfg.global_multiplier = val;
                }
            }
        } else {
            println!("{}", TextProvider::ui(UiKey::ErrInvalidNum, lang));
        }
        return;
    }

    // 全局Apex倍率命令 a <N>
    if mode == "a" {
        if let Ok(val) = parts[1].parse::<u32>() {
            if val >= 1 {
                for cfg in configs.iter_mut() {
                    cfg.apex_multiplier = val;
                }
            }
        } else {
            println!("{}", TextProvider::ui(UiKey::ErrInvalidNum, lang));
        }
        return;
    }

    // 单个动作命令 t <ID>
    if let Ok(id) = parts[1].parse::<usize>() {
        if let Some(cfg) = configs.iter_mut().find(|c| c.id == id) {
            if mode == "t" {
                cfg.enabled = !cfg.enabled;
            }
        } else {
            println!("{}", TextProvider::ui(UiKey::ErrInvalidId, lang));
        }
    } else {
        println!("{}", TextProvider::ui(UiKey::ErrInvalidId, lang));
    }
}

fn parse_ap_input(input: &str, last_ap: i32, lang: Lang) -> Result<i32, String> {
    let chars: Vec<char> = input.chars().collect();
    let err_msg = TextProvider::ui(UiKey::ErrInvalidNum, lang).to_string();
    if chars.is_empty() {
        return Err(err_msg);
    }
    if chars[0] == '+' {
        let val = chars[1..]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .map_err(|_| err_msg.clone())?;
        Ok(last_ap + val)
    } else if chars[0] == '-' {
        let val = chars[1..]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .map_err(|_| err_msg.clone())?;
        Ok(last_ap - val)
    } else {
        input.parse::<i32>().map_err(|_| err_msg)
    }
}

// 返回两套方案: (不带Apex的方案, 带Apex的方案)
fn generate_solver_actions(
    configs: &Vec<ActionConfig>,
    lang: Lang,
    r_field: u32,
    r_link: u32,
) -> (Vec<SolverAction>, Vec<SolverAction>) {
    let mut list_no_apex = Vec::new();
    let mut list_with_apex = Vec::new();
    let link_ap = configs
        .iter()
        .find(|c| c.key == ActionKey::CreateLink)
        .map(|c| c.base_ap)
        .unwrap_or(313);
    let d_link_ap = configs
        .iter()
        .find(|c| c.key == ActionKey::DestroyLink)
        .map(|c| c.base_ap)
        .unwrap_or(187);

    for c in configs {
        if !c.enabled {
            continue;
        }
        let raw_name = TextProvider::action_name(c.key, lang);

        if c.key == ActionKey::CreateField {
            let bundle = (c.base_ap * r_field as i32) + (link_ap * r_link as i32);
            // 不带Apex: base * global
            let ap_no_apex = bundle * c.global_multiplier as i32;
            // 带Apex: base * global * apex
            let ap_with_apex = bundle * c.global_multiplier as i32 * c.apex_multiplier as i32;

            list_no_apex.push(SolverAction {
                name: format!("{} ({}F + {}L)", raw_name, r_field, r_link),
                ap: ap_no_apex,
                bundle_info: Some(format!("(Batch: {} Field + {} Link)", r_field, r_link)),
            });
            list_with_apex.push(SolverAction {
                name: format!("{} ({}F + {}L) [Apex]", raw_name, r_field, r_link),
                ap: ap_with_apex,
                bundle_info: Some(format!("(Batch: {} Field + {} Link)", r_field, r_link)),
            });
        } else if c.key == ActionKey::DestroyField {
            let bundle = (c.base_ap * r_field as i32) + (d_link_ap * r_link as i32);
            let ap_no_apex = bundle * c.global_multiplier as i32;
            let ap_with_apex = bundle * c.global_multiplier as i32 * c.apex_multiplier as i32;

            list_no_apex.push(SolverAction {
                name: format!("{} ({}F + {}L)", raw_name, r_field, r_link),
                ap: ap_no_apex,
                bundle_info: Some(format!("(Batch: {} Field + {} Link)", r_field, r_link)),
            });
            list_with_apex.push(SolverAction {
                name: format!("{} ({}F + {}L) [Apex]", raw_name, r_field, r_link),
                ap: ap_with_apex,
                bundle_info: Some(format!("(Batch: {} Field + {} Link)", r_field, r_link)),
            });
        } else {
            let ap_no_apex = c.base_ap * c.global_multiplier as i32;
            let ap_with_apex = c.base_ap * c.global_multiplier as i32 * c.apex_multiplier as i32;

            let name_no_apex = if c.global_multiplier > 1 {
                format!("{} (x{}G)", raw_name, c.global_multiplier)
            } else {
                raw_name.to_string()
            };
            let name_with_apex = if c.apex_multiplier > 1 {
                format!(
                    "{} (x{}G x{}A) [Apex]",
                    raw_name, c.global_multiplier, c.apex_multiplier
                )
            } else {
                format!("{} (x{}G) [Apex]", raw_name, c.global_multiplier)
            };

            list_no_apex.push(SolverAction {
                name: name_no_apex,
                ap: ap_no_apex,
                bundle_info: None,
            });
            list_with_apex.push(SolverAction {
                name: name_with_apex,
                ap: ap_with_apex,
                bundle_info: None,
            });
        }
    }
    list_no_apex.sort_by(|a, b| b.ap.cmp(&a.ap));
    list_with_apex.sort_by(|a, b| b.ap.cmp(&a.ap));
    (list_no_apex, list_with_apex)
}

// 混合方案：优先使用带Apex的动作，然后用不带Apex的动作补足
fn solve_mixed(
    diff: i32,
    actions_no_apex: &Vec<SolverAction>,
    actions_with_apex: &Vec<SolverAction>,
    lang: Lang,
    configs: &Vec<ActionConfig>,
    priority_action_id: Option<usize>,
) {
    let mut recommendation_no_apex: HashMap<String, i32> = HashMap::new();
    let mut recommendation_with_apex: HashMap<String, i32> = HashMap::new();
    let mut remaining = diff;

    // 如果指定了优先动作，则优先使用该动作
    if let Some(priority_id) = priority_action_id {
        // 查找对应的优先动作（在Apex和非Apex列表中）
        if let Some(priority_action) = actions_with_apex.iter().find(|a| {
            // 检查是否是优先动作（通过名称判断）
            let action_name = TextProvider::action_name(get_action_key_from_name(&a.name), lang);
            let config = configs.iter().find(|c| TextProvider::action_name(c.key, lang) == action_name);
            if let Some(config) = config {
                config.id == priority_id
            } else {
                false
            }
        }) {
            let priority_bulk = remaining / priority_action.ap;
            if priority_bulk > 0 {
                recommendation_with_apex.insert(priority_action.name.clone(), priority_bulk);
                remaining -= priority_bulk * priority_action.ap;
            }
        } else if let Some(priority_action) = actions_no_apex.iter().find(|a| {
            // 检查是否是优先动作（通过名称判断）
            let action_name = TextProvider::action_name(get_action_key_from_name(&a.name), lang);
            let config = configs.iter().find(|c| TextProvider::action_name(c.key, lang) == action_name);
            if let Some(config) = config {
                config.id == priority_id
            } else {
                false
            }
        }) {
            let priority_bulk = remaining / priority_action.ap;
            if priority_bulk > 0 {
                recommendation_no_apex.insert(priority_action.name.clone(), priority_bulk);
                remaining -= priority_bulk * priority_action.ap;
            }
        }
    }

    // 修复问题：当初始值为0时，应该一次性决定如何分配Apex和非Apex动作
    // 我们需要一个更智能的策略来避免不必要的分割

    // 首先尝试完全使用Apex动作（如果可能的话）
    if !actions_with_apex.is_empty() {
        if let Some(max_apex) = actions_with_apex.first() {
            let apex_count = remaining / max_apex.ap;
            if apex_count > 0 {
                *recommendation_with_apex.entry(max_apex.name.clone()).or_insert(0) += apex_count;
                remaining -= apex_count * max_apex.ap;
            }
        }
    }

    // 然后使用动态规划来处理剩余的AP
    let max_s = remaining as usize;
    let mut dp = vec![i32::MAX; max_s + 1];
    let mut parent = vec![None; max_s + 1];
    dp[0] = 0;

    // 考虑所有可用的动作（包括Apex和非Apex）
    let all_actions = actions_no_apex.iter().chain(actions_with_apex.iter()).collect::<Vec<_>>();

    for i in 0..=max_s {
        if dp[i] == i32::MAX {
            continue;
        }
        for (idx, action) in all_actions.iter().enumerate() {
            let next_ap = i + action.ap as usize;
            if next_ap <= max_s {
                if dp[next_ap] > dp[i] + 1 {
                    dp[next_ap] = dp[i] + 1;
                    parent[next_ap] = Some((i, idx));
                }
            }
        }
    }

    if dp[max_s] != i32::MAX {
        let msg = TextProvider::ui(UiKey::ResultSuccess, lang);
        println!(
            "\n{}",
            msg.replacen("{}", COLOR_YELLOW, 1)
                .replacen("{}", &diff.to_string(), 1)
                .replacen("{}", COLOR_RESET, 1)
        );

        // 记录剩余AP的方案
        let mut curr = max_s;
        while curr > 0 {
            if let Some((prev, idx)) = parent[curr] {
                let action = all_actions[idx];
                // 检查动作名称是否包含"[Apex]"来确定是否为Apex动作
                if action.name.contains("[Apex]") {
                    *recommendation_with_apex.entry(action.name.clone()).or_insert(0) += 1;
                } else {
                    *recommendation_no_apex.entry(action.name.clone()).or_insert(0) += 1;
                }
                curr = prev;
            } else {
                break;
            }
        }

        // 打印带Apex的动作
        if !recommendation_with_apex.is_empty() {
            println!(
                "\n{}{}=== 使用Apex道具 ==={}{}",
                COLOR_YELLOW, COLOR_CYAN, COLOR_RESET, COLOR_RESET
            );
            print_recommendations_mixed(&recommendation_with_apex, actions_with_apex, lang, true);
        }

        // 打印不带Apex的动作
        if !recommendation_no_apex.is_empty() {
            println!(
                "\n{}{}=== 普通动作（无Apex） ==={}{}",
                COLOR_YELLOW, COLOR_CYAN, COLOR_RESET, COLOR_RESET
            );
            print_recommendations_mixed(&recommendation_no_apex, actions_no_apex, lang, false);
        }

        // 计算总动作次数
        let total_apex: i32 = recommendation_with_apex.values().sum();
        let total_normal: i32 = recommendation_no_apex.values().sum();
        println!(
            "\n{}总计: {} 次Apex动作 + {} 次普通动作{}",
            COLOR_CYAN, total_apex, total_normal, COLOR_RESET
        );
    } else {
        println!("\n{}", TextProvider::ui(UiKey::ResultFail, lang));
        // 尝试找最近的解
        let mut count = 0;
        for i in (0..max_s).rev() {
            if dp[i] != i32::MAX {
                let gap = max_s - i;
                println!(
                    "{}. {} AP (Diff {} AP)",
                    count + 1,
                    40_000_000 - gap as i32,
                    gap
                );
                count += 1;
                if count >= 10 {
                    break;
                }
            }
        }
    }
}

// 旧函数保留，用于纯普通方案计算
fn solve_ap(mut diff: i32, actions: &Vec<SolverAction>, lang: Lang) {
    let mut recommendation = HashMap::new();
    let buffer = 30000;

    if diff > buffer {
        if let Some(max) = actions.first() {
            let bulk = (diff - buffer) / max.ap;
            if bulk > 0 {
                recommendation.insert(max.name.clone(), bulk);
                diff -= bulk * max.ap;
            }
        }
    }

    let max_s = diff as usize;
    let mut dp = vec![i32::MAX; max_s + 1];
    let mut parent = vec![None; max_s + 1];
    dp[0] = 0;

    for i in 0..=max_s {
        if dp[i] == i32::MAX {
            continue;
        }
        for (idx, action) in actions.iter().enumerate() {
            let next_ap = i + action.ap as usize;
            if next_ap <= max_s {
                if dp[next_ap] > dp[i] + 1 {
                    dp[next_ap] = dp[i] + 1;
                    parent[next_ap] = Some((i, idx));
                }
            }
        }
    }

    if dp[max_s] != i32::MAX {
        let msg = TextProvider::ui(UiKey::ResultSuccess, lang);
        println!(
            "\n{}",
            msg.replacen("{}", COLOR_YELLOW, 1)
                .replacen("{}", &diff.to_string(), 1)
                .replacen("{}", COLOR_RESET, 1)
        );

        let mut curr = max_s;
        while curr > 0 {
            if let Some((prev, idx)) = parent[curr] {
                *recommendation.entry(actions[idx].name.clone()).or_insert(0) += 1;
                curr = prev;
            } else {
                break;
            }
        }
        print_recommendations(recommendation, actions, lang);
    } else {
        println!("\n{}", TextProvider::ui(UiKey::ResultFail, lang));
        let mut count = 0;
        for i in (0..max_s).rev() {
            if dp[i] != i32::MAX {
                let gap = max_s - i;
                println!(
                    "{}. {} AP (Diff {} AP)",
                    count + 1,
                    40_000_000 - gap as i32,
                    gap
                );
                count += 1;
                if count >= 10 {
                    break;
                }
            }
        }
    }
}

fn print_recommendations(rec: HashMap<String, i32>, order: &Vec<SolverAction>, lang: Lang) {
    println!("----------------------------------------");
    let suffix = TextProvider::ui(UiKey::ResultListPrefix, lang);
    for action in order {
        if let Some(&count) = rec.get(&action.name) {
            let name_padded = pad_str_visual(&action.name, 35);
            let extra = match &action.bundle_info {
                Some(i) => format!("{}{}", COLOR_GRAY, i),
                None => "".to_string(),
            };
            println!(
                " - {} : x {}{}{} {} {}",
                name_padded, COLOR_CYAN, count, COLOR_RESET, suffix, extra
            );
        }
    }
    println!("----------------------------------------");
}

// 混合方案使用的推荐打印函数
fn print_recommendations_mixed(
    rec: &HashMap<String, i32>,
    order: &Vec<SolverAction>,
    _lang: Lang,
    is_apex: bool,
) {
    let suffix = if is_apex { "次 (Apex)" } else { "次" };
    for action in order {
        if let Some(&count) = rec.get(&action.name) {
            let name_padded = pad_str_visual(&action.name, 35);
            let extra = match &action.bundle_info {
                Some(i) => format!("{}{}", COLOR_GRAY, i),
                None => "".to_string(),
            };
            let color = if is_apex { COLOR_YELLOW } else { COLOR_CYAN };
            println!(
                " - {} : x {}{}{} {} {}",
                name_padded, color, count, COLOR_RESET, suffix, extra
            );
        }
    }
}

// 从动作名称中提取动作键的辅助函数
fn get_action_key_from_name(name: &str) -> ActionKey {
    if name.contains("扫描 Portal") || name.contains("Scan") {
        ActionKey::Scan
    } else if name.contains("建立 Control Field") || name.contains("Create Control Field") {
        ActionKey::CreateField
    } else if name.contains("占领中立 Portal") || name.contains("Capture Neutral") {
        ActionKey::Capture
    } else if name.contains("摧毁 Control Field") || name.contains("Destroy Control Field") {
        ActionKey::DestroyField
    } else if name.contains("部署第8个脚") || name.contains("Deploy 8th Res") {
        ActionKey::Deploy8th
    } else if name.contains("建立 Link") || name.contains("Create Link") {
        ActionKey::CreateLink
    } else if name.contains("摧毁 Link") || name.contains("Destroy Link") {
        ActionKey::DestroyLink
    } else if name.contains("部署/安装 Mod") || name.contains("Deploy Mod") {
        ActionKey::DeployMod
    } else if name.contains("入侵敌方 Portal") || name.contains("Hack Enemy") {
        ActionKey::HackEnemy
    } else if name.contains("摧毁 Resonator") || name.contains("Destroy Resonator") {
        ActionKey::DestroyRes
    } else if name.contains("升级 Resonator") || name.contains("Upgrade Resonator") {
        ActionKey::UpgradeRes
    } else if name.contains("部署 Resonator") || name.contains("Deploy Resonator") {
        ActionKey::DeployRes
    } else {
        // 默认返回Scan
        ActionKey::Scan
    }
}
