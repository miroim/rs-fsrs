pub mod utils;

use std::{
    ops::{Add, Mul, Sub},
    vec,
};

#[cfg(test)]
use fsrs::{FuzzRange, Parameters, Rating};

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

#[test]
fn test_next_interval_max_limit() {
    let mut params = Parameters {
        maximum_interval: 365,
        ..Default::default()
    };
    let interval_modifier =
        (f64::powf(params.request_retention as f64, 1.0 / params.decay) - 1.0) / params.factor;
    let stability = 737.47;
    let next_interval = params.next_interval(stability, 0);
    let test_fuzz = 98;

    assert_eq!(next_interval, params.maximum_interval as f64);

    params.enable_fuzz = true;
    let next_interval_fuzz = params.next_interval(stability, test_fuzz);
    let (min_interval, max_interval) = FuzzRange::get_fuzz_range(
        stability * interval_modifier,
        test_fuzz,
        params.maximum_interval,
    );

    assert!(next_interval_fuzz >= min_interval as f64);
    assert!(next_interval_fuzz <= max_interval as f64);
    assert_eq!(max_interval, params.maximum_interval as i64);
}

#[test]
fn test_next_difficulty() {
    const TEST_WEIGHT: [f64; 19] = [
        0.4072, 1.1829, 3.1262, 15.4722, 7.2102, 0.5316, 1.0651, 0.0234, 1.616, 0.1544, 1.0824,
        1.9813, 0.0953, 0.2975, 2.2042, 0.2407, 2.9466, 0.5034, 0.6567,
    ];
    let params = Parameters {
        w: TEST_WEIGHT,
        ..Default::default()
    };

    fn next_d(d: f64, rating: Rating) -> f64 {
        let params = Parameters {
            w: TEST_WEIGHT,
            ..Default::default()
        };

        fn mean_reversion(init_d: f64, current: f64) -> f64 {
            let params = Parameters {
                w: TEST_WEIGHT,
                ..Default::default()
            };
            let f1 = params.w[7] * init_d;
            let f2 = (1.0 - params.w[7]) * current;

            f1 + f2
        }

        fn init_difficulty(rating: Rating) -> f64 {
            let params = Parameters {
                w: TEST_WEIGHT,
                ..Default::default()
            };
            params.w[4] - f64::exp(params.w[5] * ((rating as i64 as f64) - 1.0)) + 1.0
        }

        let next_d = d - (params.w[6] * (rating as i64 as f64 - 3.0));

        mean_reversion(init_difficulty(Rating::Easy), next_d).clamp(1.0, 10.0)
    }

    let mut difficulty_history = vec![];
    let mut expect_difficulty = vec![];

    for &rating in Rating::iter() {
        let difficulty = params.next_difficulty(5.0, rating);
        let expect = next_d(5.0, rating);
        difficulty_history.push(utils::round_float(difficulty, 8));
        expect_difficulty.push(utils::round_float(expect, 8));
    }
    assert_eq!(
        difficulty_history,
        [7.04017216, 5.9999955, 4.95981884, 3.91964218]
    );
    assert_eq!(difficulty_history, expect_difficulty);
}

#[test]
fn test_next_stability() {
    const TEST_WEIGHT: [f64; 19] = [
        0.4072, 1.1829, 3.1262, 15.4722, 7.2102, 0.5316, 1.0651, 0.0234, 1.616, 0.1544, 1.0824,
        1.9813, 0.0953, 0.2975, 2.2042, 0.2407, 2.9466, 0.5034, 0.6567,
    ];

    fn next_forget_stability(d: f64, s: f64, r: f64) -> f64 {
        let params = Parameters {
            w: TEST_WEIGHT,
            ..Default::default()
        };

        params.w[11]
            .mul(d.powf(-params.w[12]))
            .mul((s + 1.0).powf(params.w[13]).sub(1.0))
            .mul(f64::exp((1.0 - r) * params.w[14]))
    }

    fn next_recall_stability(d: f64, s: f64, retrievability: f64, rating: Rating) -> f64 {
        let params = Parameters {
            w: TEST_WEIGHT,
            ..Default::default()
        };
        let modifier = match rating {
            Rating::Hard => params.w[15],
            Rating::Easy => params.w[16],
            _ => 1.0,
        };

        s.mul(
            1.0.add(
                params.w[8]
                    .exp()
                    .mul(11.0.sub(d))
                    .mul(s.powf(-params.w[9]))
                    .mul(params.w[10].mul(1.0.sub(retrievability)).exp().sub(1.0))
                    .mul(modifier),
            ),
        )
    }

    fn next_short_term_stability(s: f64, rating: Rating) -> f64 {
        let params = Parameters {
            w: TEST_WEIGHT,
            ..Default::default()
        };

        s.mul(
            params.w[17]
                .mul((rating as i64 as f64).sub(3.0).add(params.w[18]))
                .exp(),
        )
    }

    fn next_s(d: f64, s: f64, retrievability: f64, rating: Rating) -> f64 {
        match rating {
            Rating::Again => next_forget_stability(d, s, retrievability),
            _ => next_recall_stability(d, s, retrievability, rating),
        }
    }

    let mut s_recall_history = vec![];
    let mut s_forget_history = vec![];
    let mut s_short_history = vec![];
    let mut next_s_history = vec![];

    let mut expect_s_recall_history = vec![];
    let mut expect_s_forget_history = vec![];
    let mut expect_s_short_history = vec![];
    let mut expect_next_s_history = vec![];

    let test_s = [5.0; 4];
    let test_d = [1.0, 2.0, 3.0, 4.0];
    let test_retrievability = [0.9, 0.8, 0.7, 0.6];
    let params = Parameters {
        w: TEST_WEIGHT,
        ..Default::default()
    };

    for (i, &rating) in Rating::iter().enumerate() {
        let s_recall =
            params.next_recall_stability(test_d[i], test_s[i], test_retrievability[i], rating);
        let s_forget = params.next_forget_stability(test_d[i], test_s[i], test_retrievability[i]);
        let s_short = params.short_term_stability(test_s[i], rating);

        let expect_s_recall =
            next_recall_stability(test_d[i], test_s[i], test_retrievability[i], rating);
        let expect_s_forget = next_forget_stability(test_d[i], test_s[i], test_retrievability[i]);
        let expect_s_short = next_short_term_stability(test_s[i], rating);
        let expect_next_s = next_s(test_d[i], test_s[i], test_retrievability[i], rating);
        s_recall_history.push(utils::round_float(s_recall, 8));
        s_forget_history.push(utils::round_float(s_forget, 8));
        s_short_history.push(utils::round_float(s_short, 8));

        expect_s_recall_history.push(utils::round_float(expect_s_recall, 8));
        expect_s_forget_history.push(utils::round_float(expect_s_forget, 8));
        expect_s_short_history.push(utils::round_float(expect_s_short, 8));

        match rating {
            Rating::Again => next_s_history.push(utils::round_float(s_forget, 8)),
            _ => next_s_history.push(utils::round_float(s_recall, 8)),
        }
        expect_next_s_history.push(utils::round_float(expect_next_s, 8));
    }

    assert_eq!(
        s_recall_history,
        [27.43740902, 15.27687386, 65.24019626, 224.35058851,]
    );
    assert_eq!(s_recall_history, expect_s_recall_history);
    assert_eq!(
        s_forget_history,
        [1.73909651, 2.0293769, 2.43393181, 2.95208552,]
    );
    assert_eq!(
        s_short_history,
        [2.54268521, 4.20645686, 6.95889497, 11.51235369,]
    );
    assert_eq!(s_short_history, expect_s_short_history);
    assert_eq!(
        next_s_history,
        [
            s_forget_history[0],
            s_recall_history[1],
            s_recall_history[2],
            s_recall_history[3],
        ]
    );
    assert_eq!(next_s_history, expect_next_s_history);
}
