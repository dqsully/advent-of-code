pub fn valid_inputs_for_problem(time: f64, distance: f64) -> i32 {
    let o = (time * time / 4.0 - distance).sqrt();

    let min = (-o + time / 2.0).floor() as i32;
    let max = (o + time / 2.0).ceil() as i32 - 1;

    max - min
}
