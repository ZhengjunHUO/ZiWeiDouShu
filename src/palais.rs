use crate::consts::{GAN, PALAIS, TIANFU_SYSTEM, WUXINGJU, WUXINGJU_DICT, ZHI, ZIWEI_SYSTEM};
use crate::structs::Birthday;

#[derive(Debug, Default)]
pub(crate) struct Palais {
    pub(crate) name: String,
    pub(crate) gz_name: String,
    pub(crate) gz_idx: (usize, usize),
    pub(crate) daxian: String,
    pub(crate) xiaoxian: String,
    pub(crate) stars_a: Vec<String>,
    pub(crate) stars_b: Vec<String>,
    pub(crate) stars_c: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct Mingpan {
    pub(crate) all_palais: [Palais; 12],
    pub(crate) info: String,
    pub(crate) ming_idx: usize,
    pub(crate) ziwei_idx: usize,
    pub(crate) wxj_idx: usize,
}

macro_rules! push_star {
    ($obj: ident, $palais_idx: expr, $star_array: ident, $star_name: literal) => {
        $obj.all_palais[$palais_idx]
            .$star_array
            .push(String::from($star_name));
    };
}

impl Mingpan {
    pub(crate) fn with_info(mut self, birth: &Birthday) -> Self {
        self.info = format!("{}", birth);
        self
    }

    /// 定十二宫天干
    pub(crate) fn with_tiangan_name(mut self, year_gan_idx: usize) -> Self {
        let start_idx = ((year_gan_idx % 5) * 2 + 2) % 10;

        (0..12).into_iter().for_each(|idx| {
            self.all_palais[(2 + idx) % 12].gz_name =
                format!("{}{}", GAN[(start_idx + idx) % 10], ZHI[(2 + idx) % 12]);
            self.all_palais[(2 + idx) % 12].gz_idx = ((start_idx + idx) % 10, (2 + idx) % 12);
        });
        self
    }

    /// 安命身宫及十二宫名
    pub(crate) fn with_shenming_palais(mut self, month: usize, hour: usize) -> Self {
        let zishi_idx = (1 + month) % 12;
        let ming_idx = (zishi_idx + 12 - hour) % 12;
        self.ming_idx = ming_idx;
        let shen_idx = (zishi_idx + hour) % 12;

        (0..12).into_iter().for_each(|idx| {
            let curr_idx = (ming_idx + idx) % 12;
            let mut curr_name = String::from(PALAIS[idx]);
            if curr_idx == shen_idx {
                curr_name.push_str("-身宫");
            }
            self.all_palais[curr_idx].name = curr_name;
        });
        self
    }

    /// 定五行局，dep: with_tiangan_name & with_shenming_palais
    pub(crate) fn with_wuxingju(mut self) -> Self {
        let gz = self.all_palais[self.ming_idx].gz_idx;
        let gan = gz.0;
        let mut zhi = gz.1;
        if zhi >= 6 {
            zhi = zhi - 6;
        }

        self.wxj_idx = *WUXINGJU_DICT.get(&(gan / 2, zhi / 2)).unwrap();
        self.info.push_str("\n");
        self.info.push_str(WUXINGJU[self.wxj_idx]);
        self
    }

    /// 安紫微星系，dep: with_wuxingju
    pub(crate) fn with_ziwei(mut self, day: usize) -> Self {
        let ju = self.wxj_idx + 2;
        if day % ju == 0 {
            self.ziwei_idx = (day / ju + 1) % 12;
        } else {
            let delta = ju - day % ju;
            let temp = day / ju + 2;
            if delta % 2 == 1 {
                self.ziwei_idx = (temp - delta + 12) % 12;
            } else {
                self.ziwei_idx = (temp + delta) % 12;
            }
        }

        ZIWEI_SYSTEM.iter().for_each(|&(idx, star)| {
            self.all_palais[(self.ziwei_idx + idx) % 12]
                .stars_a
                .push(star.to_owned());
        });
        self
    }

    /// 安天府星系，dep: with_ziwei
    pub(crate) fn with_tianfu(mut self) -> Self {
        let tianfu_idx = (16 - self.ziwei_idx) % 12;

        TIANFU_SYSTEM.iter().for_each(|&(idx, star)| {
            self.all_palais[(tianfu_idx + idx) % 12]
                .stars_a
                .push(star.to_owned());
        });
        self
    }

    /// 安时系星
    pub(crate) fn with_hour_based(mut self, hour: usize, nian_zhi: usize) -> Self {
        push_star!(self, (10 - hour + 12) % 12, stars_b, "文昌");
        push_star!(self, (hour + 4) % 12, stars_b, "文曲");

        let huo_idx;
        let ling_idx;
        match nian_zhi % 4 {
            // 子辰申
            0 => {
                huo_idx = (hour + 2) % 12;
                ling_idx = (hour + 10) % 12;
            }
            // 丑巳酉
            1 => {
                huo_idx = (hour + 3) % 12;
                ling_idx = (hour + 10) % 12;
            }
            // 寅午戌
            2 => {
                huo_idx = (hour + 1) % 12;
                ling_idx = (hour + 3) % 12;
            }
            // 卯未亥
            3 => {
                huo_idx = (hour + 9) % 12;
                ling_idx = (hour + 10) % 12;
            }
            _ => unreachable!(),
        }
        push_star!(self, huo_idx, stars_b, "火星");
        push_star!(self, ling_idx, stars_b, "铃星");

        push_star!(self, 11 - hour, stars_b, "地空");
        push_star!(self, (hour + 11) % 12, stars_b, "地劫");
        push_star!(self, (hour + 6) % 12, stars_c, "台辅");
        push_star!(self, (hour + 2) % 12, stars_c, "封诰");

        self
    }
}
