pub mod config;
pub mod solver;
pub mod ui;

use config::{ActionConfig, Lang, default_configs};
use solver::{generate_solver_actions, solve_ap, solve_mixed};
use ui::{TextProvider, UiKey, print_header, print_config_list, print_prompts, COLOR_GREEN, COLOR_RESET, COLOR_RED};
use std::env;
use std::io;

fn main() {
    let mut current_lang = detect_system_lang();
    let mut ratio_field: u32 = 1;
    let mut ratio_link: u32 = 2;
    let mut target_ap: i32 = 40_000_000;
    let mut priority_action_id: Option<usize> = None;

    let mut configs = default_configs();
    let mut last_ap: i32 = 0;

    print_header(current_lang);
    print_config_list(&configs, current_lang, ratio_field, ratio_link, priority_action_id);

    loop {
        print_prompts(current_lang, last_ap, ratio_field, ratio_link);
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
                    if configs.iter().any(|c| c.id == action_id) {
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
                    println!("--> {}{}{}", COLOR_GREEN, TextProvider::format_msg_goal_reached(current_lang, new_ap), COLOR_RESET);
                } else {
                    let diff = target_ap - new_ap;
                    println!("--> {}", TextProvider::format_msg_diff(current_lang, new_ap, diff));

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
                        let has_apex = configs.iter().any(|c| c.apex_multiplier > 1);

                        if has_apex && !actions_with_apex.is_empty() {
                            solve_mixed(diff, &actions_no_apex, &actions_with_apex, current_lang, priority_action_id);
                        } else {
                            solve_ap(diff, &actions_no_apex, current_lang);
                        }
                    }
                }
            }
            Err(e) => println!("{}[!] {}{}", COLOR_RED, e, COLOR_RESET),
        }
    }
}

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

fn handle_config_command(input: &str, configs: &mut [ActionConfig], lang: Lang) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 2 {
        println!("{}", TextProvider::ui(UiKey::ErrInvalidCmd, lang));
        return;
    }
    let mode = parts[0];

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
