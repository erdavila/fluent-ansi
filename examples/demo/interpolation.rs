use crate::RGBf64;

pub fn from(start: impl Into<f64>, end: impl Into<f64>) -> From {
    let start = start.into();
    let end = end.into();
    From(Range { start, end })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Range<T> {
    start: T,
    end: T,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct From(Range<f64>);

impl From {
    pub fn to(self, start: impl Into<f64>, end: impl Into<f64>) -> Ranges<f64> {
        let start = start.into();
        let end = end.into();
        Ranges {
            from: self.0,
            to: Range { start, end },
        }
    }

    pub fn to_color(self, start: impl Into<RGBf64>, end: impl Into<RGBf64>) -> Ranges<RGBf64> {
        let start = start.into();
        let end = end.into();
        Ranges {
            from: self.0,
            to: Range { start, end },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ranges<T> {
    from: Range<f64>,
    to: Range<T>,
}

impl Ranges<f64> {
    pub fn interpolate(self, n: impl Into<f64>) -> f64 {
        let n = n.into();
        self.to.start
            + (self.to.end - self.to.start) * (n - self.from.start)
                / (self.from.end - self.from.start)
    }
}

impl Ranges<RGBf64> {
    pub fn interpolate(self, n: impl Into<f64>) -> RGBf64 {
        let n = n.into();

        let r = self
            .from()
            .to(self.to.start.r, self.to.end.r)
            .interpolate(n);
        let g = self
            .from()
            .to(self.to.start.g, self.to.end.g)
            .interpolate(n);
        let b = self
            .from()
            .to(self.to.start.b, self.to.end.b)
            .interpolate(n);

        RGBf64 { r, g, b }
    }

    fn from(self) -> From {
        From(self.from)
    }
}
