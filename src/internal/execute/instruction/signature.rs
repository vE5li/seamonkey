#[derive(Clone)]
pub enum Signature {
    Map,
    List,
    Path,
    String,
    Identifier,
    Keyword,
    Float,
    Integer,
    Character,
    Boolean,
    Type,
    Length,
    Random,
    Time,
    Input,
    Shell,
    Terminate,
    Return,
    Remember,
    Fuze,
    Range,
    Fill,
    FillBack,
    Print,
    PrintLine,
    Error,
    Ensure,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Logarithm,
    Power,
    SquareRoot,
    Negate,
    Absolute,
    Ceiling,
    Round,
    Floor,
    Sine,
    Cosine,
    Tangent,
    Not,
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
    Empty,
    Flip,
    Join,
    Split,
    Uppercase,
    Lowercase,
    Insert,
    Overwrite,
    Move,
    Push,
    Append,
    Remove,
    System,
    Silent,
    Keys,
    Values,
    Pairs,
    Serialize,
    Deserialize,
    ReadFile,
    WriteFile,
    ReadMap,
    WriteMap,
    ReadList,
    WriteList,
    Modify,
    Call,
    CallList,
    Invoke,
    CompileFile,
    CompileString,
    CompileModule,
    Pass,
    NewPass,
    Merge,
    Slice,
    Index,
    Resolve,
    Replace,
    Position,
    Iterate,
    For,
    If,
    While,
    Else,
    End,
    Break,
    Continue,
    Tokenize,
    Parse,
    Build,
}