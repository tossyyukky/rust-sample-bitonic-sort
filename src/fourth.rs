use super::SortOrder;
use rayon;
use std::cmp::Ordering;

// 並列に処理するかを決めるしきい値
const PARALLEL_THRESHOLD: usize = 4096;

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
//        let first = &mut x[..mid_point];
//        let second = &mut x[mid_point..];
        // xをmid_pointを境にした2つの可変の借用に分割し
        // firstとsecondに束縛する
        let (first, second) = x.split_at_mut(mid_point);

        // xの分割後の要素数をしきい値と比較する
        if mid_point >= PARALLEL_THRESHOLD {
            // しきい値以上なら並列にソートする（並列処理）
            rayon::join(|| do_sort(first, true, comparator),
                        || do_sort(second, false, comparator)
            );
        } else {
            // しきい値未満なら順番にソートする（順次処理）
            do_sort(&mut x[..mid_point], true, comparator);
            do_sort(&mut x[mid_point..], false, comparator);
        }
        sub_sort(x, forward, comparator);
    }
}

pub fn sort<T: Ord + Send>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    // do_sortを呼ぶ代わりにsort_byを呼ぶ
    // is_power_of_twoはsort_byが呼ぶのでここからは削除
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, compartor: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        compare_and_swap(x, forward, compartor);
        let mid_point = x.len() / 2;
        let (first, second) = x.split_at_mut(mid_point);

        if mid_point >= PARALLEL_THRESHOLD {
            rayon::join(|| sub_sort(first, forward, compartor),
                        || sub_sort(second, forward, compartor));
        } else {
            sub_sort(first, forward, compartor);
            sub_sort(second, forward, compartor);
        }
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    // 比較に先立ち、forward(bool)をOrderingに変換する
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // comparatorクロージャで2要素を比較し、返されたOrderingのバリアントが
        // swap_conditionと等しいなら要素を交換する
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i)
        }
    }
}

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

#[cfg(test)]
mod tests {
    // 親モジュール(first)のsort関数を使用する
    use super::{sort, sort_by};
    use crate::SortOrder::*;
    use crate::utils::{new_u32_vec, is_sorted_ascending, is_sorted_descending};

    // #[test]のついた関数はcargo testとした時に実行される
    #[test]
    fn sort_u32_ascending() {
        // テストデータとしてu32型のベクタを作成しxに束縛する
        // sort関数によって内容が更新されるので、可変を表すmutが必要
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // xのスライスを作成し、sort関数を呼び出す
        // &mut x は&mut x[..]と書いてもいい
        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        // xの要素が降順にソートされていることを確認する
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        // 文字列のベクタを作りソートする
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        );
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11]; // x.len()が2のべき乗になっていない
        assert!(sort(&mut x, &Ascending).is_err()); // 戻り値はErr
    }

    // 構造体Studentを定義する
    // 構造体は関連する値を一つにまとめたデータ構造。複数のデータフィールドを持つ
    // deriveアトリビュートを使い、DebugトレイトとPartialEqトレイトを自動導出する
    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String, // first_bame(名前)フィールド。String型
        last_name: String,  // last_name(名字)フィールド。String型
        age: u8,            // age(年齢)フィールド。u8型(8ビット符号なし整数)
    }

    // implブロックを使うと対象の型に関連関数やメソッドを実装できる
    impl Student {
        // 構造体Studentを初期化して返す。Selfはimpl対象の型（Student）の別名
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                // to_stringメソッドで&str型の引数からString型の値を作る。詳しくは5章で説明
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age, // フィールドと変数が同じ名前なら省略形で書ける（JSみたいに
            }
        }
    }

    //    impl PartialEq for Student {
    //        fn eq(&self, other: &self) -> bool {
    //             selfとotherですべてのフィールド同士を比較して、
    //             どのフィールドも等しいならselfとotherは等しい
    //            self.first_name == other.first_name
    //            && self.last_name == other.last_name
    //            && self.age == other.age
    //        }
    //    }

    #[test]
    fn sort_students_by_age_ascending() {
        // 4人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタを作成する
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        // ソート後の期待値を作成する
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(
            // sort_by関数でソートする。第荷引数はソート順を決めるクロージャ
            // 引数に2つのStudent構造体を取り、ageフィールドの値をcmpメソッドで
            // 比較することで大小を決定する
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );

        assert_eq!(x, expected)
    }

    #[test]
    fn sort_student_by_name_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 16);
        let kyoko = Student::new("Kyoko", "Ito", 16);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 16);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(
            sort_by(
                &mut x,
                // まずlast_nameを比較する
                &|a, b| a
                    .last_name
                    .cmp(&b.last_name)
                    // もしlast_nameが等しくない（LessまたはGreater）ならそれを返す
                    // last_nameが等しい（Equal）ならfirst_nameを比較する
                    .then_with(|| a.first_name.cmp(&b.first_name))
            ),
            Ok(())
        );
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_u32_large() {
        {
            // 乱数で65,536要素のデータ列を作る（65,536は2の16乗）
            let mut x = new_u32_vec(65536);
            // 昇順にソートする
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            assert!(is_sorted_descending(&x));
        }
    }
}
