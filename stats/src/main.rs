extern crate gnuplot;
extern crate stats;

use gnuplot::*;
use stats::Stats;

fn main() {
    // インスタンス生成
    let mut stat = Stats::new();

    // サンプルデータ
    let y: Vec<f64> = vec![
        100.0, 105.7, 117.1, 122.2, 107.1, 113.9, 126.8, 128.5, 119., 127.2, 127.7, 138.2, 133.3,
        142.8, 153.4, 156.9, 157.7, 156., 164.5, 166.1, 164.6, 162.1, 175.7, 182.8, 182.1, 185.9,
        185.9, 197.4, 206.3, 190.4, 197.8, 200.1, 208.9,
    ];
    let sz = y.len();
    let mut x: Vec<f64> = vec![0.0; sz];
    let mut p: Vec<f64> = vec![0.0; sz];

    // データを分析
    for i in 0..sz {
        x[i] = i as f64;
        stat.add(x[i], y[i]);
    }
    //　傾きと切片を求める
    let a = stat.slope();
    let b = stat.intercept();
    println!("切片={}, 傾き={}", b, a);

    // 推定値を計算
    for i in 0..sz {
        p[i] = a * x[i] + b;
    }
    // プロット
    let x2 = x.clone();
    let mut fg = Figure::new();
    fg.axes2d()
        .points(&x, &y, &[Color("red")])
        .lines(&x2, &p, &[Color("blue")]);
    fg.show();
}
