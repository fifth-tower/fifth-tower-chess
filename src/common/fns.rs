use std::ops::Deref;

pub fn to_turn_label(second: u16) -> String {
    if second == 0 {
        "不限".to_string()
    } else {
        format!("{}s", second)
    }
}
