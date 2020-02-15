use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;
use term::Attr::Standout;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNGを初期化する。再現性をもたせるため毎回同じシードを使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    // n個の要素が格納できるようベクタを初期化する
    let mut v = Vec::with_capacity(n);

    // 0からn - 1までの合計n回、繰り返し乱数を生成し、ベクタに追加する
    // （0からn - 1の数列は使わないので _ で受けることですぐに破棄する）
    for _ in 0..n {
        v.push(rng.sample(&Standout))
    }
    v
}

