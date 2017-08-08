pub enum ExprKind {
    InPlace(P<Expr>, P<Expr>),
    Array(Vec<P<Expr>>),
    Call(P<Expr>, Vec<P<Expr>>),
    MethodCall(PathSegment, Vec<P<Expr>>),
    Tup(Vec<P<Expr>>),
    Binary(BinOp, P<Expr>, P<Expr>),
    Atom(P<Lit>),
    Type(P<Expr>, P<Ty>),
    If(P<Expr>, P<Block>, Option<P<Expr>>),
    Block(P<Block>),
    Catch(P<Block>),
    Assign(P<Expr>, P<Expr>),
    Field(P<Expr>, SpannedIdent),
    TupField(P<Expr>, Spanned<usize>),
    Index(P<Expr>, P<Expr>),
    Path(Option<QSelf>, Path),
    Mac(Mac),
    Struct(Path, Vec<Field>, Option<P<Expr>>),
    Repeat(P<Expr>, P<Expr>),
    Paren(P<Expr>),
    Try(P<Expr>),
}


// region of code.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
    pub ctxt: SyntaxContext,
}


// A module decration
#[derive(Clone, PartialEq, Eq, RustcEncodable, RustcDecodable, Hash, Debug)]
pub struct Mod {
    pub inner: Span,
    pub items: Vec<P<Item>>,
}

/// A literal
pub type Lit = Spanned<LitKind>;

#[derive(Clone, PartialEq, Eq, RustcEncodable, RustcDecodable, Hash, Debug, Copy)]
pub enum LitIntType {
    Signed(IntTy),
    Unsigned(UintTy),
    Unsuffixed,
}

#[derive(Clone, PartialEq, Eq, RustcEncodable, RustcDecodable, Hash,)]
pub struct Expr {
    pub id: NodeId,
    pub node: ExprKind,
    pub attrs: ThinVec<Attribute>
}

