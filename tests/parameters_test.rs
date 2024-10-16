pub mod utils;

#[cfg(test)]
use fsrs::{Parameters, Rating};

#[test]
fn test_forgeting_curve() {
    const TEST_DELTA: [i64; 6] = [0, 1, 2, 3, 4, 5];
    const TEST_STABILITY: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 4.0, 2.0];

    let params = Parameters::default();
    let expect_result = [1.0, 0.946059, 0.9299294, 0.92216794, 0.9, 0.79394596];

    for i in 0..6 {
        let result = f64::powf(
            1.0 + (params.factor * TEST_DELTA[i] as f64 / TEST_STABILITY[i]),
            params.decay,
        );
        let expect = params.forgeting_curve(TEST_DELTA[i], TEST_STABILITY[i]);

        assert_eq!(result, expect);
        assert_eq!(utils::round_float(result, 8), expect_result[i]);
    }
}

#[test]
fn test_init_difficulty() {
    let params = Parameters::default();

    for &rating in Rating::iter() {
        let difficulty = params.init_difficulty(rating);
        let expect = params.w[4] - f64::exp(params.w[5] * ((rating as i64 as f64) - 1.0)) + 1.0;

        assert_eq!(difficulty, expect);
    }
}

#[test]
fn test_init_stability() {
    let params = Parameters::default();

    for (i, &rating) in Rating::iter().enumerate() {
        let stability = params.init_stability(rating);

        assert_eq!(stability, params.w[i]);
    }
}

#[test]
fn test_next_interval() {
    let mut params = Parameters::default();
    let mut interval_results: Vec<f64> = vec![];

    for i in 1..=10 {
        params.request_retention = i as f64 / 10.0;
        interval_results.push(params.next_interval(1.0, 0));
    }

    let expect = [422.0, 102.0, 43.0, 22.0, 13.0, 8.0, 4.0, 2.0, 1.0, 1.0];

    assert_eq!(interval_results, expect);
}
