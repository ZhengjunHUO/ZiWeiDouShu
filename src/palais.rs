use crate::consts::{
    BOSHI, GAN, MAIN_STARS, PALAIS, SIHUAXING, SIHUA_MAP, TIANFU_SYSTEM, WUXINGJU, WUXINGJU_DICT,
    ZHI, ZIWEI_SYSTEM,
};
use crate::global::MAIN_STARS_LOCATION;
use crate::structs::Birthday;

#[derive(Debug, Default)]
pub(crate) struct Palais {
    pub(crate) name: String,
    pub(crate) gz_name: String,
    pub(crate) gz_idx: (usize, usize),
    pub(crate) daxian: String,
    pub(crate) xiaoxian: String,
    pub(crate) stars_a: Vec<Etoile>,
    pub(crate) stars_b: Vec<Etoile>,
    pub(crate) stars_c: Vec<Etoile>,
    pub(crate) stars_d: Vec<Etoile>,
}

#[derive(Debug, Default)]
pub(crate) struct Mingpan {
    pub(crate) all_palais: [Palais; 12],
    pub(crate) info: String,
    pub(crate) ming_idx: usize,
    pub(crate) shen_idx: usize,
    pub(crate) ziwei_idx: usize,
    pub(crate) wxj_idx: usize,
}

#[derive(Debug, Default)]
pub(crate) struct Etoile {
    pub(crate) name: String,
    pub(crate) lumino: usize,
    pub(crate) hua: Option<usize>,
}

impl Etoile {
    pub(crate) fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

impl std::string::ToString for Etoile {
    fn to_string(&self) -> String {
        let mut rslt = self.name.clone();
        // TODO: Look into dict for lumino
        match self.hua {
            Some(i) => rslt.push_str(SIHUAXING[i]),
            None => (),
        }
        rslt
    }
}

macro_rules! push_star {
    ($obj: ident, $palais_idx: expr, $star_array: ident, $star_name: expr) => {
        $obj.all_palais[($palais_idx) % 12]
            .$star_array
            .push(Etoile::default().with_name(String::from($star_name)));
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
        self.shen_idx = shen_idx;

        (0..12).into_iter().for_each(|idx| {
            let curr_idx = (ming_idx + idx) % 12;
            let mut curr_name = String::from(PALAIS[idx]);
            if curr_idx == shen_idx {
                curr_name.push_str("-身宮");
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
                self.ziwei_idx = (12 + temp - delta) % 12;
            } else {
                self.ziwei_idx = (temp + delta) % 12;
            }
        }

        let mut ms = MAIN_STARS_LOCATION.lock().unwrap();
        ZIWEI_SYSTEM.iter().for_each(|&(idx, star)| {
            self.all_palais[(self.ziwei_idx + idx) % 12]
                .stars_a
                .push(Etoile {
                    name: star.to_owned(),
                    lumino: 0,
                    hua: None,
                });
            ms.entry(star)
                .and_modify(|loc| *loc = (self.ziwei_idx + idx) % 12);
        });
        self
    }

    /// 安天府星系，dep: with_ziwei
    pub(crate) fn with_tianfu(mut self) -> Self {
        let tianfu_idx = (16 - self.ziwei_idx) % 12;

        let mut ms = MAIN_STARS_LOCATION.lock().unwrap();
        TIANFU_SYSTEM.iter().for_each(|&(idx, star)| {
            self.all_palais[(tianfu_idx + idx) % 12]
                .stars_a
                .push(Etoile {
                    name: star.to_owned(),
                    lumino: 0,
                    hua: None,
                });
            ms.entry(star)
                .and_modify(|loc| *loc = (tianfu_idx + idx) % 12);
        });

        self
    }

    /// 安时系星
    pub(crate) fn with_hour_based(mut self, hour: usize, nian_zhi: usize) -> Self {
        push_star!(self, 22 - hour, stars_b, "文昌");
        push_star!(self, hour + 4, stars_b, "文曲");

        let huo_idx;
        let ling_idx;
        match nian_zhi % 4 {
            // 子辰申
            0 => {
                huo_idx = hour + 2;
                ling_idx = hour + 10;
            }
            // 丑巳酉
            1 => {
                huo_idx = hour + 3;
                ling_idx = hour + 10;
            }
            // 寅午戌
            2 => {
                huo_idx = hour + 1;
                ling_idx = hour + 3;
            }
            // 卯未亥
            3 => {
                huo_idx = hour + 9;
                ling_idx = hour + 10;
            }
            _ => unreachable!(),
        }
        push_star!(self, huo_idx, stars_b, "火星");
        push_star!(self, ling_idx, stars_b, "鈴星");

        push_star!(self, 11 - hour, stars_b, "地空");
        push_star!(self, hour + 11, stars_b, "地劫");
        push_star!(self, hour + 6, stars_c, "台輔");
        push_star!(self, hour + 2, stars_c, "封誥");

        self
    }

    /// 安月系星
    pub(crate) fn with_month_based(mut self, month: usize) -> Self {
        push_star!(self, month + 3, stars_b, "左輔");
        push_star!(self, 23 - month, stars_b, "右弼");
        push_star!(self, month + 8, stars_c, "天刑");
        push_star!(self, month, stars_c, "天姚");

        let yuema: [usize; 4] = [8, 5, 2, 11];
        push_star!(self, yuema[(month - 1) % 4], stars_c, "月馬");

        if (month + 7) % 2 == 0 {
            push_star!(self, month + 7, stars_c, "解神");
        } else {
            push_star!(self, month + 6, stars_c, "解神");
        }

        let tianwu: [usize; 4] = [5, 8, 2, 11];
        push_star!(self, tianwu[(month - 1) % 4], stars_c, "天巫");

        let tianyue: [usize; 12] = [10, 5, 4, 2, 7, 3, 11, 7, 2, 6, 10, 2];
        push_star!(self, tianyue[month - 1], stars_c, "天月");

        push_star!(self, (28 - month * 2), stars_c, "陰煞");
        self
    }

    /// 安日系星
    pub(crate) fn with_day_based(mut self, day: usize, month: usize, hour: usize) -> Self {
        // 基于左辅右弼计算
        push_star!(self, month + 3 + day - 1, stars_c, "三台");
        push_star!(self, 47 - month - (day - 1), stars_c, "八座");
        // 基于文昌文曲计算
        push_star!(self, 22 - hour + day - 2, stars_c, "恩光");
        push_star!(self, hour + 4 + day - 2, stars_c, "天貴");

        self
    }

    /// 安年干系星
    pub(crate) fn with_year_gan_based(mut self, year_gan_idx: usize, is_clockwise: bool) -> Self {
        let lucun: [usize; 10] = [2, 3, 5, 6, 5, 6, 8, 9, 11, 0];
        let lucun_idx = lucun[year_gan_idx];
        push_star!(self, lucun_idx, stars_b, "祿存");
        push_star!(self, lucun_idx + 1, stars_b, "擎羊");
        push_star!(self, lucun_idx + 11, stars_b, "陀羅");

        let tiankui: [usize; 10] = [1, 0, 11, 11, 1, 0, 1, 6, 3, 3];
        let tianyue: [usize; 10] = [7, 8, 9, 9, 7, 8, 7, 2, 5, 5];
        push_star!(self, tiankui[year_gan_idx], stars_b, "天魁");
        push_star!(self, tianyue[year_gan_idx], stars_b, "天鉞");

        let tianguan: [usize; 10] = [7, 4, 5, 2, 3, 9, 11, 9, 10, 6];
        let tianfu: [usize; 10] = [9, 8, 0, 11, 3, 2, 6, 5, 6, 5];
        push_star!(self, tianguan[year_gan_idx], stars_c, "天官");
        push_star!(self, tianfu[year_gan_idx], stars_c, "天福");

        // 四化星
        let ms = MAIN_STARS_LOCATION.lock().unwrap();
        let sihua = SIHUA_MAP[year_gan_idx];

        for i in 0..4 {
            let star = MAIN_STARS[sihua[i]];
            let star_loc = *ms.get(star).unwrap();

            let list;
            if sihua[i] < 14 {
                list = &mut self.all_palais[star_loc].stars_a
            } else {
                list = &mut self.all_palais[star_loc].stars_b
            }
            for s in list {
                if s.name == star {
                    s.hua = Some(i);
                    break;
                }
            }
        }

        // 博士十二神
        for i in 0..12 {
            if is_clockwise {
                push_star!(self, lucun_idx + i, stars_d, BOSHI[i]);
            } else {
                push_star!(self, 12 + lucun_idx - i, stars_d, BOSHI[i]);
            }
        }

        self
    }

    /// 安年支系星
    pub(crate) fn with_year_zhi_based(mut self, year_zhi_idx: usize) -> Self {
        push_star!(self, 18 - year_zhi_idx, stars_c, "天哭");
        push_star!(self, year_zhi_idx + 6, stars_c, "天虛");
        push_star!(self, year_zhi_idx + 4, stars_c, "龍池");
        push_star!(self, 22 - year_zhi_idx, stars_c, "鳳閣");
        push_star!(self, 15 - year_zhi_idx, stars_c, "紅鸞");
        push_star!(self, 21 - year_zhi_idx, stars_c, "天喜");
        push_star!(
            self,
            (5 + (((10 + year_zhi_idx) % 12) / 3) * 3) % 12,
            stars_c,
            "孤辰"
        );
        push_star!(
            self,
            (1 + (((10 + year_zhi_idx) % 12) / 3) * 3) % 12,
            stars_c,
            "寡宿"
        );

        let feilian: [usize; 12] = [8, 9, 10, 5, 6, 7, 2, 3, 4, 11, 0, 1];
        push_star!(self, feilian[year_zhi_idx], stars_c, "蜚廉");

        let posui: [usize; 3] = [5, 1, 9];
        push_star!(self, posui[year_zhi_idx % 3], stars_c, "破碎");

        push_star!(self, self.ming_idx + year_zhi_idx, stars_c, "天才");
        push_star!(self, self.shen_idx + year_zhi_idx, stars_c, "天壽");

        let tianma: [usize; 4] = [2, 11, 8, 5];
        push_star!(self, tianma[year_zhi_idx % 4], stars_b, "天馬");

        push_star!(self, year_zhi_idx + 1, stars_c, "天空");

        let huagai: [usize; 4] = [4, 1, 10, 7];
        push_star!(self, huagai[year_zhi_idx % 4], stars_c, "華蓋");

        push_star!(self, year_zhi_idx, stars_c, "太歲");

        let xianchi: [usize; 4] = [9, 6, 3, 0];
        push_star!(self, xianchi[year_zhi_idx % 4], stars_c, "咸池");

        self
    }

    pub(crate) fn with_daxian(mut self, is_clockwise: bool) -> Self {
        let mut begin = self.wxj_idx + 2;
        let mut end = self.wxj_idx + 11;

        for i in 0..12 {
            if is_clockwise {
                self.all_palais[(self.ming_idx + i) % 12].daxian = format!("{}-{}", begin, end);
            } else {
                self.all_palais[(12 + self.ming_idx - i) % 12].daxian =
                    format!("{}-{}", begin, end);
            }
            begin += 10;
            end += 10;
        }

        self
    }
}
