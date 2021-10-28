/*
    問題: 応答者、マネージャー、ディレクター、3段階のレベルの従業員がいるコールセンターをイメージする。まず問い合わせが来たら手の空いている応答者につなぐ。応答者で対応できない場合はマネージャにつなぐ。マネージャが忙しい場合や対応しきれない場合はディレクターにつなぐ。このような状況についてクラスとデータ構造を設計する。最初につなげる従業員に問い合わせを割り当てるメソッドdispatchCall()も実装する。
*/


// 業務ドメインと支援/汎用サブドメインがある．
// ドメインモデル貧血症（『ユースケース駆動開発実践ガイド』p.270）を避ける．データだけのモデルは×．
// → Callは，クラス的な扱いではなく，構造体（値オブジェクトあたりの話だったような気がする）で処理した方がよさそう．

// 振る舞いが異なるものはそれぞれ別のモデルにする．
// 共通部分はトレイトで抽出する．
// → 従業員トレイトを作って，応答者，マネージャー，ディレクターがそれぞれ実装するようにした方がよいっぽい．

// p.365
use std::collections::VecDeque;
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone, Debug)]
enum Rank {
    Respondent,
    Manager,
    Director,
}

// 応答者，マネージャー，ディレクターは未実装．
trait Employee {
    fn receive_call(&mut self, call: &Call);
    fn complete_call(&mut self, call: &mut Call);
    fn escalate(&mut self, call: &mut Call);
    fn is_free(&self) -> bool;
    fn get_rank(&self) -> Rank;
}

#[derive(Copy, Clone, Debug)]
struct Call {
    rank: Rank,
    completed: bool,
}

// コールをハンドルするというよりは，対応者（ハンドラー）を呼び出すということのよう．
// トレイトオブジェクトの使用例．
// トレイト型への参照: トレイトオブジェクト → p.230
// まさに，「色々な型が混ざる場合はトレイトオブジェクト」の例．

// 内部可変性:  (1)https://qiita.com/wada314/items/24249418983312795c08
//             (2)https://doc.rust-lang.org/std/cell/
/* (2)より
 there are occasions when interior mutability might be appropriate, or even must be used, e.g.

Introducing mutability 'inside' of something immutable
Implementation details of logically-immutable methods.
Mutating implementations of Clone.
*/

#[allow(dead_code)]
struct CallHandler {
    call_queue: VecDeque<Call>,
    handler_list: Vec<Box<RefCell<dyn Employee>>>,
}

#[allow(dead_code)]
impl CallHandler 
{
    fn new() -> Self {
        CallHandler {
            call_queue: VecDeque::<Call>::new(),
            handler_list: Vec::<Box<RefCell<dyn Employee>>>::new(),
        }
    }
    fn get_handler_for_call(&self, call: &Call) -> Option<&Box<RefCell<dyn Employee>>> {
        for e in &self.handler_list {
            if e.borrow().is_free() && call.rank == e.borrow().get_rank() {
                return Some(e)
            }
        }
        None
    }
    // 参照: p.93
    // 複数読み出しか単一書き込みルール．
    // 参照は，所有権のないポインタ．(p.94)
    fn dispatch_call(&self, call: &Call, handler: &Box<RefCell<dyn Employee>>) {
        handler.borrow_mut().receive_call(&call)
    }
    fn receive_call(&mut self, rank: Rank) -> bool {
        let call = Call {
            rank: rank,
            completed: false,
        };
        // assign_callと同じくif letの方が短い．matchで書く場合はこう．
        let handler = self.get_handler_for_call(&call);
        match handler {
            Some(handler) => self.dispatch_call(&call, handler),
            None => {
                self.call_queue.push_back(call);
                return false
            },
        }
        true
    }
    fn assign_call(&mut self) -> bool {
        if let Some(call) = self.call_queue.pop_front() {
            if let Some(handler) = self.get_handler_for_call(&call) {
                self.dispatch_call(&call, &handler);
                return true
            }
        }
        false
    }
}

fn main() {}
