use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNGを初期化する。再現性をもたせるため毎回同じシードを使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
////    // n個の要素が格納できるようベクタを初期化する
////    let mut v = Vec::with_capacity(n);
//
//    // 0からn - 1までの合計n回、繰り返し乱数を生成し、ベクタに追加する
//    // （0からn - 1の数列は使わないので _ で受けることですぐに破棄する）
//    for _ in 0..n {
//        v.push(rng.sample(&Standout))
//    }
//
//    v
    // rng.sample_iter()は乱数を無限に生成するイテレータを返す
    // take(n)は元のイテレータから最初のn要素だけ取り出すイテレータを返す
    // collect()はイテレータっから値を収集してベクタやハッシュマップのようなコレクションに格納する
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    // windows(2)は元のイテレータから1要素刻みで2要素ずつ値を取り出す新しいイテレータを返す
    // 例えば元が[1,2,3,4]なら[1,2][2,3][3,4]を順に返す
    //
    // all(...)はイテレータから値（例:[1,2]）を取り出し、クロージャに渡す
    // クロージャがfalseを返したらそこで処理を打ち切りfalseを返す
    // クロージャがtrueを返している間は、イテレータから次の値を取り出し、クロージャへ与え続ける
    // クロージャが一度もfalseを返さなかったらall(...)はtrueを返す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}
