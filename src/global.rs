use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub(crate) static ref MAIN_STARS_LOCATION: Mutex<HashMap<&'static str, usize>> =
        Mutex::new(HashMap::from([
            ("紫微", 0),
            ("天機", 0),
            ("太陽", 0),
            ("武曲", 0),
            ("天同", 0),
            ("廉貞", 0),
            ("天府", 0),
            ("太陰", 0),
            ("貪狼", 0),
            ("巨門", 0),
            ("天相", 0),
            ("天梁", 0),
            ("七殺", 0),
            ("破軍", 0),
            ("左輔", 0),
            ("右弼", 0),
            ("文昌", 0),
            ("文曲", 0),
            ("天魁", 0),
            ("天鉞", 0),
            ("擎羊", 0),
            ("陀羅", 0),
            ("火星", 0),
            ("鈴星", 0),
            ("地空", 0),
            ("地劫", 0),
            ("祿存", 0),
            ("天馬", 0),
        ]));
}
