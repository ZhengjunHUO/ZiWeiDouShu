use lunardate::LunarDate;
use rust_ephemeris::lunnar::{GanZhi, SolorDate};

pub(crate) struct Birthday {
    pub(crate) year: GanZhi,
    pub(crate) month: GanZhi,
    pub(crate) day: GanZhi,
    pub(crate) hour: GanZhi,
    pub(crate) lunar: LunarDate,
    pub(crate) gender: String,
}

impl Birthday {
    pub(crate) fn from_gregorian(info: (i32, u32, u32, f64, bool)) -> Self {
        let d = SolorDate(info.0, info.1 as i32, info.2 as i32);
        let sz = d.sizhu(info.3 / 24.0);
        Self {
            year: sz.0,
            month: sz.1,
            day: sz.2,
            hour: sz.3,
            lunar: LunarDate::from_solar_date(info.0, info.1, info.2).unwrap(),
            gender: match (sz.0 .0 % 2 == 0, info.4) {
                (true, true) => String::from("陽男"),
                (true, false) => String::from("陽女"),
                (false, true) => String::from("陰男"),
                (false, false) => String::from("陰女"),
            },
        }
    }
}

impl std::fmt::Display for Birthday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "農曆生日: {}年{}{}月{}日{}時\n八字: {} {} {} {} ({})",
            self.lunar.year(),
            if self.lunar.is_leap_month() {
                "闰"
            } else {
                ""
            },
            self.lunar.month(),
            self.lunar.day(),
            self.hour.zhi(),
            self.year,
            self.month,
            self.day,
            self.hour,
            self.gender
        )
    }
}
