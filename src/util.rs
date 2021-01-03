fn INT(x: f64) -> f64 {
    return x.floor();
}

fn FIX(x: f64) -> f64 {
    if x >= 0.0 {
        return x.floor();
    } else {
        return x.ceil();
    }
}

fn ABS(x: f64) -> f64 {
    return x.abs();
}

fn FRAC(x: f64) -> f64 {
    return x.fract().abs();
}

fn MOD(x1: f64, x2: f64) -> f64 {
    // return (x1 % x2).abs();
    // return (x1 - (x2 * ((x1 / x2) as i64) as f64)).abs();
    if x1 < 0.0 && x2 > 0.0 {
        return x2 - (x1.abs() % x2);
    } else {
        return x1 % x2;
    }
}

fn ROUND(x: f64) -> f64 {
    return x.round();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn util_INT() {
        assert_eq!(INT(1.5), 1.0);
        assert_eq!(INT(1.4), 1.0);
        assert_eq!(INT(-1.5), -2.0);
        assert_eq!(INT(-1.4), -2.0);
    }

    #[test]
    fn util_FIX() {
        assert_eq!(FIX(1.5), 1.0);
        assert_eq!(FIX(1.4), 1.0);
        assert_eq!(FIX(-1.5), -1.0);
        assert_eq!(FIX(-1.4), -1.0);
    }

    #[test]
    fn util_ABS() {
        assert_eq!(ABS(-5.0), 5.0);
        assert_eq!(ABS(5.4), 5.4);
        assert_eq!(ABS(0.0), 0.0);
    }

    #[test]
    fn util_FRAC() {
        assert_eq!(FRAC(1.5), 0.5);
        assert_eq!(FRAC(-1.5), 0.5);
    }

    #[test]
    fn util_MOD() {
        assert_eq!(MOD(-100.0, 8.0), 4.0);
        assert_eq!(MOD(-400.0, 360.0), 320.0);
        assert_eq!(MOD(270.0, 180.0), 90.0);
        assert_eq!((MOD(-270.8, 180.0) * 10.0).round() / 10.0, 89.2);
        assert_eq!(MOD(390.0, 360.0), 30.0);
        assert_eq!(MOD(390.5, 360.0), 30.5);
    }

    #[test]
    fn util_ROUND() {
        assert_eq!(ROUND(1.4), 1.0);
        assert_eq!(ROUND(1.8), 2.0);
        assert_eq!(ROUND(-1.4), -1.0);
        assert_eq!(ROUND(-1.8), -2.0);
    }
}