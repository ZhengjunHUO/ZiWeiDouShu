use lazy_static::lazy_static;
use std::collections::HashMap;

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
