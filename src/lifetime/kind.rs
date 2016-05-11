
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Fn,
    Arg,
    Block,
    Expr,
    Add,
    Mul,
    Pow,
    Base,
    Exp,
    Val,
    Call,
    CallArg,
    Assign,
    Left,
    Right,
    Item,
    Return,
    Object,
    Array,
    ArrayItem,
    ArrayFill,
    Fill,
    N,
    KeyValue,
    For,
    ForN,
    Sum,
    Min,
    Max,
    Sift,
    Any,
    All,
    Start,
    End,
    Init,
    Cond,
    ElseIfCond,
    ElseIfBlock,
    Step,
    Compare,
    If,
    TrueBlock,
    ElseBlock,
    Loop,
    Id,
    Break,
    Continue,
    UnOp,
    Vec4,
    X,
    Y,
    Z,
    W,
    Type,
    Arr,
    Opt,
    Res,
    RetType,
    ReturnVoid,
}

impl Kind {
    pub fn new(name: &str) -> Option<Kind> {
        Some(match name {
            "fn" => Kind::Fn,
            "arg" => Kind::Arg,
            "block" => Kind::Block,
            "expr" => Kind::Expr,
            "add" => Kind::Add,
            "mul" => Kind::Mul,
            "pow" => Kind::Pow,
            "base" => Kind::Base,
            "exp" => Kind::Exp,
            "val" => Kind::Val,
            "call" => Kind::Call,
            "call_arg" => Kind::CallArg,
            "named_call" => Kind::Call,
            "assign" => Kind::Assign,
            "left" => Kind::Left,
            "right" => Kind::Right,
            "item" => Kind::Item,
            "return" => Kind::Return,
            "object" => Kind::Object,
            "array" => Kind::Array,
            "array_item" => Kind::ArrayItem,
            "array_fill" => Kind::ArrayFill,
            "fill" => Kind::Fill,
            "n" => Kind::N,
            "key_value" => Kind::KeyValue,
            "for" => Kind::For,
            "for_n" => Kind::ForN,
            "sum" => Kind::Sum,
            "min" => Kind::Min,
            "max" => Kind::Max,
            "sift" => Kind::Sift,
            "start" => Kind::Start,
            "any" => Kind::Any,
            "all" => Kind::All,
            "end" => Kind::End,
            "init" => Kind::Init,
            "cond" => Kind::Cond,
            "else_if_cond" => Kind::ElseIfCond,
            "else_if_block" => Kind::ElseIfBlock,
            "step" => Kind::Step,
            "compare" => Kind::Compare,
            "if" => Kind::If,
            "true_block" => Kind::TrueBlock,
            "else_block" => Kind::ElseBlock,
            "loop" => Kind::Loop,
            "id" => Kind::Id,
            "break" => Kind::Break,
            "continue" => Kind::Continue,
            "unop" => Kind::UnOp,
            "vec4" => Kind::Vec4,
            "x" => Kind::X,
            "y" => Kind::Y,
            "z" => Kind::Z,
            "w" => Kind::W,
            "type" => Kind::Type,
            "arr" => Kind::Arr,
            "opt" => Kind::Opt,
            "res" => Kind::Res,
            "ret_type" => Kind::RetType,
            "return_void" => Kind::ReturnVoid,
            _ => return None
        })
    }

    pub fn is_decl_loop(&self) -> bool {
        match *self {
            Kind::ForN | Kind::Sum | Kind::Min | Kind::Max | Kind::Sift
            | Kind::Any | Kind::All => true,
            _ => false
        }
    }
}
