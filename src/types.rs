use std::{cmp::Ord, collections::BinaryHeap};
use core::cmp::{Ordering};
use std::time::SystemTime;

use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Book {
    pub sell_book: BinaryHeap<SellOrder>,
    pub buy_book: BinaryHeap<BuyOrder>,
}

/// Orderbook, matches orders and handles incoming transactions
impl Book {
    pub fn new() -> Self {
        Self {
            sell_book: BinaryHeap::new(),
            buy_book: BinaryHeap::new(),
        }
    }

    pub fn new_order(&mut self, order_type: OrderType, price: u64, size: i64) {
        match order_type {
            OrderType::BUY => {
                self.buy_book.push(BuyOrder {
                    size,
                    price,
                    timestamp: SystemTime::now().elapsed().unwrap().as_millis(),
                });
            },
            OrderType::SELL => {
                self.sell_book.push(SellOrder {
                    size,
                    price,
                    timestamp: SystemTime::now().elapsed().unwrap().as_millis(),
                });
            }
        }

        self.attempt_match();
    }

    pub fn get_vec(&self) -> (Vec<BuyOrder>, Vec<SellOrder>) {
        (self.buy_book.clone().into_sorted_vec(), self.sell_book.clone().into_sorted_vec())
    }

    fn attempt_match(&mut self) {
        if let Some(highest_buy) = self.buy_book.peek() {
            if let Some(highest_sell) = self.sell_book.peek() {
                
                if highest_sell.price <= highest_buy.price {
                    let remainder = highest_sell.size - highest_buy.size;
                    if remainder == 0 {
                        println!("Matched {:?} against {:?}", highest_buy, highest_sell);
                        self.sell_book.pop();
                        self.buy_book.pop();
                    }
                    else if remainder > 0 {
                        println!("Matched {:?} against {:?}", highest_buy, highest_sell);
                        self.sell_book.peek_mut().unwrap().size -= highest_buy.size;
                        self.buy_book.pop();
                    }
                    else {
                        println!("Matched {:?} against {:?}", highest_buy, highest_sell);
                        self.buy_book.peek_mut().unwrap().size -= highest_sell.size;
                        self.sell_book.pop();
                    }
        
                    self.attempt_match();
                }
            } else { println!("No sell orders") }
        } else { println!("No buy orders") }
    }
}

pub enum OrderType {
    BUY,
    SELL
}

// Sell Order

#[derive(Debug, Clone, Serialize)]
pub struct SellOrder {
    pub size: i64,
    pub price: u64,
    pub timestamp: u128,
}

impl Ord for SellOrder {
    // Lowest is 'best' AKA highest
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.price.cmp(&other.price);
        if a == Ordering::Equal {
            return self.timestamp.cmp(&other.timestamp).reverse()
        }
        a.reverse()
    }
}

impl Eq for SellOrder {

}

impl PartialOrd for SellOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SellOrder {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

// Buy Order

#[derive(Debug, Clone, Serialize)]
pub struct BuyOrder {
    pub size: i64,
    pub price: u64,
    pub timestamp: u128,
}

impl Ord for BuyOrder {
    // Lowest is 'best' AKA highest
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.price.cmp(&other.price);
        if a == Ordering::Equal {
            return self.timestamp.cmp(&other.timestamp).reverse()
        }
        a
    }
}

impl Eq for BuyOrder {

}

impl PartialOrd for BuyOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BuyOrder {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

// The raw json transaction
#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    pub size: i64,
    pub price: u64,
    pub sell: bool,
}