#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub enum PointRelationX {
    LeftOfPoint,
    RightOfPoint,
    OnPointX,
}

pub enum PointRelationY {
    AbovePoint,
    BelowPoint,
    OnPointY,
}

pub enum PointEquality {
    Equal,
    NotEqual,
}


impl Point {
    pub fn offset_x(&self, offset: i32) -> Point {
        Point {
            x: self.x + offset,
            y: self.y,
        }
    }
    pub fn offset_y(&self, offset: i32) -> Point {
        Point {
            x: self.x,
            y: self.y + offset,
        }
    }
    pub fn offset(&self, offset: Point) -> Point {
        Point {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }

    pub fn compare_x(&self, point: Point) -> PointRelationX {
        if self.x > point.x {
            PointRelationX::RightOfPoint
        } else if self.x < point.x {
            PointRelationX::LeftOfPoint
        } else {
            PointRelationX::OnPointX
        }
    }

    pub fn compare_y(&self, point: Point) -> PointRelationY {
        if self.y > point.y {
            PointRelationY::BelowPoint
        } else if self.y < point.y {
            PointRelationY::AbovePoint
        } else {
            PointRelationY::OnPointY
        }
    }

    pub fn compare(&self, point: Point) -> PointEquality {
        if self.x == point.x && self.y == point.y {
            PointEquality::Equal
        } else {
            PointEquality::NotEqual
        }
    }
}

pub enum Contains {
    DoesContain,
    DoesNotContain,
}

#[derive(Debug, Copy, Clone)]
pub struct Bound {
    pub min: Point,
    pub max: Point,
}

impl Bound {
    pub fn contains(&self, point: Point) -> Contains {
        if point.x >= self.min.x && point.x <= self.max.x && point.y >= self.min.y &&
           point.y <= self.max.y {
            Contains::DoesContain
        } else {
            Contains::DoesNotContain
        }
    }
}
