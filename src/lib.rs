#![doc = include_str!("../README.md")]

fn pinch(v: f64) -> f64 {
    if v < 0.5 {
        -v * v
    } else {
        v * v
    }
}
/// a generalized, parameterized curve formula for generating lots of different curve shapes useful for easing, etc.
///
/// # Arguments
/// * `x` - input that drives the curve. clamped to 0..=1
/// * `a` - controls the start of the curve
/// * `b` - controls the middle of the curve
/// * `c` - pinches the tail of the curve
///
/// # Example
/// ```rs
/// use torcurve_rs::torcurve;
/// fn run_code() {
///     for i in 0..=10 {
///          println!("curve: torcuve(i * 0.1, 3, 0, 0);
///     }
/// }
/// ```
///
/// note: implemented using <https://jsfiddle.net/torcado194/5ocmt48a/latest> as reference,
/// the js fiddle may be useful to tune the parameters
///
pub fn torcurve(x: f64, a: f64, b: f64, c: f64) -> f64 {
    let c = pinch(c);
    let x = x.clamp(0.0, 1.0); //clamp input to [0-1], behavior is undefined otherwise
    let s = a.exp(); //could be any exponential like 2^a or 3^a, or just linear
    let s2 = 1.0 / s;
    let t = b.clamp(0.0, 1.0);
    let u = c; //should normally be clamped but creates possibly useful results outside of the 0-1 range
    let eps = 0.00001; //protect against div/0

    let (c1, c2, c3) = if x < t {
        // c1 = (t*x)/(x+s*(t-x)+eps);
        let c1 = (t * x) / (x + s * (t - x) + eps);
        //  c2 = t-Math.pow(1/(t+eps), s2-1)*Math.pow(Math.abs(x-t), s2);
        let c2 = t - (1.0 / ((t + eps).powf(s2 - 1.0)) * (x - t).abs().powf(s2));
        // c3 = Math.pow(1/(t+eps), s-1)*Math.pow(x,s);
        let c3 = (1.0 / (t + eps)).powf(s - 1.0) * x.powf(s);
        (c1, c2, c3)
    } else {
        // c1 = (1-t)*(x-1)/(1-x-s*(t-x)+eps)+1;
        let c1 = (1.0 - t) * (x - 1.0) / (1.0 - x - s * (t - x) + eps) + 1.0;
        // c2 = Math.pow(1/((1-t)+eps), s2-1)*Math.pow(Math.abs(x-t), s2)+t;
        let c2 = (1.0 / ((1.0 - t) + eps)).powf(s2 - 1.0) * (x - t).abs().powf(s2) + t;
        // c3 = 1-Math.pow(1/((1-t)+eps), s-1)*Math.pow(1-x,s);
        let c3 = 1.0 - (1.0 / ((1.0 - t) + eps)).powf(s - 1.0) * (1.0 - x).powf(s);
        (c1, c2, c3)
    };
    let res = if u <= 0.0 {
        (-u) * c2 + (1.0 + u) * c1
    } else {
        (u) * c3 + (1.0 - u) * c1
    };
    if res.is_nan() {
        0.0
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::torcurve;

    #[test]
    fn it_works() {
        // Sampled data from the javascript implementation
        let test_samples = vec![
            0.0,
            0.09999800003999922,
            0.19999600007999843,
            0.29999400011999766,
            0.39999200015999686,
            0.5000099998000039,
            0.6000079998400032,
            0.7000059998800024,
            0.8000039999200016,
            0.9000019999600009,
            1.0,
        ];
        for (i, test_item) in test_samples.iter().enumerate() {
            assert_eq!(torcurve(i as f64 * 0.1, 0.0, 0.5, 0.0), *test_item);
        }
    }
}
