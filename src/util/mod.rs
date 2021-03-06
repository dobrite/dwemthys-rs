pub enum XPointRelation {
    LeftOfPoint,
    RightOfPoint,
    OnPointX,
}

pub enum YPointRelation {
    AbovePoint,
    BelowPoint,
    OnPointY,
}

pub enum PointEquality {
    PointsEqual,
    PointsNotEqual,
}

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn offset_x(&self, offset: i32) -> Point {
        Point { x: self.x + offset, y : self.y }
    }

    pub fn offset_y(&self, offset: i32) -> Point {
        Point { x: self.x, y : self.y + offset }
    }

    pub fn offset(&self, offset: Point) -> Point {
        Point { x: self.x + offset.x, y : self.y + offset.y }
    }

    pub fn compare_x(&self, point: Point) -> XPointRelation {
        if self.x > point.x {
            RightOfPoint
        } else if self.x < point.x {
            LeftOfPoint
        } else {
            OnPointX
        }
    }

    pub fn compare_y(&self, point: Point) -> YPointRelation {
        if self.y > point.y {
            BelowPoint
        } else if self.y < point.y {
            AbovePoint
        } else {
            OnPointY
        }
    }

    pub fn compare(&self, point: Point) -> PointEquality {
        if self.x == point.x && self.y == point.y {
            PointsEqual
        } else {
            PointsNotEqual
        }
    }
}

pub struct Bound {
    pub min: Point,
    pub max: Point
}

pub enum Contains {
    DoesContain,
    DoesNotContain
}

impl Bound {
    pub fn contains(&self, point: Point) -> Contains {
        if
            point.x >= self.min.x &&
            point.x <= self.max.x &&
            point.y >= self.min.y &&
            point.y <= self.max.y
        {
            DoesContain
        } else {
            DoesNotContain
        }
    }
}
