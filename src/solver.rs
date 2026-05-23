use crate::config::{ActionConfig, ActionKey, Lang};
use crate::ui::{TextProvider, UiKey, COLOR_CYAN, COLOR_RESET, COLOR_YELLOW, print_recommendations_mixed};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SolverAction {
    pub id: usize,
    pub name: String,
    pub ap: i32,
    pub bundle_info: Option<String>,
}

pub fn generate_solver_actions(
    configs: &[ActionConfig],
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

        if c.key == ActionKey::CreateField || c.key == ActionKey::DestroyField {
            let link_cost = if c.key == ActionKey::CreateField { link_ap } else { d_link_ap };
            let bundle = (c.base_ap * r_field as i32) + (link_cost * r_link as i32);
            
            let ap_no_apex = bundle * c.global_multiplier as i32;
            let ap_with_apex = bundle * c.global_multiplier as i32 * c.apex_multiplier as i32;

            list_no_apex.push(SolverAction {
                id: c.id,
                name: format!("{} ({}F + {}L)", raw_name, r_field, r_link),
                ap: ap_no_apex,
                bundle_info: Some(format!("(Batch: {} Field + {} Link)", r_field, r_link)),
            });
            list_with_apex.push(SolverAction {
                id: c.id,
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
                id: c.id,
                name: name_no_apex,
                ap: ap_no_apex,
                bundle_info: None,
            });
            list_with_apex.push(SolverAction {
                id: c.id,
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

pub fn solve_mixed(
    diff: i32,
    actions_no_apex: &[SolverAction],
    actions_with_apex: &[SolverAction],
    lang: Lang,
    priority_action_id: Option<usize>,
) {
    let mut recommendation_no_apex: HashMap<String, i32> = HashMap::new();
    let mut recommendation_with_apex: HashMap<String, i32> = HashMap::new();
    let mut remaining = diff;

    if let Some(priority_id) = priority_action_id {
        if let Some(priority_action) = actions_with_apex.iter().find(|a| a.id == priority_id) {
            let priority_bulk = remaining / priority_action.ap;
            if priority_bulk > 0 {
                recommendation_with_apex.insert(priority_action.name.clone(), priority_bulk);
                remaining -= priority_bulk * priority_action.ap;
            }
        } else if let Some(priority_action) = actions_no_apex.iter().find(|a| a.id == priority_id) {
            let priority_bulk = remaining / priority_action.ap;
            if priority_bulk > 0 {
                recommendation_no_apex.insert(priority_action.name.clone(), priority_bulk);
                remaining -= priority_bulk * priority_action.ap;
            }
        }
    }

    if !actions_with_apex.is_empty() {
        if let Some(max_apex) = actions_with_apex.first() {
            let apex_count = remaining / max_apex.ap;
            if apex_count > 0 {
                *recommendation_with_apex.entry(max_apex.name.clone()).or_insert(0) += apex_count;
                remaining -= apex_count * max_apex.ap;
            }
        }
    }

    let max_s = remaining as usize;
    let mut dp = vec![i32::MAX; max_s + 1];
    let mut parent = vec![None; max_s + 1];
    dp[0] = 0;

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
        let msg = TextProvider::format_result_success(lang, diff);
        println!("\n{}", msg);

        let mut curr = max_s;
        while curr > 0 {
            if let Some((prev, idx)) = parent[curr] {
                let action = all_actions[idx];
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

        if !recommendation_with_apex.is_empty() {
            println!(
                "\n{}{}=== 使用Apex道具 ==={}{}",
                COLOR_YELLOW, COLOR_CYAN, COLOR_RESET, COLOR_RESET
            );
            print_recommendations_mixed(&recommendation_with_apex, actions_with_apex, lang, true);
        }

        if !recommendation_no_apex.is_empty() {
            println!(
                "\n{}{}=== 普通动作（无Apex） ==={}{}",
                COLOR_YELLOW, COLOR_CYAN, COLOR_RESET, COLOR_RESET
            );
            print_recommendations_mixed(&recommendation_no_apex, actions_no_apex, lang, false);
        }

        let total_apex: i32 = recommendation_with_apex.values().sum();
        let total_normal: i32 = recommendation_no_apex.values().sum();
        println!(
            "\n{}总计: {} 次Apex动作 + {} 次普通动作{}",
            COLOR_CYAN, total_apex, total_normal, COLOR_RESET
        );
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

pub fn solve_ap(mut diff: i32, actions: &[SolverAction], lang: Lang) {
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
        let msg = TextProvider::format_result_success(lang, diff);
        println!("\n{}", msg);

        let mut curr = max_s;
        while curr > 0 {
            if let Some((prev, idx)) = parent[curr] {
                *recommendation.entry(actions[idx].name.clone()).or_insert(0) += 1;
                curr = prev;
            } else {
                break;
            }
        }
        
        println!("----------------------------------------");
        let suffix = TextProvider::ui(UiKey::ResultListPrefix, lang);
        for action in actions {
            if let Some(&count) = recommendation.get(&action.name) {
                let name_padded = crate::ui::pad_str_visual(&action.name, 35);
                let extra = match &action.bundle_info {
                    Some(i) => format!("{}{}", crate::ui::COLOR_GRAY, i),
                    None => "".to_string(),
                };
                println!(
                    " - {} : x {}{}{} {} {}",
                    name_padded, crate::ui::COLOR_CYAN, count, crate::ui::COLOR_RESET, suffix, extra
                );
            }
        }
        println!("----------------------------------------");
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
