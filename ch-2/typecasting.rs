fn main() {
    let (true_p, true_n, false_p, false_n) = (100, 50, 10, 5);
    let total = true_n + true_p + false_n + false_p;
    println!("Total prediction is {}", total);
    println!(
        "Accuracy of the model {:.2}",
        percentage(accuracy(true_p, true_n, total))
    );
}

fn accuracy(tp: i32, tn: i32, total: i32) -> f32 {
    (tp as f32 + tn as f32) / (total as f32)
}

fn percentage(value: f32) -> f32 {
    value * 100.0
}
