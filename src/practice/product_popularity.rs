use std::collections::HashMap;

pub fn example() {
    let mut products = HashMap::new();

    products.insert("product 1", vec![1, 2, 2, 3]);
    products.insert("product 2", vec![4, 5, 6, 3, 4]);
    products.insert("product 3", vec![8, 8, 7, 6, 5, 4, 4, 1]);

    for (product_id, popularity) in products {
        if populatiry_analysis(popularity) {
            println!("{} popularity is increasing or decreasing", product_id);
        } else {
            println!("{} popularity is fluctuating", product_id);
        }
    }
}

fn populatiry_analysis(scores: Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..scores.len() - 1 {
        if scores[i] > scores[i + 1] {
            increasing = false;
        }

        if scores[i] < scores[i + 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}
