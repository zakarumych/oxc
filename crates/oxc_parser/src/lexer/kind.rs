//! ECMAScript Token Kinds

use std::fmt;

use oxc_ast::Atom;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum Kind {
    Undetermined,
    #[default]
    Eof,
    WhiteSpace,
    NewLine,
    Comment,
    MultiLineComment,
    // 12.6 identifier
    Ident,
    // 12.6.2 keyword
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Return,
    Super,
    Switch,
    This,
    Throw,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    // Contextual Keywords
    Async,
    From,
    Get,
    Meta, // import.meta
    Of,
    Set,
    Target, // new.target
    Accessor,
    // TypeScript Contextual Keywords
    Abstract,
    As,
    Asserts,
    Assert,
    Any,
    Boolean,
    Constructor,
    Declare,
    Infer,
    Intrinsic,
    Is,
    KeyOf,
    Module,
    Namespace,
    Never,
    Out,
    Readonly,
    Require,
    Number,
    Object,
    Satisfies,
    String, // the "string" keyword for TypeScript
    Symbol,
    Type,
    Undefined,
    Unique,
    Unknown,
    Global,
    BigInt,
    Override,
    // Future keywords (strict mode reserved words)
    Implements,
    Interface,
    Let,
    Package,
    Private,
    Protected,
    Public,
    Static,
    Yield,
    // 12.7 punctuators
    Amp, // &
    Amp2,
    Amp2Eq,
    AmpEq,
    Bang,
    Caret,
    CaretEq,
    Colon,
    Comma,
    Dot,
    Dot3, // ...
    Eq,
    Eq2,
    Eq3,
    GtEq, // >=
    LAngle,
    LBrack,
    LCurly,
    LParen,
    LtEq, // <=
    Minus,
    Minus2,
    MinusEq,
    Neq,
    Neq2,
    Percent,
    PercentEq,
    Pipe,
    Pipe2,
    Pipe2Eq,
    PipeEq,
    Plus,
    Plus2,
    PlusEq,
    Question,
    Question2,
    Question2Eq,
    QuestionDot,
    RAngle,
    RBrack,
    RCurly,
    RParen,
    Semicolon,
    ShiftLeft,     // <<
    ShiftLeftEq,   // <<=
    ShiftRight,    // >>
    ShiftRight3,   // >>>
    ShiftRight3Eq, // >>>=
    ShiftRightEq,  // >>=
    Slash,
    SlashEq,
    Star,
    Star2,
    Star2Eq,
    StarEq,
    Tilde,
    // arrow function
    Arrow,
    // 12.8.1 Null Literals
    Null,
    // 12.8.2 Boolean Literals
    True,
    False,
    // 12.8.3 Numeric Literals
    Decimal,
    Float,
    Binary,
    Octal,
    Hex,
    // 12.8.4 String Literals
    /// String Type
    Str,
    // 12.8.5 Regular Expression Literals
    RegExp,
    // 12.8.6 Template Literal
    NoSubstitutionTemplate,
    TemplateHead,
    TemplateMiddle,
    TemplateTail,
    // es2022 Private Identifier
    PrivateIdentifier,
    // JSX
    JSXText,
    // Decorator
    At,
}

#[allow(clippy::enum_glob_use)]
use self::Kind::*;

static KW_IS: Atom = Atom::new_inline("is");
static KW_AS: Atom = Atom::new_inline("as");
static KW_DO: Atom = Atom::new_inline("do");
static KW_IF: Atom = Atom::new_inline("if");
static KW_IN: Atom = Atom::new_inline("in");
static KW_OF: Atom = Atom::new_inline("of");
static KW_ANY: Atom = Atom::new_inline("any");
static KW_FOR: Atom = Atom::new_inline("for");
static KW_GET: Atom = Atom::new_inline("get");
static KW_LET: Atom = Atom::new_inline("let");
static KW_NEW: Atom = Atom::new_inline("new");
static KW_OUT: Atom = Atom::new_inline("out");
static KW_SET: Atom = Atom::new_inline("set");
static KW_TRY: Atom = Atom::new_inline("try");
static KW_VAR: Atom = Atom::new_inline("var");
static KW_CASE: Atom = Atom::new_inline("case");
static KW_ELSE: Atom = Atom::new_inline("else");
static KW_ENUM: Atom = Atom::new_inline("enum");
static KW_FROM: Atom = Atom::new_inline("from");
static KW_META: Atom = Atom::new_inline("meta");
static KW_NULL: Atom = Atom::new_inline("null");
static KW_THIS: Atom = Atom::new_inline("this");
static KW_TRUE: Atom = Atom::new_inline("true");
static KW_TYPE: Atom = Atom::new_inline("type");
static KW_VOID: Atom = Atom::new_inline("void");
static KW_WITH: Atom = Atom::new_inline("with");
static KW_ASYNC: Atom = Atom::new_inline("async");
static KW_AWAIT: Atom = Atom::new_inline("await");
static KW_BREAK: Atom = Atom::new_inline("break");
static KW_CATCH: Atom = Atom::new_inline("catch");
static KW_CLASS: Atom = Atom::new_inline("class");
static KW_CONST: Atom = Atom::new_inline("const");
static KW_FALSE: Atom = Atom::new_inline("false");
static KW_INFER: Atom = Atom::new_inline("infer");
static KW_KEYOF: Atom = Atom::new_inline("keyof");
static KW_NEVER: Atom = Atom::new_inline("never");
static KW_SUPER: Atom = Atom::new_inline("super");
static KW_THROW: Atom = Atom::new_inline("throw");
static KW_WHILE: Atom = Atom::new_inline("while");
static KW_YIELD: Atom = Atom::new_inline("yield");
static KW_ASSERT: Atom = Atom::new_inline("assert");
static KW_BIGINT: Atom = Atom::new_inline("bigint");
static KW_DELETE: Atom = Atom::new_inline("delete");
static KW_EXPORT: Atom = Atom::new_inline("export");
static KW_GLOBAL: Atom = Atom::new_inline("global");
static KW_IMPORT: Atom = Atom::new_inline("import");
static KW_MODULE: Atom = Atom::new_inline("module");
static KW_NUMBER: Atom = Atom::new_inline("number");
static KW_OBJECT: Atom = Atom::new_inline("object");
static KW_PUBLIC: Atom = Atom::new_inline("public");
static KW_RETURN: Atom = Atom::new_inline("return");
static KW_STATIC: Atom = Atom::new_inline("static");
static KW_STRING: Atom = Atom::new_inline("string");
static KW_SWITCH: Atom = Atom::new_inline("switch");
static KW_SYMBOL: Atom = Atom::new_inline("symbol");
static KW_TARGET: Atom = Atom::new_inline("target");
static KW_TYPEOF: Atom = Atom::new_inline("typeof");
static KW_UNIQUE: Atom = Atom::new_inline("unique");
static KW_ASSERTS: Atom = Atom::new_inline("asserts");
static KW_BOOLEAN: Atom = Atom::new_inline("boolean");
static KW_DECLARE: Atom = Atom::new_inline("declare");
static KW_DEFAULT: Atom = Atom::new_inline("default");
static KW_EXTENDS: Atom = Atom::new_inline("extends");
static KW_FINALLY: Atom = Atom::new_inline("finally");
static KW_PACKAGE: Atom = Atom::new_inline("package");
static KW_PRIVATE: Atom = Atom::new_inline("private");
static KW_REQUIRE: Atom = Atom::new_inline("require");
static KW_UNKNOWN: Atom = Atom::new_inline("unknown");
static KW_ABSTRACT: Atom = Atom::new_inline("abstract");
static KW_ACCESSOR: Atom = Atom::new_inline("accessor");
static KW_CONTINUE: Atom = Atom::new_inline("continue");
static KW_DEBUGGER: Atom = Atom::new_inline("debugger");
static KW_FUNCTION: Atom = Atom::new_inline("function");
static KW_OVERRIDE: Atom = Atom::new_inline("override");
static KW_READONLY: Atom = Atom::new_inline("readonly");
static KW_INTERFACE: Atom = Atom::new_inline("interface");
static KW_INTRINSIC: Atom = Atom::new_inline("intrinsic");
static KW_NAMESPACE: Atom = Atom::new_inline("namespace");
static KW_PROTECTED: Atom = Atom::new_inline("protected");
static KW_SATISFIES: Atom = Atom::new_inline("satisfies");
static KW_UNDEFINED: Atom = Atom::new_inline("undefined");
static KW_IMPLEMENTS: Atom = Atom::new_inline("implements");
static KW_INSTANCEOF: Atom = Atom::new_inline("instanceof");
static KW_CONSTRUCTOR: Atom = Atom::new_inline("constructor");

impl Kind {
    #[must_use]
    pub fn is_eof(self) -> bool {
        matches!(self, Eof)
    }

    #[must_use]
    pub fn is_trivia(self) -> bool {
        matches!(self, WhiteSpace | NewLine | Comment | MultiLineComment)
    }

    #[must_use]
    pub fn is_number(self) -> bool {
        matches!(self, Float | Decimal | Binary | Octal | Hex)
    }

    #[must_use]
    pub fn matches_number_char(self, c: char) -> bool {
        match self {
            Decimal => c.is_ascii_digit(),
            Binary => matches!(c, '0'..='1'),
            Octal => matches!(c, '0'..='7'),
            Hex => c.is_ascii_hexdigit(),
            _ => unreachable!(),
        }
    }

    /// [Identifiers](https://tc39.es/ecma262/#sec-identifiers)
    /// `IdentifierReference`
    #[must_use]
    pub fn is_identifier_reference(self, r#yield: bool, r#await: bool) -> bool {
        self.is_identifier() || (!r#yield && self == Yield) || (!r#await && self == Await)
    }

    /// `BindingIdentifier`
    #[must_use]
    pub fn is_binding_identifier(self) -> bool {
        self.is_identifier() || matches!(self, Yield | Await)
    }

    /// `LabelIdentifier`
    #[must_use]
    pub fn is_label_identifier(self, r#yield: bool, r#await: bool) -> bool {
        self.is_identifier() || (!r#yield && self == Yield) || (!r#await && self == Await)
    }

    /// Identifier
    /// `IdentifierName` but not `ReservedWord`
    #[must_use]
    pub fn is_identifier(self) -> bool {
        self.is_identifier_name() && !self.is_reserved_keyword()
    }

    /// `IdentifierName`
    #[must_use]
    pub fn is_identifier_name(self) -> bool {
        matches!(self, Ident) || self.is_all_keyword()
    }

    /// Check the succeeding token of a `let` keyword
    // let { a, b } = c, let [a, b] = c, let ident
    #[must_use]
    pub fn is_after_let(self) -> bool {
        self != Self::In && (matches!(self, LCurly | LBrack | Ident) || self.is_all_keyword())
    }

    /// Section 13.2.4 Literals
    /// Literal :
    ///     `NullLiteral`
    ///     `BooleanLiteral`
    ///     `NumericLiteral`
    ///     `StringLiteral`
    #[must_use]
    pub fn is_literal(self) -> bool {
        matches!(self, Null | True | False | Str | RegExp) || self.is_number()
    }

    #[must_use]
    pub fn is_after_await_or_yield(self) -> bool {
        !self.is_binary_operator() && (self.is_literal() || self.is_identifier_name())
    }

    /// Section 13.2.6 Object Initializer
    /// `LiteralPropertyName` :
    ///     `IdentifierName`
    ///     `StringLiteral`
    ///     `NumericLiteral`
    #[must_use]
    pub fn is_literal_property_name(self) -> bool {
        self.is_identifier_name() || self == Str || self.is_number()
    }

    #[must_use]
    pub fn is_variable_declaration(self) -> bool {
        matches!(self, Var | Let | Const)
    }

    /// Section 15.4 Method Definitions
    /// `ClassElementName`[Yield, Await] :
    ///   `PropertyName`[?Yield, ?Await]
    ///   `PrivateIdentifier`
    /// `PropertyName`[Yield, Await] :
    ///   `LiteralPropertyName`
    ///   `ComputedPropertyName`[?Yield, ?Await]
    #[must_use]
    pub fn is_class_element_name_start(self) -> bool {
        self.is_literal_property_name() || matches!(self, LBrack | PrivateIdentifier)
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn is_assignment_operator(self) -> bool {
        matches!(self, Eq | PlusEq | MinusEq | StarEq | SlashEq | PercentEq | ShiftLeftEq | ShiftRightEq
            | ShiftRight3Eq | Pipe2Eq | Amp2Eq | PipeEq | CaretEq | AmpEq | Question2Eq
            | Star2Eq)
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn is_binary_operator(self) -> bool {
        matches!(self, Eq2 | Neq | Eq3 | Neq2 | LAngle | LtEq | RAngle | GtEq | ShiftLeft | ShiftRight
            | ShiftRight3 | Plus | Minus | Star | Slash | Percent | Pipe | Caret | Amp | In
            | Instanceof | Star2)
    }

    #[must_use]
    pub fn is_logical_operator(self) -> bool {
        matches!(self, Pipe2 | Amp2 | Question2)
    }

    #[must_use]
    pub fn is_unary_operator(self) -> bool {
        matches!(self, Minus | Plus | Bang | Tilde | Typeof | Void | Delete)
    }

    #[must_use]
    pub fn is_update_operator(self) -> bool {
        matches!(self, Plus2 | Minus2)
    }

    /// [Keywords and Reserved Words](https://tc39.es/ecma262/#sec-keywords-and-reserved-words)
    #[must_use]
    pub fn is_all_keyword(self) -> bool {
        self.is_reserved_keyword()
            || self.is_contextual_keyword()
            || self.is_strict_mode_contextual_keyword()
            || self.is_future_reserved_keyword()
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn is_reserved_keyword(self) -> bool {
        matches!(self, Await | Break | Case | Catch | Class | Const | Continue | Debugger | Default
            | Delete | Do | Else | Enum | Export | Extends | False | Finally | For | Function | If
            | Import | In | Instanceof | New | Null | Return | Super | Switch | This | Throw
            | True | Try | Typeof | Var | Void | While | With | Yield)
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn is_strict_mode_contextual_keyword(self) -> bool {
        matches!(self, Let | Static | Implements | Interface | Package | Private | Protected | Public)
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn is_contextual_keyword(self) -> bool {
        matches!(self, Async | From | Get | Meta | Of | Set | Target | Accessor | Abstract | As | Asserts
            | Assert | Any | Boolean | Constructor | Declare | Infer | Intrinsic | Is | KeyOf | Module
            | Namespace | Never | Out | Readonly | Require | Number | Object | Satisfies | String
            | Symbol | Type | Undefined | Unique | Unknown | Global | BigInt | Override)
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn is_future_reserved_keyword(self) -> bool {
        matches!(self, Implements | Interface | Package | Private | Protected | Public | Static)
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn is_at_expression(self) -> bool {
        self.is_unary_operator()
            || self.is_update_operator()
            || self.is_reserved_keyword()
            || self.is_literal()
            || matches!(self, Neq | LParen | LBrack | LCurly | LAngle | Dot3
                | Slash | SlashEq | TemplateHead | NoSubstitutionTemplate | PrivateIdentifier | Ident | Async)
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn is_modifier_kind(self) -> bool {
        matches!(self, Abstract | Accessor | Async | Const | Declare | Default
          | Export | In | Out | Public | Private | Protected | Readonly | Static | Override)
    }

    #[must_use]
    pub fn match_keyword(s: &str) -> (Self, Atom) {
        let len = s.len();
        if len == 1 || len >= 12 {
            return (Ident, Atom::new(s));
        }
        Self::match_keyword_impl(s).map_or_else(|| (Ident, Atom::new(s)), |(k, s)| (k, s.clone()))
    }

    fn match_keyword_impl(s: &str) -> Option<(Self, &'static Atom)> {
        match s {
            "as" => Some((As, &KW_AS)),
            "do" => Some((Do, &KW_DO)),
            "if" => Some((If, &KW_IF)),
            "in" => Some((In, &KW_IN)),
            "is" => Some((Is, &KW_IS)),
            "of" => Some((Of, &KW_OF)),

            "any" => Some((Any, &KW_ANY)),
            "for" => Some((For, &KW_FOR)),
            "get" => Some((Get, &KW_GET)),
            "let" => Some((Let, &KW_LET)),
            "new" => Some((New, &KW_NEW)),
            "out" => Some((Out, &KW_OUT)),
            "set" => Some((Set, &KW_SET)),
            "try" => Some((Try, &KW_TRY)),
            "var" => Some((Var, &KW_VAR)),

            "case" => Some((Case, &KW_CASE)),
            "else" => Some((Else, &KW_ELSE)),
            "enum" => Some((Enum, &KW_ENUM)),
            "from" => Some((From, &KW_FROM)),
            "meta" => Some((Meta, &KW_META)),
            "null" => Some((Null, &KW_NULL)),
            "this" => Some((This, &KW_THIS)),
            "true" => Some((True, &KW_TRUE)),
            "type" => Some((Type, &KW_TYPE)),
            "void" => Some((Void, &KW_VOID)),
            "with" => Some((With, &KW_WITH)),

            "async" => Some((Async, &KW_ASYNC)),
            "await" => Some((Await, &KW_AWAIT)),
            "break" => Some((Break, &KW_BREAK)),
            "catch" => Some((Catch, &KW_CATCH)),
            "class" => Some((Class, &KW_CLASS)),
            "const" => Some((Const, &KW_CONST)),
            "false" => Some((False, &KW_FALSE)),
            "infer" => Some((Infer, &KW_INFER)),
            "keyof" => Some((KeyOf, &KW_KEYOF)),
            "never" => Some((Never, &KW_NEVER)),
            "super" => Some((Super, &KW_SUPER)),
            "throw" => Some((Throw, &KW_THROW)),
            "while" => Some((While, &KW_WHILE)),
            "yield" => Some((Yield, &KW_YIELD)),

            "assert" => Some((Assert, &KW_ASSERT)),
            "bigint" => Some((BigInt, &KW_BIGINT)),
            "delete" => Some((Delete, &KW_DELETE)),
            "export" => Some((Export, &KW_EXPORT)),
            "global" => Some((Global, &KW_GLOBAL)),
            "import" => Some((Import, &KW_IMPORT)),
            "module" => Some((Module, &KW_MODULE)),
            "number" => Some((Number, &KW_NUMBER)),
            "object" => Some((Object, &KW_OBJECT)),
            "public" => Some((Public, &KW_PUBLIC)),
            "return" => Some((Return, &KW_RETURN)),
            "static" => Some((Static, &KW_STATIC)),
            "string" => Some((String, &KW_STRING)),
            "switch" => Some((Switch, &KW_SWITCH)),
            "symbol" => Some((Symbol, &KW_SYMBOL)),
            "target" => Some((Target, &KW_TARGET)),
            "typeof" => Some((Typeof, &KW_TYPEOF)),
            "unique" => Some((Unique, &KW_UNIQUE)),

            "asserts" => Some((Asserts, &KW_ASSERTS)),
            "boolean" => Some((Boolean, &KW_BOOLEAN)),
            "declare" => Some((Declare, &KW_DECLARE)),
            "default" => Some((Default, &KW_DEFAULT)),
            "extends" => Some((Extends, &KW_EXTENDS)),
            "finally" => Some((Finally, &KW_FINALLY)),
            "package" => Some((Package, &KW_PACKAGE)),
            "private" => Some((Private, &KW_PRIVATE)),
            "require" => Some((Require, &KW_REQUIRE)),
            "unknown" => Some((Unknown, &KW_UNKNOWN)),

            "abstract" => Some((Abstract, &KW_ABSTRACT)),
            "accessor" => Some((Accessor, &KW_ACCESSOR)),
            "continue" => Some((Continue, &KW_CONTINUE)),
            "debugger" => Some((Debugger, &KW_DEBUGGER)),
            "function" => Some((Function, &KW_FUNCTION)),
            "override" => Some((Override, &KW_OVERRIDE)),
            "readonly" => Some((Readonly, &KW_READONLY)),

            "interface" => Some((Interface, &KW_INTERFACE)),
            "intrinsic" => Some((Intrinsic, &KW_INTRINSIC)),
            "namespace" => Some((Namespace, &KW_NAMESPACE)),
            "protected" => Some((Protected, &KW_PROTECTED)),
            "satisfies" => Some((Satisfies, &KW_SATISFIES)),
            "undefined" => Some((Undefined, &KW_UNDEFINED)),

            "implements" => Some((Implements, &KW_IMPLEMENTS)),
            "instanceof" => Some((Instanceof, &KW_INSTANCEOF)),

            "constructor" => Some((Constructor, &KW_CONSTRUCTOR)),
            _ => None,
        }
    }

    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn to_str(self) -> &'static str {
        match self {
            Undetermined => "Unknown",
            Eof => "EOF",
            NewLine => "\n",
            Comment => "//",
            MultiLineComment => "/** */",
            WhiteSpace => " ",
            Ident => "Identifier",
            Await => "await",
            Break => "break",
            Case => "case",
            Catch => "catch",
            Class => "class",
            Const => "const",
            Continue => "continue",
            Debugger => "debugger",
            Default => "default",
            Delete => "delete",
            Do => "do",
            Else => "else",
            Enum => "enum",
            Export => "export",
            Extends => "extends",
            Finally => "finally",
            For => "for",
            Function => "function",
            If => "if",
            Import => "import",
            In => "in",
            Instanceof => "instanceof",
            New => "new",
            Return => "return",
            Super => "super",
            Switch => "switch",
            This => "this",
            Throw => "throw",
            Try => "try",
            Typeof => "typeof",
            Var => "var",
            Void => "void",
            While => "while",
            With => "with",
            As => "as",
            Async => "async",
            From => "from",
            Get => "get",
            Meta => "meta",
            Of => "of",
            Set => "set",
            Asserts => "asserts",
            Accessor => "accessor",
            Abstract => "abstract",
            Readonly => "readonly",
            Declare => "declare",
            Override => "override",
            Type => "type",
            Target => "target",
            Implements => "implements",
            Interface => "interface",
            Package => "package",
            Private => "private",
            Protected => "protected",
            Public => "public",
            Static => "static",
            Let => "let",
            Yield => "yield",
            Amp => "&",
            Amp2 => "&&",
            Amp2Eq => "&&=",
            AmpEq => "&=",
            Bang => "!",
            Caret => "^",
            CaretEq => "^=",
            Colon => ":",
            Comma => ",",
            Dot => ".",
            Dot3 => "...",
            Eq => "=",
            Eq2 => "==",
            Eq3 => "===",
            GtEq => ">=",
            LAngle => "<",
            LBrack => "[",
            LCurly => "{",
            LParen => "(",
            LtEq => "<=",
            Minus => "-",
            Minus2 => "--",
            MinusEq => "-=",
            Neq => "!=",
            Neq2 => "!==",
            Percent => "%",
            PercentEq => "%=",
            Pipe => "|",
            Pipe2 => "||",
            Pipe2Eq => "||=",
            PipeEq => "|=",
            Plus => "+",
            Plus2 => "++",
            PlusEq => "+=",
            Question => "?",
            Question2 => "??",
            Question2Eq => "??=",
            QuestionDot => "?.",
            RAngle => ">",
            RBrack => "]",
            RCurly => "}",
            RParen => ")",
            Semicolon => ";",
            ShiftLeft => "<<",
            ShiftLeftEq => "<<=",
            ShiftRight => ">>",
            ShiftRight3 => ">>>",
            ShiftRight3Eq => ">>>=",
            ShiftRightEq => ">>=",
            Slash => "/",
            SlashEq => "/=",
            Star => "*",
            Star2 => "**",
            Star2Eq => "**=",
            StarEq => "*=",
            Tilde => "~",
            Arrow => "=>",
            Null => "null",
            True => "true",
            False => "false",
            Decimal => "decimal",
            Float => "float",
            Binary => "binary",
            Octal => "octal",
            Hex => "hex",
            Str | String => "string",
            RegExp => "/regexp/",
            NoSubstitutionTemplate => "${}",
            TemplateHead => "${",
            TemplateMiddle => "${expr}",
            TemplateTail => "$}",
            PrivateIdentifier => "#identifier",
            JSXText => "jsx",
            At => "@",
            Assert => "assert",
            Any => "any",
            Boolean => "boolean",
            Constructor => "constructor",
            Infer => "infer",
            Intrinsic => "intrinsic",
            Is => "is",
            KeyOf => "keyof",
            Module => "module",
            Namespace => "namaespace",
            Never => "never",
            Out => "out",
            Require => "require",
            Number => "number",
            Object => "object",
            Satisfies => "satisfies",
            Symbol => "symbol",
            Undefined => "undefined",
            Unique => "unique",
            Unknown => "unknown",
            Global => "global",
            BigInt => "bigint",
        }
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn can_follow_type_arguments_in_expr(self) -> bool {
        matches!(self, Self::LParen | Self::NoSubstitutionTemplate | Self::TemplateHead
            | Self::Comma | Self::Dot | Self::QuestionDot | Self::RParen | Self::RBrack
            | Self::Colon | Self::Semicolon | Self::Question | Self::Eq3 | Self::Eq2
            | Self::Neq | Self::Neq2 | Self::Amp2 | Self::Pipe2 | Self::Question2
            | Self::Caret | Self::Amp | Self::Pipe | Self::RCurly | Self::Eof)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
