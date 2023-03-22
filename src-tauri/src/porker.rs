//! ポーカーモジュール 個々の役を判定する関数は，事前にrankの順にソートされていることを前提としています．

// anyhow入れてみたはいいものの，あまり使い方がわからない
use anyhow::anyhow;
use num_derive::FromPrimitive;
use rand::{thread_rng, Rng};
use rustc_hash::FxHashMap;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::thread;

//mod test;

///カード1枚のデータを保持する構造体です．ID,スート(記号), ランク(数字)からできています．
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Card {
    pub id: u32,
    pub suit: Suit,
    pub rank: u32,
}

/// 記号情報を保持する列挙型です． NumクレートのFromPrimitivを活用することにより，u32型をSuit型に変換する機能を提供しています．
/// num::FromPrimitive::from_u32(<u32>).unwrap() でu32型からSuit型に変換できます．
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(u32)]
pub enum Suit {
    #[default]
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(PartialEq, Eq)]
pub enum Role {
    NoPair,
    OnePair,
    TwoPair,
    ThreeCard,
    Straight,
    Flush,
    FullHouse,
    FourCard,
    StraightFlush,
    RoyalStraightFlush,
}

impl Card {
    ///IDを渡すことで，スートとランクを計算し，Card型を生成します．
    pub fn new<T>(id: T) -> Self
    where
        T: TryInto<u32>,
        <T as std::convert::TryInto<u32>>::Error: std::fmt::Debug,
    {
        let id = id.try_into().expect("Error can't convert to u32");
        let suit = num::FromPrimitive::from_u32(id / 13).unwrap();
        let rank = (id % 13) + 1;
        Self { id, suit, rank }
    }

    /// デバッグ用に52枚すべてのカードidをもったベクタを返します．
    #[allow(unused)]
    pub fn all_cards_id() -> Vec<u32> {
        let mut cards = Vec::new();

        for i in 0..52 {
            cards.push(i);
        }

        cards
    }
}

type PorkerResult<T> = anyhow::Result<T>;

///手札のカードのid配列を読み込んでCard型配列に変換します．
pub fn make_cards_from_id(cards_id: &[u32; 5]) -> [Card; 5] {
    let mut cards = [Card {
        id: 0,
        suit: num::FromPrimitive::from_u32(0).unwrap(),
        rank: 0,
    }; 5];

    for (i, v) in cards_id.iter().enumerate() {
        cards[i] = Card::new(*v);
    }

    cards
}

/// 使用するカードのID一覧を持つベクタから，ランダムに選んだ5枚で手札ID配列を生成します
pub fn handout_cards(use_cards: &Arc<Vec<u32>>) -> PorkerResult<[u32; 5]> {
    // 重複回避のためにハッシュマップを使用しています

    let mut handout_hash = FxHashMap::default();
    let mut rng = thread_rng();

    // entryを使って重複をしないようにデータを挿入
    // 重複しないカードが5枚未満の場合，エラーとなる
    let mut count = 0;
    while handout_hash.iter().len() < 5 {
        let len = handout_hash.len();
        let num = rng.gen_range(0..use_cards.len());

        //entryを使うのとif let 使うの あまり速度変わらなかった
        //見やすいentryを実行している
        handout_hash.entry(use_cards[num]).or_insert(len);
        /*
        if let None = handout_hash.get(&use_cards[num]){
            handout_hash.insert(use_cards[num], len);
        }
        */

        count += 1;

        if count > 20 {
            return Err(anyhow!("Error: Invalid useCards"));
        }
    }

    //5枚だとわかっているので，ハッシュマップの値を配列に変換
    let mut handout_id: [u32; 5] = [0; 5];

    for (key, value) in handout_hash.iter() {
        handout_id[*value] = *key;
    }

    Ok(handout_id)
}

/// 同じランクのカードが何枚あるかを数え，その枚数に応じたRoleを返します．
// 途中までResult型を返していましたが，設計上のミスだったためOption型を返すようになりました
// 列挙型の使用に伴い大幅な仕様変更がありました. boolを返さず．データが意味を持つようになりました．
pub fn is_pair(cards: &[Card; 5]) -> Option<Role> {
    // 同じランクのカードが何枚あるかを数える．
    let pair_count = (0..5)
        .map(|i| cards.iter().filter(|x| x.rank == cards[i].rank).count())
        .max()
        .unwrap();

    // pair_countが2の場合はツーペアの可能性があるため，処理を分岐しています
    match pair_count {
        4 => Some(Role::FourCard),
        3 => Some(Role::ThreeCard),
        2 => {
            if is_twopair(cards).is_some() {
                Some(Role::TwoPair)
            } else {
                Some(Role::OnePair)
            }
        }
        1 => Some(Role::NoPair),
        _ => None,
    }
}

pub fn is_flush(cards: &[Card; 5]) -> Option<Role> {
    // すべてのスートが同じかどうか
    if cards.iter().all(|x| x.suit == cards[0].suit) {
        Some(Role::Flush)
    } else {
        None
    }
}

pub fn is_strait(cards: &mut [Card; 5]) -> Option<Role> {
    // すべてのランクが連続しているかどうか

    // エースハイストレートの場合は，1, 10, 11, 12, 13となる．
    const ACE_HIGH: [u32; 5] = [1, 10, 11, 12, 13];

    let is_strait_1 = cards.iter().zip(ACE_HIGH.iter()).all(|(a, b)| a.rank == *b);
    let is_strait_2 = (0..4).all(|i| cards[i].rank + 1 == cards[i + 1].rank);

    if is_strait_1 || is_strait_2 {
        Some(Role::Straight)
    } else {
        None
    }
}

pub fn is_royalflush(cards: &[Card; 5]) -> Option<Role> {
    is_flush(cards)?;

    let mut is_royalflush = Some(Role::RoyalStraightFlush);
    for i in cards.iter().take(5) {
        if i.rank < 10 && i.rank > 1 {
            is_royalflush = None;
        }
    }

    is_royalflush
}

pub fn is_straitflush(cards: &mut [Card; 5]) -> Option<Role> {
    if is_strait(cards).is_some() && is_flush(cards).is_some() {
        Some(Role::StraightFlush)
    } else {
        None
    }
}

pub fn is_twopair(cards: &[Card; 5]) -> Option<Role> {
    let mut count1 = false;
    let mut counted = 999;

    'outer: for i in 0..5 {
        counted = cards[i].rank;

        for j in i + 1..5 {
            if cards[i].rank == cards[j].rank {
                count1 = true;
                break 'outer;
            }
        }
    }

    if !count1 {
        return None;
    }

    let mut count2 = false;
    'outer: for i in 0..5 {
        if cards[i].rank == counted {
            continue;
        }

        for j in i + 1..5 {
            if cards[i].rank == cards[j].rank {
                count2 = true;
                break 'outer;
            }
        }
    }

    if count1 && count2 {
        Some(Role::TwoPair)
    } else {
        None
    }
}

pub fn is_fulhouse(cards: &mut [Card; 5]) -> Option<Role> {
    //  rankをキーにソートされているならば2パターンしかありません

    let is_fulhouse_1 = 
        //cards[0].rank == cards[0].rank&&
        cards[1].rank == cards[0].rank
        //&& cards[2].rank == cards[2].rank
        && cards[3].rank == cards[2].rank
        && cards[4].rank == cards[2].rank
    ;

    let is_fulhouse_2 = 
        //cards[0].rank == cards[0].rank&&
        cards[1].rank == cards[0].rank
        && cards[2].rank == cards[0].rank
        //&& cards[3].rank == cards[3].rank
        && cards[4].rank == cards[3].rank
    ;

    if is_fulhouse_1 || is_fulhouse_2 {
        Some(Role::FullHouse)
    } else {
        None
    }
}

/// 役判定を行います.
pub fn count_judge_role(cards: &mut [Card; 5], role_count: &mut [u32; 10]) {
    // 事前にカード配列をソートしておく
    // カード配列をrankをキーにソート． 安定ソートである必要はないため，unstable で不安定ソートを使うことにより高速化
    cards.sort_unstable_by(|a, b| a.rank.cmp(&b.rank));

    if is_royalflush(cards).is_some() {
        role_count[9] += 1;
    } else if is_straitflush(cards).is_some() {
        role_count[8] += 1;
    } else if is_strait(cards).is_some() {
        role_count[7] += 1;
    } else if is_flush(cards).is_some() {
        role_count[6] += 1;
    } else if is_fulhouse(cards).is_some() {
        role_count[5] += 1;
    } else {
        match is_pair(cards) {
            Some(Role::FourCard) => role_count[4] += 1,
            Some(Role::ThreeCard) => role_count[3] += 1,
            Some(Role::TwoPair) => role_count[2] += 1,
            Some(Role::OnePair) => role_count[1] += 1,
            Some(Role::NoPair) => role_count[0] += 1,
            _ => (),
        }
    }
}

/// デバッグ用に，それぞれの役が出る確率を計算して表示します．
pub fn debug_judge_role(role_count: &[u32; 10], total_num_of_atempt: u32) {
    let roles = [
        "ノーペア",
        "ワンペア",
        "ツーペア",
        "スリーカード",
        "フォーカード",
        "フルハウス",
        "フラッシュ",
        "ストレート",
        "ストレートフラッシュ",
        "ロイヤルストレートフラッシュ",
    ];
    let mut rate = [0.; 10];

    for i in 0..10 {
        rate[i] = role_count[i] as f64 / total_num_of_atempt as f64;
        println!("{:<20}: {:.5}%", roles[i], rate[i] * 100.);
    }
    println!();
}

/// 必要な処理がひとまとめになった関数です．
/// 回数制限，手札選び，役判定，指定回数ループ，スコア計算
/// 事実上，pubキーワードはこの関数にのみついていれば問題ありません

#[derive(serde::Serialize)]
pub struct ResultData {
    pub score: u32,
    pub total_num_of_atempt: u32,
    pub role_count: [u32; 10],
}

pub fn million_porker_parallel<T>(use_cards: Arc<Vec<u32>>, num: T) -> PorkerResult<ResultData>
where
    T: TryInto<u32>,
    <T as std::convert::TryInto<u32>>::Error: std::fmt::Debug,
{
    let thread_num: usize = num_cpus::get();

    let num: u32 = match num.try_into() {
        Ok(n) => n,
        Err(e) => {
            return Err(anyhow!("{:?}", e));
        }
    };

    //ループ回数が100万回を超えていたら，100万回まで減らす
    let loop_num = if num > 10_000_000 { 10_000_000 } else { num };

    let role_counter = Arc::new(Mutex::new([0; 10]));
    //let use_cards = Arc::new(use_cards);
    let mut handles = vec![];

    let d = loop_num / (thread_num - 1) as u32;
    let left = loop_num % (thread_num - 1) as u32;

    println!("{} : {}", d, left);

    let mut v = vec![];
    for i in 0..thread_num {
        if i == thread_num - 1 {
            v.push(0..left);
        } else {
            v.push(0..d);
        }
    }

    println!("{:?}", v[0]);
    // 720

    for i in v {
        let role_counter_shere = Arc::clone(&role_counter);
        let use_cards = Arc::clone(&use_cards);
        let handle = thread::spawn(move || {
            //let mut num = role_counter.lock().unwrap();
            let mut role_counter_exclusiv = [0; 10];

            for _ in i {
                //let mut role_counter = role_counter;

                //カードをランダムに5枚選び出す（idのみ）
                let use_cards = handout_cards(&use_cards).unwrap();

                //idからCard型を生成する
                let mut cards = make_cards_from_id(&use_cards);
                // 役判定を行う

                //let mut a = role_counter;
                //println!("{:?}", a);

                count_judge_role(&mut cards, &mut role_counter_exclusiv);

            }
            
            role_counter_shere.lock().unwrap().iter_mut().zip(role_counter_exclusiv.iter()).for_each(|(a, b)| *a += *b);

            // println!("{:?}", role_counter_shere.lock().unwrap());

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let score = calc_score(&role_counter);

    let role_count = *role_counter.lock().unwrap();

    // Ok(ResultData { role_count,  score, total_num_of_atempt: loop_num })
    Ok(ResultData {
        total_num_of_atempt: loop_num,
        score,
        role_count,
    })
}

/// 総スコアを計算します．
pub fn calc_score(role_count: &Arc<Mutex<[u32; 10]>>) -> u32 {
    /*indexの小さい順に
        ノーペア,
        ワンペア,
        ツーペア,
        スリーカード,
        フォーカード,
        フルハウス,
        フラッシュ,
        ストレート,
        ストレートフラッシュ,
        ロイヤルストレートフラッシュ,
    */
    const SCORE_SHEET: [u32; 10] = [1, 5, 10, 20, 100, 150, 200, 500, 800, 1500];
    let role_count = role_count.lock().unwrap();

    let sum_score: u32 = role_count
        .iter()
        .zip(SCORE_SHEET.iter())
        .map(|x| x.0 * x.1)
        .sum();

    sum_score
}
