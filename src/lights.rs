use crate::brightness::Brightness;


pub struct PointLight {
    x: i64,
    y: i64,
    radius: u16,
}

impl PointLight {
    pub fn new(x: i64, y: i64, radius: u16) -> PointLight {
        PointLight { x, y, radius }
    }
    
    pub fn get_brightness(&self, x: i64, y: i64) -> Brightness {
        let distance = self.distance(x, y);
        let ratio = distance / self.radius as f64;
        if ratio > 1.0 {
            return Brightness::Black;
        } else if ratio > 0.8 {
            return Brightness::Dark;
        } else if ratio > 0.6 {
            return Brightness::Dim;
        }
        return Brightness::Normal;
    }
    
    #[inline(always)]
    fn distance(&self, x: i64, y: i64) -> f64 {
        f64::sqrt(((self.x - x).pow(2) + (self.y - y).pow(2)) as f64)
    }
}