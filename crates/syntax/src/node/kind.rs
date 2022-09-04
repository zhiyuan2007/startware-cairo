// Autogenerated file. To regenerate, please run `cargo run --bin generate_syntax`.
use core::fmt;
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum SyntaxKind {
    Terminal,
    TriviumSkippedTerminal,
    Trivia,
    StructArgExpr,
    OptionStructArgExprEmpty,
    StructArgSingle,
    StructArgTail,
    StructArgList,
    ArgListBraced,
    ExprList,
    ExprMissing,
    OptionGenericArgsEmpty,
    OptionGenericArgsSome,
    PathSegment,
    ExprPath,
    ExprLiteral,
    ExprParenthesized,
    ExprUnary,
    ExprBinary,
    ExprTuple,
    ExprListParenthesized,
    ExprFunctionCall,
    ExprStructCtorCall,
    ExprBlock,
    MatchArm,
    MatchArms,
    ExprMatch,
    TypeClause,
    NonOptionTypeClauseMissing,
    OptionTypeClauseEmpty,
    ReturnTypeClause,
    OptionReturnTypeClauseEmpty,
    StatementList,
    StatementMissing,
    StatementLet,
    OptionSemicolonEmpty,
    StatementExpr,
    StatementReturn,
    Param,
    ParamList,
    ParamListParenthesized,
    ParamListBraced,
    FunctionSignature,
    ItemList,
    ItemModule,
    ItemFreeFunction,
    ItemExternFunction,
    ItemExternType,
    ItemTrait,
    ItemImpl,
    ItemStruct,
    ItemEnum,
    ItemUse,
    SyntaxFile,
}
impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
