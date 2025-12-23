pub trait ASTNode {
    fn solve(&self) -> isize;
}

pub struct ASTLiteral {
    pub value: isize,
}
impl ASTLiteral {
    pub fn new(value: isize) -> ASTLiteral {
        ASTLiteral { value: value }
    }
}
impl ASTNode for ASTLiteral {
    fn solve(&self) -> isize {
        return self.value;
    }
}

pub struct ASTBinary {
    pub left: Box<dyn ASTNode>,
    pub right: Box<dyn ASTNode>,
    pub op: ASTBinaryOp,
}
#[derive(Clone)]
pub enum ASTBinaryOp {
    Plus,
    Minus,
    Mul,
    Div,
    ShiftLeft,
    ShiftRight,
}
impl ASTBinary {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>, op: ASTBinaryOp) -> ASTBinary {
        ASTBinary {
            left: left,
            right: right,
            op,
        }
    }
}
impl ASTNode for ASTBinary {
    fn solve(&self) -> isize {
        match self.op {
            ASTBinaryOp::Mul => self.left.solve() * self.right.solve(),
            ASTBinaryOp::Div => self.left.solve() / self.right.solve(),
            ASTBinaryOp::Plus => self.left.solve() + self.right.solve(),
            ASTBinaryOp::Minus => self.left.solve() - self.right.solve(),
            ASTBinaryOp::ShiftLeft => self.left.solve() << self.right.solve(),
            ASTBinaryOp::ShiftRight => self.left.solve() >> self.right.solve(),
        }
    }
}
pub struct ASTUnary {
    pub right: Box<dyn ASTNode>,
    pub op: ASTUnaryOp,
}
pub enum ASTUnaryOp {
    Minus,
    Not,
}
impl ASTUnary {
    pub fn new(right: Box<dyn ASTNode>, op: ASTUnaryOp) -> ASTUnary {
        ASTUnary { right: right, op }
    }
}
impl ASTNode for ASTUnary {
    fn solve(&self) -> isize {
        match self.op {
            ASTUnaryOp::Minus => -self.right.solve(),
            ASTUnaryOp::Not => !self.right.solve(),
        }
    }
}
