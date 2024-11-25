
//構造体でのpublic
fn main() {
mod back_of_house {
    pub struct Breakfast {//structの構造
        pub toast: String,
        seasonal_fruit: String,//ここはpublic出ないので、参照できず、公開されない
    }//publicにしたものすべてに"pub"をつける必要がある

    impl Breakfast {//"impl"はmod上で動作する関数のこと　詳しくはhttps://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/0e7a37を参照
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // 下の行のコメントを外すとコンパイルできない。食事についてくる
    // 季節のフルーツを知ることも修正することも許されていないので
    // meal.seasonal_fruit = String::from("blueberries");
}
}

