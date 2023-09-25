use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub(crate) const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub(crate) const ZHI: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];
pub(crate) const PALAIS: [&str; 12] = [
    "命宫",
    "父母宫",
    "福德宫",
    "田宅宫",
    "官禄宫",
    "交友宫",
    "迁移宫",
    "疾厄宫",
    "财帛宫",
    "子女宫",
    "夫妻宫",
    "兄弟宫",
];
pub(crate) const WUXINGJU: [&str; 5] = ["水二局", "木三局", "金四局", "土五局", "火六局"];
pub(crate) const ZIWEI_SYSTEM: [(usize, &str); 6] = [
    (0, "紫微"),
    (4, "廉贞"),
    (7, "天同"),
    (8, "武曲"),
    (9, "太阳"),
    (11, "天机"),
];
pub(crate) const TIANFU_SYSTEM: [(usize, &str); 8] = [
    (0, "天府"),
    (1, "太阴"),
    (2, "贪狼"),
    (3, "巨门"),
    (4, "天相"),
    (5, "天梁"),
    (6, "七杀"),
    (10, "破军"),
];
pub(crate) const SIHUAXING: [&str; 4] = ["禄", "权", "科", "忌"];
pub(crate) const SIHUA_MAP: [(usize, usize, usize, usize); 10] = [
    (5, 13, 3, 2),
    (1, 11, 0, 7),
    (4, 1, 16, 5),
    (7, 4, 1, 9),
    (8, 7, 15, 1),
    (3, 8, 11, 17),
    (2, 3, 7, 4),
    (9, 2, 17, 16),
    (11, 0, 14, 3),
    (13, 9, 7, 8),
];

lazy_static! {
    pub(crate) static ref GAN_DICT: HashMap<String, usize> = GAN
        .iter()
        .enumerate()
        .map(|(idx, &s)| (String::from(s), idx))
        .collect();
    pub(crate) static ref ZHI_DICT: HashMap<String, usize> = ZHI
        .iter()
        .enumerate()
        .map(|(idx, &s)| (String::from(s), idx))
        .collect();
    pub(crate) static ref WUXINGJU_DICT: HashMap<(usize, usize), usize> = HashMap::from([
        ((0, 0), 2),
        ((0, 1), 0),
        ((0, 2), 4),
        ((1, 0), 0),
        ((1, 1), 4),
        ((1, 2), 3),
        ((2, 0), 4),
        ((2, 1), 3),
        ((2, 2), 1),
        ((3, 0), 3),
        ((3, 1), 1),
        ((3, 2), 2),
        ((4, 0), 1),
        ((4, 1), 2),
        ((4, 2), 0),
    ]);
    pub(crate) static ref MAIN_STARS: Mutex<[(&'static str, usize); 28]> = Mutex::new([
        ("紫微", 0),
        ("天机", 0),
        ("太阳", 0),
        ("武曲", 0),
        ("天同", 0),
        ("廉贞", 0),
        ("天府", 0),
        ("太阴", 0),
        ("贪狼", 0),
        ("巨门", 0),
        ("天相", 0),
        ("天梁", 0),
        ("七杀", 0),
        ("破军", 0),
        ("左辅", 0),
        ("右弼", 0),
        ("文昌", 0),
        ("文曲", 0),
        ("天魁", 0),
        ("天钺", 0),
        ("擎羊", 0),
        ("陀罗", 0),
        ("火星", 0),
        ("铃星", 0),
        ("地空", 0),
        ("地劫", 0),
        ("禄存", 0),
        ("天马", 0)
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict() {
        assert_eq!(GAN_DICT.get("丙"), Some(&2));
        assert_eq!(GAN_DICT.get("庚"), Some(&6));
        assert_eq!(ZHI_DICT.get("寅"), Some(&2));
        assert_eq!(ZHI_DICT.get("申"), Some(&8));
    }
}
