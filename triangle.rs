#[derive(PartialEq, Eq)]
enum TriangleType {
    Equilateral,
    Scalene,
    Isosceles,
}

pub struct Triangle {
    category: TriangleType,
}

impl Triangle {
    fn new(category: TriangleType) -> Self {
        Triangle { category: category }
    }

    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let (a, b, c) = (sides[0], sides[1], sides[2]);
        if a <= 0 || b <= 0 || c <= 0 || a + b <= c || b + c <= a || c + a <= b {
            return None;
        }
        let (ab, bc) = (a == b, b == c);
        if ab && bc {
            return Some(Triangle::new(TriangleType::Equilateral));
        }
        let ca = c == a;
        if ab || bc || ca {
            return Some(Triangle::new(TriangleType::Isosceles));
        }
        Some(Triangle::new(TriangleType::Scalene))
    }

    pub fn is_equilateral(&self) -> bool {
        self.category == TriangleType::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.category == TriangleType::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.category == TriangleType::Isosceles
    }
}
