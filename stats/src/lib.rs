// 統計関数
#[derive(Copy, Clone)]
pub struct Stats {
    n: f64,
    x: f64,
    y: f64,
    xx: f64,
    yy: f64,
    xy: f64,
}

impl Stats {
    // コンストラクタ
    pub fn new() -> Self {
        Stats {
            n: 0.0,
            x: 0.0,
            y: 0.0,
            xx: 0.0,
            yy: 0.0,
            xy: 0.0,
        }
    }

    // サンプル追加
    pub fn add(&mut self, x: f64, y: f64) {
        self.n += 1.0;
        self.x += x;
        self.y += y;
        self.xx += x * x;
        self.yy += y * y;
        self.xy += x * y;
    }
    // サンプル削除
    pub fn sub(&mut self, x: f64, y: f64) {
        self.n -= 1.0;
        self.x -= x;
        self.y -= y;
        self.xx -= x * x;
        self.yy -= y * y;
        self.xy -= x * y;
    }

    //相関係数
    pub fn corr(&self) -> f64 {
        let div = self.dev_sq_x().sqrt() * self.dev_sq_y().sqrt();
        if div > 0.0 {
            self.dev_prod_xy() / div
        } else {
            0.0
        }
    }
    //残差平方和
    pub fn residuals(&self) -> f64 {
        let devsqx = self.dev_sq_x();
        if devsqx > 0.0 {
            self.dev_sq_y() - (self.dev_prod_xy().powi(2) / devsqx)
        } else {
            0.0
        }
    }
    // 切片
    pub fn intercept(&self) -> f64 {
        if self.n > 0.0 {
            (self.y - self.slope() * self.x) / self.n
        } else {
            self.mean_y()
        }
    }
    // 傾き
    pub fn slope(&self) -> f64 {
        let devsqx = self.dev_sq_x();
        if devsqx > 0.0 {
            self.dev_prod_xy() / devsqx
        } else {
            0.0
        }
    }
    // 偏差平方和 Ｘ
    fn dev_sq_x(&self) -> f64 {
        if self.n > 0.0 {
            (self.xx * self.n - self.x * self.x) / self.n
        } else {
            0.0
        }
    }
    // 偏差平方和 Y
    fn dev_sq_y(&self) -> f64 {
        if self.n > 0.0 {
            (self.yy * self.n - self.y * self.y) / self.n
        } else {
            0.0
        }
    }
    // 偏差積和 ＸＹ
    fn dev_prod_xy(&self) -> f64 {
        if self.n > 0.0 {
            (self.xy * self.n - self.x * self.y) / self.n
        } else {
            0.0
        }
    }
    // 平均Ｙ
    fn mean_y(&self) -> f64 {
        if self.n > 0.0 {
            self.y / self.n
        } else {
            0.0
        }
    }
}
