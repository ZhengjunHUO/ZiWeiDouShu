use crate::consts::{GAN, GAN_DICT, ZHI};

#[derive(Debug, Default)]
pub(crate) struct Palais {
    pub(crate) name: String,
    pub(crate) gz_name: String,
    pub(crate) daxian: String,
    pub(crate) xiaoxian: String,
    pub(crate) stars_a: Vec<String>,
    pub(crate) stars_b: Vec<String>,
    pub(crate) stars_c: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct Mingpan {
    pub(crate) all_palais: [Palais; 12],
}

impl Mingpan {
    /// 定十二宫天干
    pub(crate) fn with_tiangan_name(mut self, start_idx: usize) -> Self {
        (0..12).into_iter().for_each(|idx| {
            self.all_palais[(2 + idx) % 12].gz_name =
                format!("{}{}", GAN[(start_idx + idx) % 10], ZHI[(2 + idx) % 12])
        });
        self
    }
}
