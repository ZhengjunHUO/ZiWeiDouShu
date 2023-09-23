use lunardate::LunarDate;
use rust_ephemeris::lunnar::{GanZhi, SolorDate};

pub struct Birthday {
    pub year: GanZhi,
    pub month: GanZhi,
    pub day: GanZhi,
    pub hour: GanZhi,
    pub lunar: LunarDate,
}

impl Birthday {
    pub fn from_gregorian(date: (i32, u32, u32, f64)) -> Self {
        let d = SolorDate(date.0, date.1 as i32, date.2 as i32);
        let sz = d.sizhu(date.3 / 24.0);
        Self {
            year: sz.0,
            month: sz.1,
            day: sz.2,
            hour: sz.3,
            lunar: LunarDate::from_solar_date(date.0, date.1, date.2).unwrap(),
        }
    }
}

impl std::fmt::Display for Birthday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "農曆生日: {}年{}{}月{}日{}時\n八字: {} {} {} {}", 
            self.lunar.year(),
            if self.lunar.is_leap_month() { "闰" } else { "" },
            self.lunar.month(),
            self.lunar.day(),
            self.hour.zhi(),
            self.year, self.month, self.day, self.hour
        )
    }
}
