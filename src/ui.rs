use crate::config::{ActionConfig, ActionKey, Lang};
use std::collections::HashMap;

// --- 颜色常量 ---
pub const COLOR_RESET: &str = "\x1b[0m";
pub const COLOR_GREEN: &str = "\x1b[32m";
pub const COLOR_RED: &str = "\x1b[31m";
pub const COLOR_YELLOW: &str = "\x1b[33m";
pub const COLOR_CYAN: &str = "\x1b[36m";
pub const COLOR_GRAY: &str = "\x1b[90m";

pub enum UiKey {
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
    ErrNoActions,
    ErrInvalidId,
    ErrInvalidNum,
    ErrInvalidCmd,
    StatusOn,
    StatusOff,
    ResultFail,
    ResultListPrefix,
}

pub struct TextProvider;

impl TextProvider {
    pub fn action_name(key: ActionKey, lang: Lang) -> &'static str {
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

    pub fn ui(key: UiKey, lang: Lang) -> &'static str {
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
            (Lang::CN, UiKey::PromptLine2) => "- [+/-]  (e.g. +500)          : 基于上次 AP ({last_ap}) 计算",
            (Lang::EN, UiKey::PromptLine2) => "- [+/-]  (e.g. +500)          : Calc based on last AP ({last_ap})",
            (Lang::CN, UiKey::PromptLine3) => "- t <ID> (e.g. t 1)           : 开关某个动作",
            (Lang::EN, UiKey::PromptLine3) => "- t <ID> (e.g. t 1)           : Toggle action",
            (Lang::CN, UiKey::PromptLine4) => "- a <N> (e.g. a 2)            : 设置全局Apex倍率 (1为不使用)",
            (Lang::EN, UiKey::PromptLine4) => "- a <N> (e.g. a 2)            : Set global Apex multiplier (1=off)",
            (Lang::CN, UiKey::PromptLine5) => "- lang <en/cn>                : 切换语言 (Switch Language)",
            (Lang::EN, UiKey::PromptLine5) => "- lang <en/cn>                : Switch Language",
            (Lang::CN, UiKey::PromptLine6) => "- ratio <F> <L> (当前: {ratio})    : 设置 Field:Link 比例",
            (Lang::EN, UiKey::PromptLine6) => "- ratio <F> <L> (Curr: {ratio})    : Set Field:Link ratio",
            (Lang::CN, UiKey::PromptLine7) => "- g <N> (e.g. g 2)            : 设置全局倍率 (活动倍率)",
            (Lang::EN, UiKey::PromptLine7) => "- g <N> (e.g. g 2)            : Set global multiplier (event)",
            (Lang::CN, UiKey::PromptLine8) => "- target <N> (e.g. target 50000000) : 设置目标AP值",
            (Lang::EN, UiKey::PromptLine8) => "- target <N> (e.g. target 50000000) : Set target AP value",
            (Lang::CN, UiKey::PromptLine9) => "- priority <ID> (e.g. priority 1)   : 设置优先动作ID",
            (Lang::EN, UiKey::PromptLine9) => "- priority <ID> (e.g. priority 1)   : Set priority action ID",
            (Lang::CN, UiKey::PromptLineQ) => "- q                           : 退出",
            (Lang::EN, UiKey::PromptLineQ) => "- q                           : Quit",

            (Lang::CN, UiKey::InputArrow) => "请输入指令或 AP > ",
            (Lang::EN, UiKey::InputArrow) => "Enter command or AP > ",

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

            (Lang::CN, UiKey::ResultFail) => "[注意] 无法精确到达。最近的 10 个 AP 节点：",
            (Lang::EN, UiKey::ResultFail) => "[Note] Exact match failed. Closest 10 AP nodes:",
            (Lang::CN, UiKey::ResultListPrefix) => "次",
            (Lang::EN, UiKey::ResultListPrefix) => "times",
        }
    }
    
    // 动态消息格式化方法
    pub fn format_prompt_line2(lang: Lang, last_ap: i32) -> String {
        let template = Self::ui(UiKey::PromptLine2, lang);
        template.replace("{last_ap}", &last_ap.to_string())
    }
    
    pub fn format_prompt_line6(lang: Lang, ratio_field: u32, ratio_link: u32) -> String {
        let template = Self::ui(UiKey::PromptLine6, lang);
        let ratio_str = format!("{}:{}", ratio_field, ratio_link);
        template.replace("{ratio}", &ratio_str)
    }

    pub fn format_msg_goal_reached(lang: Lang, current_ap: i32) -> String {
        match lang {
            Lang::CN => format!("当前 AP: {}。已达成目标！", current_ap),
            Lang::EN => format!("Current AP: {}. Target reached!", current_ap),
        }
    }

    pub fn format_msg_diff(lang: Lang, current_ap: i32, diff: i32) -> String {
        match lang {
            Lang::CN => format!("当前 AP: {} (距离目标还差: {}{}{})", current_ap, COLOR_YELLOW, diff, COLOR_RESET),
            Lang::EN => format!("Current AP: {} (Diff to target: {}{}{})", current_ap, COLOR_YELLOW, diff, COLOR_RESET),
        }
    }

    pub fn format_result_success(lang: Lang, diff: i32) -> String {
        match lang {
            Lang::CN => format!("[成功] 方案如下 (剩余 {}{}{} AP 的优化):", COLOR_YELLOW, diff, COLOR_RESET),
            Lang::EN => format!("[Success] Solution found (Optimizing last {}{}{} AP):", COLOR_YELLOW, diff, COLOR_RESET),
        }
    }
}

pub fn print_header(lang: Lang) {
    println!("========================================");
    println!("{}", TextProvider::ui(UiKey::Title, lang));
    println!("========================================");
}

pub fn print_config_list(configs: &[ActionConfig], lang: Lang, r_field: u32, r_link: u32, priority_action_id: Option<usize>) {
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

    let w_name = 42;
    let w_base = 14;
    let w_state = 8;
    let w_mult = 8;

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

        if Some(c.id) == priority_action_id {
            display_name = format!("{} [优先]", display_name);
        }

        let name_padded = pad_str_visual(&display_name, w_name);
        let ap_raw = effective_ap.to_string();
        let ap_padded = pad_str_visual(&ap_raw, w_base); 
        let ap_colored = if c.enabled {
            ap_padded 
        } else {
            format!("{}{}{}", COLOR_GRAY, ap_padded, COLOR_RESET) 
        };

        let status_raw = if c.enabled {
            TextProvider::ui(UiKey::StatusOn, lang)
        } else {
            TextProvider::ui(UiKey::StatusOff, lang)
        };
        let status_padded = pad_str_visual(status_raw, w_state); 
        let status_colored = if c.enabled {
            format!("{}{}{}", COLOR_GREEN, status_padded, COLOR_RESET)
        } else {
            format!("{}{}{}", COLOR_RED, status_padded, COLOR_RESET)
        };

        let mult_raw = if c.global_multiplier > 1 || c.apex_multiplier > 1 {
            if c.apex_multiplier > 1 {
                format!("G:{} A:{}", c.global_multiplier, c.apex_multiplier)
            } else {
                format!("G:{}", c.global_multiplier)
            }
        } else {
            "--".to_string()
        };
        let mult_padded = pad_str_visual(&mult_raw, w_mult); 
        let mult_colored = if c.global_multiplier > 1 || c.apex_multiplier > 1 {
            format!("{}{}{}", COLOR_YELLOW, mult_padded, COLOR_RESET)
        } else {
            format!("{}{}{}", COLOR_GRAY, mult_padded, COLOR_RESET)
        };

        println!(
            "{:<4} {} {} {} {}",
            c.id, name_padded, ap_colored, status_colored, mult_colored
        );
    }
    println!("{:-<92}", "-");
}

pub fn pad_str_visual(s: &str, target_width: usize) -> String {
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

pub fn print_prompts(lang: Lang, last_ap: i32, ratio_field: u32, ratio_link: u32) {
    println!();
    println!("{}", TextProvider::ui(UiKey::PromptLine1, lang));
    println!("{}", TextProvider::format_prompt_line2(lang, last_ap));
    println!("{}", TextProvider::ui(UiKey::PromptLine3, lang));
    println!("{}", TextProvider::ui(UiKey::PromptLine4, lang));
    println!("{}", TextProvider::ui(UiKey::PromptLine7, lang));
    println!("{}", TextProvider::ui(UiKey::PromptLine8, lang));
    println!("{}", TextProvider::ui(UiKey::PromptLine9, lang));
    println!("{}", TextProvider::ui(UiKey::PromptLine5, lang));
    println!("{}", TextProvider::format_prompt_line6(lang, ratio_field, ratio_link));
    println!("{}", TextProvider::ui(UiKey::PromptLineQ, lang));
    print!("{}", TextProvider::ui(UiKey::InputArrow, lang));
}

pub fn print_recommendations_mixed(
    rec: &HashMap<String, i32>,
    order: &[crate::solver::SolverAction],
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
