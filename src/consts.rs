use lazy_static::lazy_static;
use std::collections::HashMap;

pub(crate) const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub(crate) const ZHI: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];
pub(crate) const PALAIS: [&str; 12] = [
    "命宮",
    "父母宮",
    "福德宮",
    "田宅宮",
    "官祿宮",
    "交友宮",
    "遷移宮",
    "疾厄宮",
    "財帛宮",
    "子女宮",
    "夫妻宮",
    "兄弟宮",
];
pub(crate) const WUXINGJU: [&str; 5] = ["水二局", "木三局", "金四局", "土五局", "火六局"];
pub(crate) const ZIWEI_SYSTEM: [(usize, &str); 6] = [
    (0, "紫微"),
    (4, "廉貞"),
    (7, "天同"),
    (8, "武曲"),
    (9, "太陽"),
    (11, "天機"),
];
pub(crate) const TIANFU_SYSTEM: [(usize, &str); 8] = [
    (0, "天府"),
    (1, "太陰"),
    (2, "貪狼"),
    (3, "巨門"),
    (4, "天相"),
    (5, "天梁"),
    (6, "七殺"),
    (10, "破軍"),
];
pub(crate) const SIHUAXING: [&str; 4] = ["祿", "權", "科", "忌"];
pub(crate) const SIHUA_MAP: [[usize; 4]; 10] = [
    [5, 13, 3, 2],
    [1, 11, 0, 7],
    [4, 1, 16, 5],
    [7, 4, 1, 9],
    [8, 7, 15, 1],
    [3, 8, 11, 17],
    [2, 3, 7, 4],
    [9, 2, 17, 16],
    [11, 0, 14, 3],
    [13, 9, 7, 8],
];

pub(crate) const MAIN_STARS: [&str; 28] = [
    "紫微", "天機", "太陽", "武曲", "天同", "廉貞", "天府", "太陰", "貪狼", "巨門", "天相", "天梁",
    "七殺", "破軍", "左輔", "右弼", "文昌", "文曲", "天魁", "天鉞", "擎羊", "陀羅", "火星", "鈴星",
    "地空", "地劫", "祿存", "天馬",
];

pub(crate) const BOSHI: [&str; 12] = [
    "博士", "力士", "青龍", "小耗", "將軍", "奏書", "飛廉", "喜神", "病符", "大耗", "伏兵", "官府",
];

pub(crate) const CHANGSHENG: [&str; 12] = [
    "長生", "沐浴", "冠帶", "臨官", "帝旺", "衰", "病", "死", "墓", "絕", "胎", "養",
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
