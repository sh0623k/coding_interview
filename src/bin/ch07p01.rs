/*
enum
p.207
データを持たないヴァリアント，タプル型ヴァリアント，構造体型ヴァリアントを持つことができる．

p.225
代数データ型を表すもの．
ヴァリアント，参照，可変性，メモリ安全性の組み合わせ．
データを正しい形に整形するための設計ツール．
あるものか，別のものか，それとも何もないかというような値に対して，
速度，安全性，コードの小ささ，ドキュメントの書きやすさのすべてにおいて，クラス階層よりも優れている．
制約: 柔軟性．ヴァリアントの追加・拡張はできない．
→ より柔軟性が必要なものは，トレイトを使う．
*/

// 細かいところは実装してないです．

// https://doc.rust-jp.rs/rust-by-example-ja/attribute/unused.html
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

trait Card {
    fn new(suit: Suit, rank: Rank) -> Self;
}


trait Deck<C: Card> {
    fn new(cards: Vec<C>) -> Self;
    fn shuffle(&mut self);
    fn get_remaining_cards(&self) -> usize;
    fn deal_hand(&mut self) -> Vec<C>;
    fn deal_card(&mut self) -> C;
}

trait Hand<C: Card> {
    fn new(cards: Vec<C>) -> Self;
    fn get_score(&self) -> usize;
    fn add_card(&mut self, card: C);
}

#[derive(Copy, Clone, Debug)]
struct BlackJackCard {
    suit: Suit,
    rank: Rank,
    available: bool,
}

impl Card for BlackJackCard {
    fn new(suit: Suit, rank: Rank) -> Self {
        BlackJackCard {
            suit: suit,
            rank: rank,
            available: true,
        }
    }
}

#[allow(dead_code)]
impl BlackJackCard {
    // 未実装．ルールに合わせて実装する．
    // 他の関数も実装する必要がある．
    fn get_value(&self) -> usize {
        let mut value = 0;
        value
    }
}

#[derive(Clone, Debug)]
struct BlackJackHand {
    cards: Vec<BlackJackCard>,
}

impl Hand<BlackJackCard> for BlackJackHand {
    fn new(cards: Vec<BlackJackCard>) -> Self {
        BlackJackHand {
            cards: cards,
        }
    }
    fn get_score(&self) -> usize {
        let mut score = 0;
        // get_valueとかBlackJackCardの実装を使って出す．
        score
    }
    fn add_card(&mut self, card: BlackJackCard) {

    }
}

struct BlackJackDeck {
    cards: Vec<BlackJackCard>,
    dealt_index: usize,
}

impl Deck<BlackJackCard> for BlackJackDeck {
    fn new(cards: Vec<BlackJackCard>) -> Self {
        BlackJackDeck{
            cards: cards,
            dealt_index: 0,
        }
    }
    fn shuffle(&mut self) {

    }
    fn get_remaining_cards(&self) -> usize {
        self.cards.len() - self.dealt_index
    }
    fn deal_hand(&mut self) -> Vec<BlackJackCard> {
        let mut hand: Vec<BlackJackCard> = vec![];
        hand
    }
    fn deal_card(&mut self) -> BlackJackCard {
        self.dealt_index += 1;
        self.cards[self.dealt_index]
    }
}

fn main() {}