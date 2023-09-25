use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub(crate) static ref MAIN_STARS_LOCATION: Mutex<HashMap<&'static str, usize>> =
        Mutex::new(HashMap::from([
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
        ]));
}
