use std::collections::{HashMap, HashSet};
use thiserror::Error;

pub mod lsp;

fn group_lines(text: &str) -> Vec<String> {
    let mut groups = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    for line in text.split("\n") {
        let line = line.split("//").next().unwrap_or("").trim();
        if line.is_empty() && depth == 0 {
            continue;
        }
        if !current.is_empty() {
            current.push('\n');
        }
        current.push_str(line);
        depth += line.chars().filter(|character| *character == '{').count();
        depth = depth.saturating_sub(line.chars().filter(|character| *character == '}').count());
        if depth == 0 {
            groups.push(current.clone());
            current.clear();
        }
    }
    if !current.is_empty() {
        groups.push(current);
    }
    groups
}

fn split_top_level(text: &str, delimiter: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (index, character) in text.char_indices() {
        match character {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
        if character == delimiter && depth == 0 {
            parts.push(&text[start..index]);
            start = index + character.len_utf8();
        }
    }
    parts.push(&text[start..]);
    parts
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseCommentError {
    #[error("No leading slash: {0}")]
    NoLeadingSlash(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    comment: String,
}

impl Comment {
    pub fn parse(comment: &str) -> Result<Self, ParseCommentError> {
        if !comment.starts_with("//") {
            return Err(ParseCommentError::NoLeadingSlash(comment.to_string()));
        }
        return Ok(Self {
            comment: comment.to_string(),
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseBitKeywordError {
    #[error("Invalid BIT keyword: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitKeyword {
    value: Option<bool>,
}

impl BitKeyword {
    pub fn parse(bit: &str) -> Result<Self, ParseBitKeywordError> {
        if bit != "BIT" {
            return Err(ParseBitKeywordError::Invalid(bit.to_string()));
        }
        return Ok(Self { value: None });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseNandKeywordError {
    #[error("Invalid NAND keyword: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NandKeyword {}

impl NandKeyword {
    pub fn parse(nand: &str) -> Result<Self, ParseNandKeywordError> {
        if nand != "NAND" {
            return Err(ParseNandKeywordError::Invalid(nand.to_string()));
        }
        return Ok(Self {});
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseBranchKeywordError {
    #[error("Invalid BRANCH keyword: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BranchKeyword {}

impl BranchKeyword {
    pub fn parse(branch: &str) -> Result<Self, ParseBranchKeywordError> {
        if branch != "BRANCH" {
            return Err(ParseBranchKeywordError::Invalid(branch.to_string()));
        }
        return Ok(Self {});
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseExternKeywordError {
    #[error("Invalid EXTERN keyword: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternKeyword {}

impl ExternKeyword {
    pub fn parse(r#extern: &str) -> Result<Self, ParseBranchKeywordError> {
        if r#extern != "EXTERN" {
            return Err(ParseBranchKeywordError::Invalid(r#extern.to_string()));
        }
        return Ok(Self {});
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseKeywordError {
    #[error("Invalid keyword error: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Keyword {
    Bit(BitKeyword),
    Nand(NandKeyword),
    Branch(BranchKeyword),
    Extern(ExternKeyword),
}

impl Keyword {
    pub fn parse(keyword: &str) -> Result<Self, ParseKeywordError> {
        if let Ok(bit_keyword) = BitKeyword::parse(keyword) {
            return Ok(Self::Bit(bit_keyword));
        }
        if let Ok(nand_keyword) = NandKeyword::parse(keyword) {
            return Ok(Self::Nand(nand_keyword));
        }
        if let Ok(branch_keyword) = BranchKeyword::parse(keyword) {
            return Ok(Self::Branch(branch_keyword));
        }
        if let Ok(extern_keyword) = ExternKeyword::parse(keyword) {
            return Ok(Self::Extern(extern_keyword));
        }
        Err(ParseKeywordError::Invalid(keyword.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseLabelSegementError {
    #[error("Illigal character: {0}")]
    IlligalCharacter(String),
    #[error("Label segments may not consist of only \'_\'")]
    AllUnderscores,
    #[error("Segment is empty")]
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabelSegement {
    segment: String,
}

impl LabelSegement {
    pub fn parse(segment: &str) -> Result<Self, ParseLabelSegementError> {
        if segment.is_empty() {
            return Err(ParseLabelSegementError::Missing);
        }

        for char in segment.chars() {
            if !char.is_ascii_alphanumeric() && char != '_' {
                return Err(ParseLabelSegementError::IlligalCharacter(char.to_string()));
            }
        }

        if segment.chars().all(|character| character == '_') {
            return Err(ParseLabelSegementError::AllUnderscores);
        }

        Ok(Self {
            segment: segment.to_string(),
        })
    }

    fn text(&self) -> &str {
        &self.segment
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseLabelError {
    #[error("Label segment erorr: {0}")]
    LabelSegementError(#[from] ParseLabelSegementError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    segments: Vec<LabelSegement>,
}

impl Label {
    pub fn parse(label: &str) -> Result<Self, ParseLabelError> {
        let segments = label
            .split(".")
            .map(LabelSegement::parse)
            .collect::<Result<Vec<LabelSegement>, ParseLabelSegementError>>()?;
        Ok(Self { segments: segments })
    }

    fn text(&self) -> String {
        self.segments
            .iter()
            .map(LabelSegement::text)
            .collect::<Vec<&str>>()
            .join(".")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseTypeError {
    #[error("Label erorr: {0}")]
    LabelError(#[from] ParseLabelError),
    #[error("Invalid type: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Bit(BitKeyword),
    Label(Label),
    Reference(Box<Type>),
    Collection(Vec<Type>),
}

impl Type {
    fn parse_reference(r#type: &str) -> Result<Type, ParseTypeError> {
        if !r#type.starts_with("*") {
            return Err(ParseTypeError::Invalid(r#type.to_string()));
        }
        let r#type = r#type.strip_prefix("*").unwrap_or("");
        Type::parse(r#type)
    }

    fn parse_collection(r#type: &str) -> Result<Vec<Type>, ParseTypeError> {
        if !r#type.starts_with("[") {
            return Err(ParseTypeError::Invalid(r#type.to_string()));
        }
        if !r#type.ends_with("]") {
            return Err(ParseTypeError::Invalid(r#type.to_string()));
        }
        let r#type = r#type.strip_circumfix("[", "]").unwrap_or("").trim();
        if r#type.is_empty() {
            return Ok(vec![]);
        }
        split_top_level(r#type, ',')
            .into_iter()
            .map(str::trim)
            .map(Type::parse)
            .collect::<Result<Vec<Type>, ParseTypeError>>()
    }

    pub fn parse(r#type: &str) -> Result<Self, ParseTypeError> {
        if let Ok(bit_keyword) = BitKeyword::parse(r#type) {
            return Ok(Self::Bit(bit_keyword));
        }
        if let Ok(label) = Label::parse(r#type) {
            return Ok(Self::Label(label));
        }
        if let Ok(reference) = Type::parse_reference(r#type) {
            return Ok(Self::Reference(Box::new(reference)));
        }
        if let Ok(collection) = Type::parse_collection(r#type) {
            return Ok(Self::Collection(collection));
        }
        Err(ParseTypeError::Invalid(r#type.to_string()))
    }

    fn text(&self) -> String {
        match self {
            Type::Bit(_) => "BIT".to_string(),
            Type::Label(label) => label.text(),
            Type::Reference(r#type) => format!("*{}", r#type.text()),
            Type::Collection(types) => format!(
                "[{}]",
                types
                    .iter()
                    .map(Type::text)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseTypedIdentifierError {
    #[error("Label erorr: {0}")]
    LabelError(#[from] ParseLabelError),
    #[error("Type erorr: {0}")]
    TypeError(#[from] ParseTypeError),
    #[error("Invalid Typed Identifier: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedIdentifier {
    label: Label,
    r#type: Option<Type>,
}

impl TypedIdentifier {
    pub fn parse(typed_identifier: &str) -> Result<Self, ParseTypedIdentifierError> {
        let parts = typed_identifier
            .split(":")
            .map(str::trim)
            .map(str::to_string)
            .collect::<Vec<String>>();

        let label = parts.get(0).map(String::as_str).map(Label::parse).ok_or(
            ParseTypedIdentifierError::Invalid(typed_identifier.to_string()),
        )??;
        let r#type = parts
            .get(1)
            .map(String::as_str)
            .map(Type::parse)
            .transpose()?;
        Ok(Self { label, r#type })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseParameterError {
    #[error("Label erorr: {0}")]
    LabelError(#[from] ParseLabelError),
    #[error("Type erorr: {0}")]
    TypeError(#[from] ParseTypeError),
    #[error("Invalid Parameter: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    label: Label,
    r#type: Type,
}

impl Parameter {
    pub fn parse(parameter: &str) -> Result<Self, ParseParameterError> {
        let parts = parameter
            .split(":")
            .map(str::trim)
            .map(str::to_string)
            .collect::<Vec<String>>();

        let label = parts
            .get(0)
            .map(String::as_str)
            .map(Label::parse)
            .ok_or(ParseParameterError::Invalid(parameter.to_string()))??;
        let r#type = parts
            .get(1)
            .map(String::as_str)
            .map(Type::parse)
            .ok_or(ParseParameterError::Invalid(parameter.to_string()))??;
        Ok(Self { label, r#type })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseTargetError {
    #[error("Typed identifier erorr: {0}")]
    TypedIdentifierError(#[from] ParseTypedIdentifierError),
    #[error("Invalid target: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Target {
    TypedIdentifier(TypedIdentifier),
    Reference(TypedIdentifier),
    Collection(Vec<Target>),
}

impl Target {
    fn parse_reference(target: &str) -> Result<TypedIdentifier, ParseTargetError> {
        if !target.starts_with("*") {
            return Err(ParseTargetError::Invalid(target.to_string()));
        }
        let target = target.strip_prefix("*").unwrap_or("");
        Ok(TypedIdentifier::parse(target)?)
    }

    fn parse_collection(target: &str) -> Result<Vec<Target>, ParseTargetError> {
        if !target.starts_with("[") {
            return Err(ParseTargetError::Invalid(target.to_string()));
        }
        if !target.ends_with("]") {
            return Err(ParseTargetError::Invalid(target.to_string()));
        }
        let target = target.strip_circumfix("[", "]").unwrap_or("");
        split_top_level(target, ',')
            .into_iter()
            .map(str::trim)
            .map(Target::parse)
            .collect::<Result<Vec<Target>, ParseTargetError>>()
    }

    pub fn parse(target: &str) -> Result<Self, ParseTargetError> {
        if let Ok(typed_identifier) = TypedIdentifier::parse(target) {
            return Ok(Self::TypedIdentifier(typed_identifier));
        }
        if let Ok(reference) = Target::parse_reference(target) {
            return Ok(Self::Reference(reference));
        }
        if let Ok(collection) = Target::parse_collection(target) {
            return Ok(Self::Collection(collection));
        }
        Err(ParseTargetError::Invalid(target.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParsePrimaryError {
    #[error("Label erorr: {0}")]
    LabelError(#[from] ParseLabelError),
    #[error("Invalid primary: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Primary {
    Label(Label),
    Reference(Label),
    Bit(BitKeyword),
}

impl Primary {
    fn parse_reference(primary: &str) -> Result<Label, ParsePrimaryError> {
        if !primary.starts_with("*") {
            return Err(ParsePrimaryError::Invalid(primary.to_string()));
        }
        let primary = primary.strip_prefix("*").unwrap_or("");
        if Keyword::parse(primary).is_ok() {
            return Err(ParsePrimaryError::Invalid(primary.to_string()));
        }
        Ok(Label::parse(primary)?)
    }

    pub fn parse(primary: &str) -> Result<Self, ParsePrimaryError> {
        if let Ok(bit_keyword) = BitKeyword::parse(primary) {
            return Ok(Self::Bit(bit_keyword));
        }
        if Keyword::parse(primary).is_ok() {
            return Err(ParsePrimaryError::Invalid(primary.to_string()));
        }
        if let Ok(label) = Label::parse(primary) {
            return Ok(Self::Label(label));
        }
        if let Ok(reference) = Primary::parse_reference(primary) {
            return Ok(Self::Reference(reference));
        }
        Err(ParsePrimaryError::Invalid(primary.to_string()))
    }

    fn text(&self) -> String {
        match self {
            Primary::Label(label) => label.text(),
            Primary::Reference(label) => format!("*{}", label.text()),
            Primary::Bit(_) => "BIT".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseNandExpressionError {
    #[error("Expression erorr: {0}")]
    ExpressionError(#[from] ParseExpressionError),
    #[error("Invalid nand expression: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NandExpression {
    left: Box<Expression>,
    right: Box<Expression>,
}

impl NandExpression {
    pub fn parse(nand: &str) -> Result<Self, ParseNandExpressionError> {
        if !nand.starts_with("NAND") {
            return Err(ParseNandExpressionError::Invalid(nand.to_string()));
        }
        let nand = nand.strip_prefix("NAND").unwrap_or("").trim();
        if !nand.starts_with("(") {
            return Err(ParseNandExpressionError::Invalid(nand.to_string()));
        }
        if !nand.ends_with(")") {
            return Err(ParseNandExpressionError::Invalid(nand.to_string()));
        }
        let nand = nand.strip_circumfix("(", ")").unwrap_or("");
        let parts = split_top_level(nand, ',');

        if parts.len() != 2 {
            return Err(ParseNandExpressionError::Invalid(nand.to_string()));
        }

        let left = parts
            .get(0)
            .map(|part| part.trim())
            .map(Expression::parse)
            .ok_or(ParseNandExpressionError::Invalid(nand.to_string()))??;
        let right = parts
            .get(1)
            .map(|part| part.trim())
            .map(Expression::parse)
            .ok_or(ParseNandExpressionError::Invalid(nand.to_string()))??;
        Ok(Self {
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseBranchExpressionError {
    #[error("Expression erorr: {0}")]
    ExpressionError(#[from] ParseExpressionError),
    #[error("Block erorr: {0}")]
    BlockError(#[from] ParseBlockError),
    #[error("Invalid branch expression: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BranchExpression {
    expression: Box<Expression>,
    left: Block,
    right: Block,
}

impl BranchExpression {
    pub fn parse(branch: &str) -> Result<Self, ParseBranchExpressionError> {
        if !branch.starts_with("BRANCH") {
            return Err(ParseBranchExpressionError::Invalid(branch.to_string()));
        }
        let branch = branch.strip_prefix("BRANCH").unwrap_or("").trim();
        if !branch.starts_with("(") {
            return Err(ParseBranchExpressionError::Invalid(branch.to_string()));
        }
        if !branch.ends_with("}") {
            return Err(ParseBranchExpressionError::Invalid(branch.to_string()));
        }
        let mut depth = 0usize;
        let mut end = None;
        for (index, character) in branch.char_indices() {
            match character {
                '(' => depth += 1,
                ')' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        end = Some(index);
                        break;
                    }
                }
                _ => {}
            }
        }
        let end = end.ok_or(ParseBranchExpressionError::Invalid(branch.to_string()))?;
        let expression_str = branch[1..end].trim();
        let expression = Box::new(Expression::parse(expression_str)?);

        let blocks_part = branch[end + 1..].trim();
        let mut depth = 0usize;
        let mut left_end = None;
        for (index, character) in blocks_part.char_indices() {
            match character {
                '{' => depth += 1,
                '}' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        left_end = Some(index);
                        break;
                    }
                }
                _ => {}
            }
        }
        let left_end = left_end.ok_or(ParseBranchExpressionError::Invalid(branch.to_string()))?;
        let left_str = &blocks_part[..=left_end];
        let right_str = blocks_part[left_end + 1..].trim();

        let left = Block::parse(left_str)?;
        let right = Block::parse(right_str)?;
        Ok(Self {
            expression,
            left,
            right,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseExternExpressionError {
    #[error("Expression erorr: {0}")]
    ExpressionError(#[from] ParseExpressionError),
    #[error("Invalid extern expression: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternExpression {
    expressions: Vec<Expression>,
}

impl ExternExpression {
    pub fn parse(r#extern: &str) -> Result<Self, ParseExternExpressionError> {
        if !r#extern.starts_with("EXTERN") {
            return Err(ParseExternExpressionError::Invalid(r#extern.to_string()));
        }
        let r#extern = r#extern.strip_prefix("EXTERN").unwrap_or("").trim();
        if !r#extern.starts_with("(") {
            return Err(ParseExternExpressionError::Invalid(r#extern.to_string()));
        }
        if !r#extern.ends_with(")") {
            return Err(ParseExternExpressionError::Invalid(r#extern.to_string()));
        }
        let r#extern = r#extern.strip_circumfix("(", ")").unwrap_or("").trim();
        if r#extern.is_empty() {
            return Ok(Self {
                expressions: vec![],
            });
        }
        let expressions = split_top_level(r#extern, ',')
            .into_iter()
            .map(str::trim)
            .map(Expression::parse)
            .collect::<Result<Vec<Expression>, ParseExpressionError>>()?;
        Ok(Self { expressions })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseCollectionExpressionError {
    #[error("Expression erorr: {0}")]
    ExpressionError(#[from] ParseExpressionError),
    #[error("Invalid collection expression: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CollectionExpression {
    expressions: Vec<Expression>,
}

impl CollectionExpression {
    pub fn parse(collection: &str) -> Result<Self, ParseCollectionExpressionError> {
        if !collection.starts_with("[") {
            return Err(ParseCollectionExpressionError::Invalid(
                collection.to_string(),
            ));
        }
        if !collection.ends_with("]") {
            return Err(ParseCollectionExpressionError::Invalid(
                collection.to_string(),
            ));
        }
        let collection = collection.strip_circumfix("[", "]").unwrap_or("").trim();
        if collection.is_empty() {
            return Ok(Self {
                expressions: vec![],
            });
        }
        let expressions = split_top_level(collection, ',')
            .into_iter()
            .map(str::trim)
            .map(Expression::parse)
            .collect::<Result<Vec<Expression>, ParseExpressionError>>()?;
        Ok(Self { expressions })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseCallExpressionError {
    #[error("Label erorr: {0}")]
    LabelError(#[from] ParseLabelError),
    #[error("Expression erorr: {0}")]
    ExpressionError(#[from] ParseExpressionError),
    #[error("Invalid call expression: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpression {
    label: Label,
    expressions: Vec<Expression>,
}

impl CallExpression {
    pub fn parse(call: &str) -> Result<Self, ParseCallExpressionError> {
        if !call.ends_with(")") {
            return Err(ParseCallExpressionError::Invalid(call.to_string()));
        }
        let call = call.strip_suffix(")").unwrap_or("");
        let (label_str, expressions_str) = call
            .split_once("(")
            .ok_or(ParseCallExpressionError::Invalid(call.to_string()))?;
        let label_str = label_str.trim();
        let expressions_str = expressions_str.trim();
        if Keyword::parse(label_str).is_ok() {
            return Err(ParseCallExpressionError::Invalid(call.to_string()));
        }
        let label = Label::parse(label_str)?;
        if expressions_str.is_empty() {
            return Ok(Self {
                label,
                expressions: vec![],
            });
        }
        let expressions = split_top_level(expressions_str, ',')
            .into_iter()
            .map(str::trim)
            .map(Expression::parse)
            .collect::<Result<Vec<Expression>, ParseExpressionError>>()?;
        Ok(Self { label, expressions })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseExpressionError {
    #[error("Invalid expression: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Primary(Primary),
    Extern(ExternExpression),
    Branch(BranchExpression),
    Nand(NandExpression),
    Collection(CollectionExpression),
    Call(CallExpression),
}

impl Expression {
    pub fn parse(expression: &str) -> Result<Self, ParseExpressionError> {
        if let Ok(primary) = Primary::parse(expression) {
            return Ok(Self::Primary(primary));
        }
        if let Ok(r#extern) = ExternExpression::parse(expression) {
            return Ok(Self::Extern(r#extern));
        }
        if let Ok(branch) = BranchExpression::parse(expression) {
            return Ok(Self::Branch(branch));
        }
        if let Ok(nand) = NandExpression::parse(expression) {
            return Ok(Self::Nand(nand));
        }
        if let Ok(collection) = CollectionExpression::parse(expression) {
            return Ok(Self::Collection(collection));
        }
        if let Ok(call) = CallExpression::parse(expression) {
            return Ok(Self::Call(call));
        }
        Err(ParseExpressionError::Invalid(expression.to_string()))
    }

    fn text(&self) -> String {
        match self {
            Expression::Primary(primary) => primary.text(),
            Expression::Extern(r#extern) => format!(
                "EXTERN({})",
                r#extern
                    .expressions
                    .iter()
                    .map(Expression::text)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expression::Branch(branch) => {
                format!("BRANCH ({}) {{ ... }} {{ ... }}", branch.expression.text())
            }
            Expression::Nand(nand) => {
                format!("NAND({}, {})", nand.left.text(), nand.right.text())
            }
            Expression::Collection(collection) => format!(
                "[{}]",
                collection
                    .expressions
                    .iter()
                    .map(Expression::text)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expression::Call(call) => format!(
                "{}({})",
                call.label.text(),
                call.expressions
                    .iter()
                    .map(Expression::text)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseParameterListError {
    #[error("Parameter erorr: {0}")]
    ParameterError(#[from] ParseParameterError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParameterList {
    parameters: Vec<Parameter>,
}

impl ParameterList {
    pub fn parse(parameter_list: &str) -> Result<Self, ParseParameterListError> {
        let parameters = split_top_level(parameter_list, ',')
            .into_iter()
            .map(str::trim)
            .map(Parameter::parse)
            .collect::<Result<Vec<Parameter>, ParseParameterError>>()?;
        Ok(Self { parameters })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseFunctionDefinitionError {
    #[error("Parameter list erorr: {0}")]
    ParameterListError(#[from] ParseParameterListError),
    #[error("Type erorr: {0}")]
    TypeError(#[from] ParseTypeError),
    #[error("Block erorr: {0}")]
    BlockError(#[from] ParseBlockError),
    #[error("Invalid function definition: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDefinition {
    parameters: Option<ParameterList>,
    r#type: Type,
    block: Block,
}

impl FunctionDefinition {
    pub fn parse(function_definition: &str) -> Result<Self, ParseFunctionDefinitionError> {
        if !function_definition.starts_with("(") {
            return Err(ParseFunctionDefinitionError::Invalid(
                function_definition.to_string(),
            ));
        }
        if !function_definition.ends_with("}") {
            return Err(ParseFunctionDefinitionError::Invalid(
                function_definition.to_string(),
            ));
        }
        let (parameters_part, rest) =
            function_definition
                .split_once(")")
                .ok_or(ParseFunctionDefinitionError::Invalid(
                    function_definition.to_string(),
                ))?;
        let parameters_str = parameters_part.strip_prefix("(").unwrap_or("").trim();
        let parameters = if parameters_str.is_empty() {
            None
        } else {
            Some(ParameterList::parse(parameters_str)?)
        };

        let (type_part, block_part) =
            rest.trim()
                .split_once("{")
                .ok_or(ParseFunctionDefinitionError::Invalid(
                    function_definition.to_string(),
                ))?;
        let r#type = Type::parse(type_part.trim())?;
        let block_str = format!("{{{}", block_part);
        let block = Block::parse(&block_str)?;
        Ok(Self {
            parameters,
            r#type,
            block,
        })
    }

    fn arity(&self) -> usize {
        self.parameters
            .as_ref()
            .map(|parameter_list| parameter_list.parameters.len())
            .unwrap_or(0)
    }

    fn parameter_types(&self) -> Vec<Type> {
        self.parameters
            .as_ref()
            .map(|parameter_list| {
                parameter_list
                    .parameters
                    .iter()
                    .map(|parameter| parameter.r#type.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseBindingValueError {
    #[error("Invalid binding value: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BindingValue {
    Expression(Expression),
    FunctionDefinition(FunctionDefinition),
}

impl BindingValue {
    pub fn parse(binding_value: &str) -> Result<Self, ParseBindingValueError> {
        if let Ok(expression) = Expression::parse(binding_value) {
            return Ok(Self::Expression(expression));
        }
        if let Ok(function_definition) = FunctionDefinition::parse(binding_value) {
            return Ok(Self::FunctionDefinition(function_definition));
        }
        Err(ParseBindingValueError::Invalid(binding_value.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseBindingError {
    #[error("Target erorr: {0}")]
    TargetError(#[from] ParseTargetError),
    #[error("Binding value erorr: {0}")]
    BindingValueError(#[from] ParseBindingValueError),
    #[error("Invalid binding: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Binding {
    target: Target,
    value: BindingValue,
}

impl Binding {
    pub fn parse(binding: &str) -> Result<Self, ParseBindingError> {
        let (target, value) = binding
            .split_once("=")
            .ok_or(ParseBindingError::Invalid(binding.to_string()))?;
        let target = Target::parse(target.trim())?;
        let value = BindingValue::parse(value.trim())?;
        Ok(Self { target, value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseStatementError {
    #[error("Invalid statement: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Binding(Binding),
    Expression(Expression),
}

impl Statement {
    pub fn parse(statement: &str) -> Result<Self, ParseStatementError> {
        if let Ok(binding) = Binding::parse(statement) {
            return Ok(Self::Binding(binding));
        }
        if let Ok(expression) = Expression::parse(statement) {
            return Ok(Self::Expression(expression));
        }
        Err(ParseStatementError::Invalid(statement.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseBlockError {
    #[error("Statement erorr: {0}")]
    StatementError(#[from] ParseStatementError),
    #[error("Expression erorr: {0}")]
    ExpressionError(#[from] ParseExpressionError),
    #[error("Invalid block: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    statements: Vec<Statement>,
    expression: Option<Box<Expression>>,
}

impl Block {
    pub fn parse(block: &str) -> Result<Self, ParseBlockError> {
        if !block.starts_with("{") {
            return Err(ParseBlockError::Invalid(block.to_string()));
        }
        if !block.ends_with("}") {
            return Err(ParseBlockError::Invalid(block.to_string()));
        }
        let block = block.strip_circumfix("{", "}").unwrap_or("").trim();
        if block.is_empty() {
            return Ok(Self {
                statements: vec![],
                expression: None,
            });
        }
        let groups = group_lines(block);

        if let Some((last, rest)) = groups.split_last() {
            if let Ok(expression) = Expression::parse(last) {
                let statements = rest
                    .iter()
                    .map(|group| Statement::parse(group))
                    .collect::<Result<Vec<Statement>, ParseStatementError>>()?;
                return Ok(Self {
                    statements,
                    expression: Some(Box::new(expression)),
                });
            }
        }

        let statements = groups
            .iter()
            .map(|group| Statement::parse(group))
            .collect::<Result<Vec<Statement>, ParseStatementError>>()?;
        Ok(Self {
            statements,
            expression: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParseProgramError {
    #[error("Statement erorr: {0}")]
    StatementError(#[from] ParseStatementError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ProgramError {
    #[error("Undefined label: {0}")]
    UndefinedLabel(String),
    #[error("Duplicate label: {0}")]
    DuplicateLabel(String),
    #[error("Duplicate local: {0}")]
    DuplicateLocal(String),
    #[error("Reference in collection: {0}")]
    ReferenceInCollection(String),
    #[error("Reference return type: {0}")]
    ReferenceReturnType(String),
    #[error("Arity mismatch: {0}")]
    ArityMismatch(String),
    #[error("Not callable: {0}")]
    NotCallable(String),
    #[error("Indeterminate expression: {0}")]
    Indeterminate(String),
    #[error("Invalid main: {0}")]
    InvalidMain(String),
    #[error("Invalid extern: {0}")]
    InvalidExtern(String),
}

const RUNTIME: &str = r#"const std = @import("std");

const @"$lapzt" = struct {
    pub const @"unit" = struct {};

    const runtime = @import("std");

    pub const @"IO.STDIN": u64 = 1;
    pub const @"IO.STDOUT": u64 = 2;
    pub const @"IO.STDERR": u64 = 3;
    pub const @"IO.ARGS": u64 = 4;

    const first_memory_handle: u64 = 5;
    const max_blocks: usize = 1024;
    const first_file_handle: u64 = first_memory_handle + @as(u64, max_blocks);
    const max_files: usize = 64;

    var blocks: [max_blocks]?[]u8 = @splat(null);
    var files: [max_files]?runtime.Io.File = @splat(null);
    var args: ?runtime.process.Args = null;
    var args_buffer: ?[]u8 = null;

    pub fn initialize(init: runtime.process.Init.Minimal) void {
        args = init.args;
    }

    pub fn @"OP.MEMORY.ACQUIRE"(size: u64) struct { bool, u64 } {
        for (&blocks, 0..) |*block, index| {
            if (block.* == null) {
                const length = runtime.math.cast(usize, size) orelse return .{ false, 0 };
                const memory = runtime.heap.page_allocator.alloc(u8, length) catch {
                    return .{ false, 0 };
                };
                @memset(memory, 0);
                block.* = memory;
                return .{ true, first_memory_handle + @as(u64, @intCast(index)) };
            }
        }
        return .{ false, 0 };
    }

    pub fn @"OP.MEMORY.RELEASE"(handle: u64) struct { bool } {
        const index = memory_index(handle) orelse return .{false};
        if (blocks[index]) |memory| {
            runtime.heap.page_allocator.free(memory);
            blocks[index] = null;
            return .{true};
        }
        return .{false};
    }

    pub fn @"OP.READ"(handle: u64, offset: u64) struct { bool, u64 } {
        if (handle == @"IO.STDIN") {
            return read_stream(runtime.Io.File.stdin(), offset);
        }
        if (handle == @"IO.ARGS") {
            return read_args(offset);
        }
        if (file_index(handle)) |index| {
            const file = files[index] orelse return .{ false, 0 };
            return read_stream(file, offset);
        }
        const index = memory_index(handle) orelse return .{ false, 0 };
        if (blocks[index]) |memory| {
            const start = runtime.math.cast(usize, offset) orelse return .{ false, 0 };
            if (start + 8 > memory.len) {
                return .{ false, 0 };
            }
            var word: u64 = 0;
            for (0..8) |byte| {
                word |= @as(u64, memory[start + byte]) << @intCast(byte * 8);
            }
            return .{ true, word };
        }
        return .{ false, 0 };
    }

    pub fn @"OP.WRITE"(handle: u64, offset: u64, value: u64) struct { bool } {
        if (handle == @"IO.STDOUT") {
            return write_stream(runtime.Io.File.stdout(), value);
        }
        if (handle == @"IO.STDERR") {
            return write_stream(runtime.Io.File.stderr(), value);
        }
        if (file_index(handle)) |index| {
            const file = files[index] orelse return .{false};
            return write_stream(file, value);
        }
        const index = memory_index(handle) orelse return .{false};
        if (blocks[index]) |memory| {
            const start = runtime.math.cast(usize, offset) orelse return .{false};
            if (start + 8 > memory.len) {
                return .{false};
            }
            for (0..8) |byte| {
                memory[start + byte] = @truncate(value >> @intCast(byte * 8));
            }
            return .{true};
        }
        return .{false};
    }

    pub fn @"OP.WRITE.BYTES"(handle: u64, offset: u64, value: u64, count: u64) struct { bool } {
        const length = runtime.math.cast(usize, count) orelse return .{false};
        if (length > 8) {
            return .{false};
        }
        if (handle == @"IO.STDOUT") {
            return write_stream_bytes(runtime.Io.File.stdout(), value, length);
        }
        if (handle == @"IO.STDERR") {
            return write_stream_bytes(runtime.Io.File.stderr(), value, length);
        }
        if (file_index(handle)) |index| {
            const file = files[index] orelse return .{false};
            return write_stream_bytes(file, value, length);
        }
        const index = memory_index(handle) orelse return .{false};
        if (blocks[index]) |memory| {
            const start = runtime.math.cast(usize, offset) orelse return .{false};
            if (start + length > memory.len) {
                return .{false};
            }
            for (0..length) |byte| {
                memory[start + byte] = @truncate(value >> @intCast(byte * 8));
            }
            return .{true};
        }
        return .{false};
    }

    pub fn @"OP.EXIT"(code: u64) struct {} {
        runtime.process.exit(@truncate(code));
    }

    pub fn @"OP.FILE.OPEN"(path: u64, offset: u64, length: u64, mode: u64) struct { bool, u64 } {
        const bytes = memory_slice(path, offset, length) orelse return .{ false, 0 };
        const io = runtime.Io.Threaded.global_single_threaded.io();
        const directory = runtime.Io.Dir.cwd();
        const file = switch (mode) {
            0 => directory.openFile(io, bytes, .{ .mode = .read_only }) catch return .{ false, 0 },
            1 => directory.createFile(io, bytes, .{ .truncate = true }) catch return .{ false, 0 },
            2 => blk: {
                const created = directory.createFile(io, bytes, .{ .truncate = false }) catch return .{ false, 0 };
                const end = created.length(io) catch {
                    created.close(io);
                    return .{ false, 0 };
                };
                io.vtable.fileSeekTo(io.userdata, created, end) catch {
                    created.close(io);
                    return .{ false, 0 };
                };
                break :blk created;
            },
            3 => directory.openFile(io, bytes, .{ .mode = .read_write }) catch return .{ false, 0 },
            else => return .{ false, 0 },
        };
        for (&files, 0..) |*slot, index| {
            if (slot.* == null) {
                slot.* = file;
                return .{ true, first_file_handle + @as(u64, @intCast(index)) };
            }
        }
        file.close(io);
        return .{ false, 0 };
    }

    pub fn @"OP.FILE.CLOSE"(handle: u64) struct { bool } {
        const index = file_index(handle) orelse return .{false};
        if (files[index]) |file| {
            file.close(runtime.Io.Threaded.global_single_threaded.io());
            files[index] = null;
            return .{true};
        }
        return .{false};
    }

    pub fn @"OP.FILE.SEEK"(handle: u64, offset: u64, whence: u64) struct { bool } {
        const index = file_index(handle) orelse return .{false};
        const file = files[index] orelse return .{false};
        const io = runtime.Io.Threaded.global_single_threaded.io();
        switch (whence) {
            0 => io.vtable.fileSeekTo(io.userdata, file, offset) catch return .{false},
            1 => io.vtable.fileSeekBy(io.userdata, file, @bitCast(offset)) catch return .{false},
            2 => {
                const length = file.length(io) catch return .{false};
                const base = runtime.math.cast(i64, length) orelse return .{false};
                const target = base +% @as(i64, @bitCast(offset));
                if (target < 0) {
                    return .{false};
                }
                io.vtable.fileSeekTo(io.userdata, file, @intCast(target)) catch return .{false};
            },
            else => return .{false},
        }
        return .{true};
    }

    pub fn @"OP.TIME.NOW"() u64 {
        const io = runtime.Io.Threaded.global_single_threaded.io();
        const nanoseconds = runtime.Io.Clock.real.now(io).toNanoseconds();
        return runtime.math.cast(u64, nanoseconds) orelse 0;
    }

    pub fn @"OP.TIME.SLEEP"(nanoseconds: u64) @"unit" {
        const io = runtime.Io.Threaded.global_single_threaded.io();
        const duration = runtime.Io.Duration.fromNanoseconds(@intCast(nanoseconds));
        runtime.Io.sleep(io, duration, .awake) catch {};
        return .{};
    }

    pub fn @"OP.RANDOM"() u64 {
        const io = runtime.Io.Threaded.global_single_threaded.io();
        var buffer: [8]u8 = undefined;
        runtime.Io.random(io, &buffer);
        var word: u64 = 0;
        for (0..8) |byte| {
            word |= @as(u64, buffer[byte]) << @intCast(byte * 8);
        }
        return word;
    }

    fn memory_slice(handle: u64, offset: u64, length: u64) ?[]u8 {
        const index = memory_index(handle) orelse return null;
        const memory = blocks[index] orelse return null;
        const start = runtime.math.cast(usize, offset) orelse return null;
        const count = runtime.math.cast(usize, length) orelse return null;
        if (start > memory.len or count > memory.len - start) {
            return null;
        }
        return memory[start..][0..count];
    }

    fn file_index(handle: u64) ?usize {
        if (handle < first_file_handle) {
            return null;
        }
        const index = runtime.math.cast(usize, handle - first_file_handle) orelse return null;
        if (index >= max_files) {
            return null;
        }
        return index;
    }

    fn memory_index(handle: u64) ?usize {
        if (handle < first_memory_handle) {
            return null;
        }
        const index = runtime.math.cast(usize, handle - first_memory_handle) orelse return null;
        if (index >= max_blocks) {
            return null;
        }
        return index;
    }

    fn read_stream(file: runtime.Io.File, offset: u64) struct { bool, u64 } {
        _ = offset;
        const io = runtime.Io.Threaded.global_single_threaded.io();
        var buffer: [8]u8 = @splat(0);
        const count = file.readStreaming(io, &.{&buffer}) catch return .{ false, 0 };
        if (count == 0) {
            return .{ false, 0 };
        }
        var word: u64 = 0;
        for (0..8) |byte| {
            word |= @as(u64, buffer[byte]) << @intCast(byte * 8);
        }
        return .{ true, word };
    }

    fn write_stream(file: runtime.Io.File, value: u64) struct { bool } {
        return write_stream_bytes(file, value, 8);
    }

    fn write_stream_bytes(file: runtime.Io.File, value: u64, count: usize) struct { bool } {
        const io = runtime.Io.Threaded.global_single_threaded.io();
        var buffer: [8]u8 = undefined;
        for (0..count) |byte| {
            buffer[byte] = @truncate(value >> @intCast(byte * 8));
        }
        file.writeStreamingAll(io, buffer[0..count]) catch return .{false};
        return .{true};
    }

    fn read_args(offset: u64) struct { bool, u64 } {
        if (args_buffer == null) {
            args_buffer = build_args() catch return .{ false, 0 };
        }
        const buffer = args_buffer.?;
        const start = runtime.math.cast(usize, offset) orelse return .{ false, 0 };
        if (start >= buffer.len) {
            return .{ false, 0 };
        }
        var word: u64 = 0;
        for (0..8) |byte| {
            if (start + byte < buffer.len) {
                word |= @as(u64, buffer[start + byte]) << @intCast(byte * 8);
            }
        }
        return .{ true, word };
    }

    fn build_args() ![]u8 {
        const allocator = runtime.heap.page_allocator;
        const arguments = args orelse return allocator.alloc(u8, 0);
        var iterator = try arguments.iterateAllocator(allocator);
        defer iterator.deinit();
        var buffer: runtime.ArrayList(u8) = .empty;
        while (iterator.next()) |argument| {
            try buffer.appendSlice(allocator, argument);
            try buffer.append(allocator, 0);
        }
        return buffer.toOwnedSlice(allocator);
    }
};

"#;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn parse(program: &str) -> Result<Self, ParseProgramError> {
        let statements = group_lines(program)
            .iter()
            .map(|group| Statement::parse(group))
            .collect::<Result<Vec<Statement>, ParseStatementError>>()?;
        Ok(Self { statements })
    }

    pub fn errors(&self) -> Vec<ProgramError> {
        let mut checker = Checker::new();
        checker.check_program(self);
        checker.errors
    }

    pub fn emit(&self) -> Result<String, Vec<ProgramError>> {
        self.emit_with_erasure(true)
    }

    pub fn emit_without_erasure(&self) -> Result<String, Vec<ProgramError>> {
        self.emit_with_erasure(false)
    }

    fn emit_with_erasure(&self, erasure: bool) -> Result<String, Vec<ProgramError>> {
        let errors = self.errors();
        if !errors.is_empty() {
            return Err(errors);
        }
        let mut emitter = Emitter::with_erasure(erasure);
        emitter.emit_program(self);
        Ok(format!("{}{}", RUNTIME, emitter.output))
    }
}

pub fn std() -> String {
    let mut source = [
        include_str!("../liblapzt/integers/unsigned/u1.lap"),
        include_str!("../liblapzt/bool.lap"),
        include_str!("../liblapzt/integers/unsigned/u0.lap"),
        include_str!("../liblapzt/integers/unsigned/u2.lap"),
        include_str!("../liblapzt/integers/unsigned/u4.lap"),
        include_str!("../liblapzt/integers/unsigned/u8.lap"),
        include_str!("../liblapzt/integers/unsigned/u16.lap"),
        include_str!("../liblapzt/integers/unsigned/u32.lap"),
        include_str!("../liblapzt/integers/unsigned/u64.lap"),
        include_str!("../liblapzt/integers/unsigned/u128.lap"),
        include_str!("../liblapzt/integers/signed/i2.lap"),
        include_str!("../liblapzt/integers/signed/i4.lap"),
        include_str!("../liblapzt/integers/signed/i8.lap"),
        include_str!("../liblapzt/integers/signed/i16.lap"),
        include_str!("../liblapzt/integers/signed/i32.lap"),
        include_str!("../liblapzt/integers/signed/i64.lap"),
        include_str!("../liblapzt/integers/signed/i128.lap"),
        include_str!("../liblapzt/integers/float/f16.lap"),
        include_str!("../liblapzt/integers/float/f32.lap"),
        include_str!("../liblapzt/integers/float/f64.lap"),
        include_str!("../liblapzt/integers/float/f128.lap"),
        include_str!("../liblapzt/constants.lap"),
        include_str!("../liblapzt/time.lap"),
        include_str!("../liblapzt/casts.lap"),
        include_str!("../liblapzt/compare.lap"),
        include_str!("../liblapzt/math.lap"),
        include_str!("../liblapzt/float_casts.lap"),
        include_str!("../liblapzt/io.lap"),
        include_str!("../liblapzt/stream.lap"),
        include_str!("../liblapzt/memory.lap"),
        include_str!("../liblapzt/process.lap"),
        include_str!("../liblapzt/option.lap"),
        include_str!("../liblapzt/assert.lap"),
        include_str!("../liblapzt/result.lap"),
        include_str!("../liblapzt/print.lap"),
        include_str!("../liblapzt/hash.lap"),
        include_str!("../liblapzt/random.lap"),
        include_str!("../liblapzt/checked.lap"),
        include_str!("../liblapzt/saturating.lap"),
        include_str!("../liblapzt/format.lap"),
        include_str!("../liblapzt/string.lap"),
        include_str!("../liblapzt/file.lap"),
        include_str!("../liblapzt/parse.lap"),
        include_str!("../liblapzt/alloc.lap"),
        include_str!("../liblapzt/args.lap"),
        include_str!("../liblapzt/vec.lap"),
        include_str!("../liblapzt/map.lap"),
        include_str!("../liblapzt/set.lap"),
        include_str!("../liblapzt/sort.lap"),
        include_str!("../liblapzt/search.lap"),
    ]
    .join("\n");
    source.push('\n');
    source
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Meaning {
    Type(Type),
    Constant(Value),
    Reference,
    Alias(Label),
    Runtime(Option<Type>),
    Function {
        local: bool,
        arity: usize,
        parameters: Vec<Type>,
        r#type: Type,
    },
}

fn value_type(value: &Value) -> Type {
    match value {
        Value::Bit(bit) => Type::Bit(BitKeyword { value: Some(*bit) }),
        Value::Collection(elements) => Type::Collection(elements.iter().map(value_type).collect()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Bit(bool),
    Collection(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Symbolic {
    Bit([bool; 2]),
    Collection(Vec<Symbolic>),
    Runtime,
}

impl Symbolic {
    fn from_value(value: &Value) -> Self {
        match value {
            Value::Bit(bit) => Self::Bit([*bit, *bit]),
            Value::Collection(elements) => {
                Self::Collection(elements.iter().map(Self::from_value).collect())
            }
        }
    }

    fn into_value(&self) -> Option<Value> {
        match self {
            Self::Bit([left, right]) if left == right => Some(Value::Bit(*left)),
            Self::Collection(elements) => elements
                .iter()
                .map(Self::into_value)
                .collect::<Option<Vec<Value>>>()
                .map(Value::Collection),
            _ => None,
        }
    }

    fn is_indeterminate(&self) -> bool {
        match self {
            Self::Bit([left, right]) => left != right,
            Self::Collection(elements) => elements.iter().any(Self::is_indeterminate),
            Self::Runtime => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scope {
    scopes: Vec<HashMap<String, Meaning>>,
}

impl Scope {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, label: &Label, meaning: Meaning) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(label.text(), meaning);
    }

    fn lookup(&self, label: &Label) -> Option<&Meaning> {
        let text = label.text();
        self.scopes.iter().rev().find_map(|scope| scope.get(&text))
    }

    fn resolve_alias_label(&self, label: &Label) -> Label {
        let mut current = label.clone();
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(current.text()) {
                return current;
            }
            match self.lookup(&current) {
                Some(Meaning::Alias(target)) => current = target.clone(),
                _ => return current,
            }
        }
    }

    fn resolve_alias(&self, label: &Label) -> Option<Meaning> {
        let target = self.resolve_alias_label(label);
        self.lookup(&target).cloned()
    }

    fn is_global(&self) -> bool {
        self.scopes.len() == 1
    }

    fn contains_global(&self, label: &Label) -> bool {
        self.scopes[0].contains_key(&label.text())
    }

    fn contains_local(&self, label: &Label) -> bool {
        self.scopes.last().unwrap().contains_key(&label.text())
    }

    fn visible_labels(&self) -> HashSet<String> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.keys().cloned())
            .collect()
    }

    fn evaluate(&self, expression: &Expression) -> Symbolic {
        match expression {
            Expression::Primary(primary) => self.evaluate_primary(primary),
            Expression::Nand(nand) => {
                let left = self.evaluate(&nand.left);
                let right = self.evaluate(&nand.right);
                match (left, right) {
                    (Symbolic::Bit(left), Symbolic::Bit(right)) => {
                        Symbolic::Bit([!(left[0] && right[0]), !(left[1] && right[1])])
                    }
                    _ => Symbolic::Runtime,
                }
            }
            Expression::Collection(collection) => {
                let elements = collection
                    .expressions
                    .iter()
                    .map(|expression| self.evaluate(expression))
                    .collect::<Vec<Symbolic>>();
                if elements.iter().any(|element| *element == Symbolic::Runtime) {
                    Symbolic::Runtime
                } else {
                    Symbolic::Collection(elements)
                }
            }
            _ => Symbolic::Runtime,
        }
    }

    fn evaluate_primary(&self, primary: &Primary) -> Symbolic {
        match primary {
            Primary::Bit(bit) => match bit.value {
                Some(value) => Symbolic::Bit([value, value]),
                None => Symbolic::Bit([false, true]),
            },
            Primary::Label(label) => match self.lookup(label) {
                Some(Meaning::Constant(value)) => Symbolic::from_value(value),
                _ => Symbolic::Runtime,
            },
            Primary::Reference(_) => Symbolic::Runtime,
        }
    }

    fn infer_type(&self, expression: &Expression) -> Option<Type> {
        match expression {
            Expression::Primary(Primary::Bit(bit)) => Some(Type::Bit(bit.clone())),
            Expression::Primary(Primary::Label(label)) => match self.lookup(label) {
                Some(Meaning::Constant(value)) => Some(value_type(value)),
                Some(Meaning::Runtime(r#type)) => r#type.clone(),
                _ => None,
            },
            Expression::Primary(Primary::Reference(label)) => match self.lookup(label) {
                Some(Meaning::Runtime(Some(r#type))) => {
                    Some(Type::Reference(Box::new(r#type.clone())))
                }
                _ => None,
            },
            Expression::Nand(_) => Some(Type::Bit(BitKeyword { value: None })),
            Expression::Collection(collection) => collection
                .expressions
                .iter()
                .map(|expression| self.infer_type(expression))
                .collect::<Option<Vec<Type>>>()
                .map(Type::Collection),
            Expression::Call(call) => match self.resolve_alias(&call.label) {
                Some(Meaning::Function { r#type, .. }) => Some(r#type),
                _ => None,
            },
            Expression::Branch(branch) => {
                let left = branch
                    .left
                    .expression
                    .as_ref()
                    .and_then(|expression| self.infer_type(expression));
                let right = branch
                    .right
                    .expression
                    .as_ref()
                    .and_then(|expression| self.infer_type(expression));
                match (left, right) {
                    (Some(left), Some(right)) if left == right => Some(left),
                    _ => None,
                }
            }
            Expression::Extern(_) => None,
        }
    }

    fn resolve_type(&self, expression: &Expression) -> Option<Type> {
        self.resolve_element_type(expression)
    }

    fn resolve_element_type(&self, expression: &Expression) -> Option<Type> {
        match expression {
            Expression::Primary(Primary::Bit(bit)) => Some(Type::Bit(bit.clone())),
            Expression::Primary(Primary::Label(label)) => match self.lookup(label) {
                Some(Meaning::Type(_)) => Some(Type::Label(label.clone())),
                _ => None,
            },
            Expression::Primary(Primary::Reference(label)) => match self.lookup(label) {
                Some(Meaning::Type(_)) => {
                    Some(Type::Reference(Box::new(Type::Label(label.clone()))))
                }
                _ => None,
            },
            Expression::Collection(collection) => collection
                .expressions
                .iter()
                .map(|expression| self.resolve_element_type(expression))
                .collect::<Option<Vec<Type>>>()
                .map(Type::Collection),
            _ => None,
        }
    }
}

fn is_runtime_label(label: &Label) -> bool {
    matches!(
        label.text().as_str(),
        "OP.MEMORY.ACQUIRE"
            | "OP.MEMORY.RELEASE"
            | "OP.READ"
            | "OP.WRITE"
            | "OP.WRITE.BYTES"
            | "OP.EXIT"
            | "OP.FILE.OPEN"
            | "OP.FILE.CLOSE"
            | "OP.FILE.SEEK"
            | "OP.TIME.NOW"
            | "OP.TIME.SLEEP"
            | "OP.RANDOM"
            | "IO.STDIN"
            | "IO.STDOUT"
            | "IO.STDERR"
            | "IO.ARGS"
    )
}

fn is_runtime_type(label: &Label) -> bool {
    label.text() == "U64"
}

fn word_name(width: usize) -> Option<&'static str> {
    match width {
        2 => Some("u2"),
        4 => Some("u4"),
        8 => Some("u8"),
        16 => Some("u16"),
        32 => Some("u32"),
        64 => Some("u64"),
        128 => Some("u128"),
        _ => None,
    }
}

fn erased_label(label: &Label) -> Option<&'static str> {
    match label.text().as_str() {
        "liblapzt.u1.erased" => Some("bool"),
        "liblapzt.u2.erased" => Some("u2"),
        "liblapzt.u4.erased" => Some("u4"),
        "liblapzt.u8.erased" => Some("u8"),
        "liblapzt.u16.erased" => Some("u16"),
        "liblapzt.u32.erased" => Some("u32"),
        "liblapzt.u64.erased" => Some("u64"),
        "liblapzt.u128.erased" => Some("u128"),
        "liblapzt.i2.erased" => Some("i2"),
        "liblapzt.i4.erased" => Some("i4"),
        "liblapzt.i8.erased" => Some("i8"),
        "liblapzt.i16.erased" => Some("i16"),
        "liblapzt.i32.erased" => Some("i32"),
        "liblapzt.i64.erased" => Some("i64"),
        "liblapzt.i128.erased" => Some("i128"),
        "liblapzt.f16.erased" => Some("f16"),
        "liblapzt.f32.erased" => Some("f32"),
        "liblapzt.f64.erased" => Some("f64"),
        "liblapzt.f128.erased" => Some("f128"),
        _ => None,
    }
}

fn unsigned_word(word: &str) -> &str {
    match word {
        "i2" => "u2",
        "i4" => "u4",
        "i8" => "u8",
        "i16" => "u16",
        "i32" => "u32",
        "i64" => "u64",
        "i128" => "u128",
        "f16" => "u16",
        "f32" => "u32",
        "f64" => "u64",
        "f128" => "u128",
        _ => word,
    }
}

fn is_unsigned_word(word: &str) -> bool {
    matches!(
        word,
        "bool" | "u1" | "u2" | "u4" | "u8" | "u16" | "u32" | "u64" | "u128"
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FloatOperation {
    Binary(&'static str),
    Modulo,
    Negate,
    Absolute,
    Minimum,
    Maximum,
    Select,
    Increment,
    Decrement,
    IsZero,
    IsOne,
    IsNan,
    IsInfinite,
    IsNegative,
    ToBool,
    FromBool,
    ToInteger,
    FromInteger,
    ToFloat,
    FromFloat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntegerOperation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
    Nand,
    Nor,
    Xnor,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Min,
    Max,
    Select,
    Inc,
    Dec,
    IsZero,
    IsOne,
    ToBool,
    FromBool,
    Neg,
    Abs,
    Sign,
}

fn erased_integer_operation(label: &Label) -> Option<(IntegerOperation, bool)> {
    let text = label.text();
    let rest = text.strip_prefix("liblapzt.")?;
    let rest = rest.strip_suffix(".erased")?;
    let (r#type, operation) = rest.split_once('.')?;
    let signed = match r#type {
        "u2" | "u4" | "u8" | "u16" | "u32" | "u64" | "u128" => false,
        "i2" | "i4" | "i8" | "i16" | "i32" | "i64" | "i128" => true,
        _ => return None,
    };
    let operation = match operation {
        "add" => IntegerOperation::Add,
        "sub" => IntegerOperation::Sub,
        "mul" => IntegerOperation::Mul,
        "div" => IntegerOperation::Div,
        "mod" => IntegerOperation::Mod,
        "and" => IntegerOperation::And,
        "or" => IntegerOperation::Or,
        "xor" => IntegerOperation::Xor,
        "not" => IntegerOperation::Not,
        "nand" => IntegerOperation::Nand,
        "nor" => IntegerOperation::Nor,
        "xnor" => IntegerOperation::Xnor,
        "eq" => IntegerOperation::Eq,
        "ne" => IntegerOperation::Ne,
        "lt" => IntegerOperation::Lt,
        "le" => IntegerOperation::Le,
        "gt" => IntegerOperation::Gt,
        "ge" => IntegerOperation::Ge,
        "min" => IntegerOperation::Min,
        "max" => IntegerOperation::Max,
        "select" => IntegerOperation::Select,
        "inc" => IntegerOperation::Inc,
        "dec" => IntegerOperation::Dec,
        "is.zero" => IntegerOperation::IsZero,
        "is.one" => IntegerOperation::IsOne,
        "to.bool" => IntegerOperation::ToBool,
        "from.bool" => IntegerOperation::FromBool,
        "neg" => IntegerOperation::Neg,
        "abs" => IntegerOperation::Abs,
        "sign" | "is.negative" => IntegerOperation::Sign,
        _ => return None,
    };
    Some((operation, signed))
}

fn erased_float_operation(label: &Label) -> Option<FloatOperation> {
    let text = label.text();
    let rest = text.strip_prefix("liblapzt.")?;
    let rest = rest.strip_suffix(".erased")?;
    let (float, operation) = rest.split_once('.')?;
    if !matches!(float, "f16" | "f32" | "f64" | "f128") {
        return None;
    }
    match operation {
        "add" => Some(FloatOperation::Binary("+")),
        "sub" => Some(FloatOperation::Binary("-")),
        "mul" => Some(FloatOperation::Binary("*")),
        "div" => Some(FloatOperation::Binary("/")),
        "mod" => Some(FloatOperation::Modulo),
        "eq" => Some(FloatOperation::Binary("==")),
        "ne" => Some(FloatOperation::Binary("!=")),
        "lt" => Some(FloatOperation::Binary("<")),
        "le" => Some(FloatOperation::Binary("<=")),
        "gt" => Some(FloatOperation::Binary(">")),
        "ge" => Some(FloatOperation::Binary(">=")),
        "neg" => Some(FloatOperation::Negate),
        "abs" => Some(FloatOperation::Absolute),
        "min" => Some(FloatOperation::Minimum),
        "max" => Some(FloatOperation::Maximum),
        "select" => Some(FloatOperation::Select),
        "inc" => Some(FloatOperation::Increment),
        "dec" => Some(FloatOperation::Decrement),
        "is.zero" => Some(FloatOperation::IsZero),
        "is.one" => Some(FloatOperation::IsOne),
        "is.nan" => Some(FloatOperation::IsNan),
        "is.infinite" => Some(FloatOperation::IsInfinite),
        "is.negative" => Some(FloatOperation::IsNegative),
        "to.bool" => Some(FloatOperation::ToBool),
        "from.bool" => Some(FloatOperation::FromBool),
        "to.u16" | "to.u32" | "to.u64" | "to.u128" => Some(FloatOperation::ToInteger),
        "from.u16" | "from.u32" | "from.u64" | "from.u128" => Some(FloatOperation::FromInteger),
        "to.i16" | "to.i32" | "to.i64" | "to.i128" => Some(FloatOperation::ToInteger),
        "from.i16" | "from.i32" | "from.i64" | "from.i128" => Some(FloatOperation::FromInteger),
        "to.f16" | "to.f32" | "to.f64" | "to.f128" => Some(FloatOperation::ToFloat),
        "from.f16" | "from.f32" | "from.f64" | "from.f128" => Some(FloatOperation::FromFloat),
        _ => None,
    }
}

fn value_width(value: &Value) -> Option<usize> {
    match value {
        Value::Bit(_) => Some(1),
        Value::Collection(elements) => {
            let mut total = 0;
            for element in elements {
                total += value_width(element)?;
            }
            Some(total)
        }
    }
}

fn flatten_value(value: &Value, bits: &mut Vec<bool>) {
    match value {
        Value::Bit(bit) => bits.push(*bit),
        Value::Collection(elements) => {
            for element in elements {
                flatten_value(element, bits);
            }
        }
    }
}

fn pack_value(value: &Value) -> u128 {
    let mut bits = Vec::new();
    flatten_value(value, &mut bits);
    let mut packed = 0u128;
    for (index, bit) in bits.iter().enumerate() {
        if *bit {
            packed |= 1u128 << index;
        }
    }
    packed
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Checker {
    scope: Scope,
    errors: Vec<ProgramError>,
}

impl Checker {
    fn new() -> Self {
        Self {
            scope: Scope::new(),
            errors: vec![],
        }
    }

    fn check_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.check_statement(statement);
        }
    }

    fn check_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Binding(binding) => self.check_binding(binding),
            Statement::Expression(expression) => self.check_expression(expression),
        }
    }

    fn check_binding(&mut self, binding: &Binding) {
        if let Target::TypedIdentifier(typed_identifier) = &binding.target {
            if self.scope.is_global() && typed_identifier.label.text() == "main" {
                self.check_main(binding);
            }
        }
        if let (
            Target::TypedIdentifier(typed_identifier),
            BindingValue::FunctionDefinition(function_definition),
        ) = (&binding.target, &binding.value)
        {
            if let Some(r#type) = &typed_identifier.r#type {
                self.check_type(r#type);
            }
            let meaning = Meaning::Function {
                local: !self.scope.is_global(),
                arity: function_definition.arity(),
                parameters: function_definition.parameter_types(),
                r#type: function_definition.r#type.clone(),
            };
            self.define(&typed_identifier.label, meaning);
            self.check_function_definition(function_definition);
            return;
        }
        if let (Target::TypedIdentifier(typed_identifier), BindingValue::Expression(expression)) =
            (&binding.target, &binding.value)
        {
            if typed_identifier.r#type.is_none()
                && self.scope.lookup(&typed_identifier.label).is_none()
            {
                if let Some(r#type) = self.scope.resolve_type(expression) {
                    self.check_type(&r#type);
                    self.define(&typed_identifier.label, Meaning::Type(r#type));
                    return;
                }
            }
            if typed_identifier.r#type.is_none() {
                if let Expression::Primary(Primary::Label(target)) = expression {
                    if matches!(
                        self.scope.resolve_alias(target),
                        Some(Meaning::Function { .. })
                    ) {
                        self.define(&typed_identifier.label, Meaning::Alias(target.clone()));
                        return;
                    }
                }
            }
        }
        self.check_binding_value(&binding.value);
        let meaning = self.binding_meaning(binding);
        self.check_target(&binding.target, meaning);
    }

    fn check_main(&mut self, binding: &Binding) {
        let function_definition = match &binding.value {
            BindingValue::FunctionDefinition(function_definition) => function_definition,
            BindingValue::Expression(_) => {
                self.errors.push(ProgramError::InvalidMain(
                    "main must be a function".to_string(),
                ));
                return;
            }
        };
        let parameters = function_definition
            .parameters
            .as_ref()
            .map(|parameter_list| parameter_list.parameters.len())
            .unwrap_or(0);
        if parameters != 0 {
            self.errors.push(ProgramError::InvalidMain(
                "main must take no parameters".to_string(),
            ));
        }
        if !matches!(&function_definition.r#type, Type::Collection(types) if types.is_empty()) {
            self.errors
                .push(ProgramError::InvalidMain("main must return []".to_string()));
        }
    }

    fn binding_meaning(&self, binding: &Binding) -> Meaning {
        match &binding.value {
            BindingValue::Expression(expression) => {
                if let Target::TypedIdentifier(typed_identifier) = &binding.target {
                    if let Some(Type::Reference(_)) = &typed_identifier.r#type {
                        return Meaning::Reference;
                    }
                }
                if let Expression::Primary(Primary::Reference(_)) = expression {
                    return Meaning::Reference;
                }
                if !self.scope.is_global() {
                    return Meaning::Runtime(self.scope.infer_type(expression));
                }
                match self.scope.evaluate(expression).into_value() {
                    Some(value) => Meaning::Constant(value),
                    None => Meaning::Runtime(self.scope.infer_type(expression)),
                }
            }
            BindingValue::FunctionDefinition(function_definition) => Meaning::Function {
                local: !self.scope.is_global(),
                arity: function_definition.arity(),
                parameters: function_definition.parameter_types(),
                r#type: function_definition.r#type.clone(),
            },
        }
    }

    fn check_binding_value(&mut self, value: &BindingValue) {
        match value {
            BindingValue::Expression(expression) => self.check_expression(expression),
            BindingValue::FunctionDefinition(function_definition) => {
                self.check_function_definition(function_definition)
            }
        }
    }

    fn check_function_definition(&mut self, function_definition: &FunctionDefinition) {
        self.check_type(&function_definition.r#type);
        if let Type::Reference(_) = &function_definition.r#type {
            self.errors.push(ProgramError::ReferenceReturnType(
                function_definition.r#type.text(),
            ));
        }
        self.scope.push();
        if let Some(parameter_list) = &function_definition.parameters {
            for parameter in &parameter_list.parameters {
                self.check_type(&parameter.r#type);
                let meaning = if let Type::Reference(_) = parameter.r#type {
                    Meaning::Reference
                } else {
                    Meaning::Runtime(Some(parameter.r#type.clone()))
                };
                self.define(&parameter.label, meaning);
            }
        }
        self.check_block(&function_definition.block);
        self.scope.pop();
    }

    fn check_block(&mut self, block: &Block) {
        self.scope.push();
        for statement in &block.statements {
            self.check_statement(statement);
        }
        if let Some(expression) = &block.expression {
            self.check_expression(expression);
        }
        self.scope.pop();
    }

    fn check_target(&mut self, target: &Target, meaning: Meaning) {
        match target {
            Target::TypedIdentifier(typed_identifier) => {
                if let Some(r#type) = &typed_identifier.r#type {
                    self.check_type(r#type);
                    let meaning = if let Type::Reference(_) = r#type {
                        Meaning::Reference
                    } else {
                        meaning
                    };
                    self.define(&typed_identifier.label, meaning);
                } else if !matches!(
                    self.scope.lookup(&typed_identifier.label),
                    Some(Meaning::Reference)
                ) {
                    self.define(&typed_identifier.label, meaning);
                }
            }
            Target::Reference(typed_identifier) => {
                if let Some(r#type) = &typed_identifier.r#type {
                    self.check_type(r#type);
                }
                self.use_label(&typed_identifier.label);
            }
            Target::Collection(targets) => {
                for target in targets {
                    self.check_target(target, Meaning::Runtime(None));
                }
            }
        }
    }

    fn check_expression(&mut self, expression: &Expression) {
        let symbolic = self.scope.evaluate(expression);
        if symbolic.into_value().is_some() {
            return;
        }
        if symbolic.is_indeterminate() {
            self.errors
                .push(ProgramError::Indeterminate(expression.text()));
            return;
        }
        match expression {
            Expression::Primary(primary) => self.check_primary(primary),
            Expression::Extern(r#extern) => self.check_extern(r#extern),
            Expression::Branch(branch) => {
                self.check_expression(&branch.expression);
                self.check_block(&branch.left);
                self.check_block(&branch.right);
            }
            Expression::Nand(nand) => {
                self.check_expression(&nand.left);
                self.check_expression(&nand.right);
            }
            Expression::Collection(collection) => {
                for expression in &collection.expressions {
                    if let Expression::Primary(Primary::Reference(label)) = expression {
                        self.errors
                            .push(ProgramError::ReferenceInCollection(label.text()));
                    }
                    self.check_expression(expression);
                }
            }
            Expression::Call(call) => self.check_call(call),
        }
    }

    fn check_extern(&mut self, r#extern: &ExternExpression) {
        let operation = match r#extern.expressions.first() {
            Some(Expression::Primary(Primary::Label(label))) => label.text(),
            _ => {
                self.errors.push(ProgramError::InvalidExtern(
                    "first argument must be an operation constant".to_string(),
                ));
                for expression in &r#extern.expressions {
                    self.check_expression(expression);
                }
                return;
            }
        };
        let arity = match operation.as_str() {
            "OP.MEMORY.ACQUIRE" => 1,
            "OP.MEMORY.RELEASE" => 1,
            "OP.READ" => 2,
            "OP.WRITE" => 3,
            "OP.WRITE.BYTES" => 4,
            "OP.EXIT" => 1,
            "OP.FILE.OPEN" => 4,
            "OP.FILE.CLOSE" => 1,
            "OP.FILE.SEEK" => 3,
            "OP.TIME.NOW" => 0,
            "OP.TIME.SLEEP" => 1,
            "OP.RANDOM" => 0,
            _ => {
                self.errors.push(ProgramError::InvalidExtern(format!(
                    "unknown operation: {operation}"
                )));
                for expression in r#extern.expressions.iter().skip(1) {
                    self.check_expression(expression);
                }
                return;
            }
        };
        if r#extern.expressions.len() - 1 != arity {
            self.errors.push(ProgramError::InvalidExtern(format!(
                "{operation} takes {arity} arguments"
            )));
        }
        for expression in r#extern.expressions.iter().skip(1) {
            self.check_expression(expression);
        }
    }

    fn check_primary(&mut self, primary: &Primary) {
        match primary {
            Primary::Label(label) => self.use_label(label),
            Primary::Reference(label) => self.use_label(label),
            Primary::Bit(_) => {}
        }
    }

    fn check_call(&mut self, call: &CallExpression) {
        for expression in &call.expressions {
            self.check_expression(expression);
        }
        match self.scope.resolve_alias(&call.label) {
            None => self
                .errors
                .push(ProgramError::UndefinedLabel(call.label.text())),
            Some(Meaning::Function { arity, .. }) => {
                if arity != call.expressions.len() {
                    self.errors
                        .push(ProgramError::ArityMismatch(call.label.text()));
                }
            }
            Some(_) => self
                .errors
                .push(ProgramError::NotCallable(call.label.text())),
        }
    }

    fn check_type(&mut self, r#type: &Type) {
        match r#type {
            Type::Bit(_) => {}
            Type::Label(label) => self.use_label(label),
            Type::Reference(r#type) => self.check_type(r#type),
            Type::Collection(types) => {
                for r#type in types {
                    self.check_type(r#type);
                }
            }
        }
    }

    fn define(&mut self, label: &Label, meaning: Meaning) {
        let text = label.text();
        if self.scope.is_global() {
            if self.scope.contains_global(label) {
                self.errors.push(ProgramError::DuplicateLabel(text));
                return;
            }
        } else {
            if self.scope.contains_local(label) {
                self.errors.push(ProgramError::DuplicateLocal(text));
                return;
            }
            if self.scope.contains_global(label) {
                self.errors.push(ProgramError::DuplicateLabel(text));
                return;
            }
        }
        self.scope.define(label, meaning);
    }

    fn use_label(&mut self, label: &Label) {
        if !is_runtime_label(label) && !is_runtime_type(label) && self.scope.lookup(label).is_none()
        {
            self.errors.push(ProgramError::UndefinedLabel(label.text()));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Terminal {
    Return,
    Break(String),
    Discard,
}

fn block_self_tail_calls(block: &Block, text: &str) -> bool {
    match block.expression.as_deref() {
        Some(expression) => expression_self_tail_calls(expression, text),
        None => false,
    }
}

fn expression_self_tail_calls(expression: &Expression, text: &str) -> bool {
    match expression {
        Expression::Call(call) => call.label.text() == text,
        Expression::Branch(branch) => {
            block_self_tail_calls(&branch.left, text) || block_self_tail_calls(&branch.right, text)
        }
        _ => false,
    }
}

fn block_uses_label(block: &Block, text: &str) -> bool {
    block
        .statements
        .iter()
        .any(|statement| statement_uses_label(statement, text))
        || block
            .expression
            .as_ref()
            .is_some_and(|expression| expression_uses_label(expression, text))
}

fn statement_uses_label(statement: &Statement, text: &str) -> bool {
    match statement {
        Statement::Binding(binding) => {
            target_uses_label(&binding.target, text)
                || binding_value_uses_label(&binding.value, text)
        }
        Statement::Expression(expression) => expression_uses_label(expression, text),
    }
}

fn target_uses_label(target: &Target, text: &str) -> bool {
    match target {
        Target::TypedIdentifier(typed_identifier) => {
            typed_identifier.r#type.is_none() && typed_identifier.label.text() == text
        }
        Target::Reference(typed_identifier) => typed_identifier.label.text() == text,
        Target::Collection(targets) => targets.iter().any(|target| target_uses_label(target, text)),
    }
}

fn binding_value_uses_label(value: &BindingValue, text: &str) -> bool {
    match value {
        BindingValue::Expression(expression) => expression_uses_label(expression, text),
        BindingValue::FunctionDefinition(function_definition) => {
            block_uses_label(&function_definition.block, text)
        }
    }
}

fn expression_uses_label(expression: &Expression, text: &str) -> bool {
    match expression {
        Expression::Primary(primary) => match primary {
            Primary::Label(label) => label.text() == text,
            Primary::Reference(label) => label.text() == text,
            Primary::Bit(_) => false,
        },
        Expression::Extern(r#extern) => r#extern
            .expressions
            .iter()
            .any(|expression| expression_uses_label(expression, text)),
        Expression::Branch(branch) => {
            expression_uses_label(&branch.expression, text)
                || block_uses_label(&branch.left, text)
                || block_uses_label(&branch.right, text)
        }
        Expression::Nand(nand) => {
            expression_uses_label(&nand.left, text) || expression_uses_label(&nand.right, text)
        }
        Expression::Collection(collection) => collection
            .expressions
            .iter()
            .any(|expression| expression_uses_label(expression, text)),
        Expression::Call(call) => call
            .expressions
            .iter()
            .any(|expression| expression_uses_label(expression, text)),
    }
}

fn collect_used_labels(block: &Block) -> HashSet<String> {
    let mut used = HashSet::new();
    collect_block_uses(block, &mut used);
    used
}

fn collect_block_uses(block: &Block, used: &mut HashSet<String>) {
    for statement in &block.statements {
        collect_statement_uses(statement, used);
    }
    if let Some(expression) = &block.expression {
        collect_expression_uses(expression, used);
    }
}

fn collect_statement_uses(statement: &Statement, used: &mut HashSet<String>) {
    match statement {
        Statement::Binding(binding) => {
            collect_target_uses(&binding.target, used, false);
            collect_binding_value_uses(&binding.value, used);
        }
        Statement::Expression(expression) => collect_expression_uses(expression, used),
    }
}

fn collect_target_uses(target: &Target, used: &mut HashSet<String>, definition: bool) {
    match target {
        Target::TypedIdentifier(typed_identifier) => {
            if typed_identifier.r#type.is_none() && !definition {
                used.insert(typed_identifier.label.text());
            }
            if let Some(r#type) = &typed_identifier.r#type {
                collect_type_uses(r#type, used);
            }
        }
        Target::Reference(typed_identifier) => {
            used.insert(typed_identifier.label.text());
            if let Some(r#type) = &typed_identifier.r#type {
                collect_type_uses(r#type, used);
            }
        }
        Target::Collection(targets) => {
            for target in targets {
                collect_target_uses(target, used, true);
            }
        }
    }
}

fn collect_binding_value_uses(value: &BindingValue, used: &mut HashSet<String>) {
    match value {
        BindingValue::Expression(expression) => collect_expression_uses(expression, used),
        BindingValue::FunctionDefinition(function_definition) => {
            if let Some(parameter_list) = &function_definition.parameters {
                for parameter in &parameter_list.parameters {
                    collect_type_uses(&parameter.r#type, used);
                }
            }
            collect_type_uses(&function_definition.r#type, used);
            collect_block_uses(&function_definition.block, used);
        }
    }
}

fn collect_expression_uses(expression: &Expression, used: &mut HashSet<String>) {
    match expression {
        Expression::Primary(primary) => match primary {
            Primary::Label(label) => {
                used.insert(label.text());
            }
            Primary::Reference(label) => {
                used.insert(label.text());
            }
            Primary::Bit(_) => {}
        },
        Expression::Extern(r#extern) => {
            for expression in &r#extern.expressions {
                collect_expression_uses(expression, used);
            }
        }
        Expression::Branch(branch) => {
            collect_expression_uses(&branch.expression, used);
            collect_block_uses(&branch.left, used);
            collect_block_uses(&branch.right, used);
        }
        Expression::Nand(nand) => {
            collect_expression_uses(&nand.left, used);
            collect_expression_uses(&nand.right, used);
        }
        Expression::Collection(collection) => {
            for expression in &collection.expressions {
                collect_expression_uses(expression, used);
            }
        }
        Expression::Call(call) => {
            used.insert(call.label.text());
            for expression in &call.expressions {
                collect_expression_uses(expression, used);
            }
        }
    }
}

fn collect_type_uses(r#type: &Type, used: &mut HashSet<String>) {
    match r#type {
        Type::Bit(_) => {}
        Type::Label(label) => {
            used.insert(label.text());
        }
        Type::Reference(r#type) => collect_type_uses(r#type, used),
        Type::Collection(types) => {
            for r#type in types {
                collect_type_uses(r#type, used);
            }
        }
    }
}

#[derive(Debug)]
struct Emitter {
    output: String,
    indentation: usize,
    scope: Scope,
    temporaries: usize,
    blocks: usize,
    erasure: bool,
    tail: bool,
    current_function: Option<String>,
    current_parameters: Vec<Type>,
    current_parameter_labels: Vec<Label>,
    current_return: Option<Type>,
    self_tail_recursive: bool,
}

impl Emitter {
    fn with_erasure(erasure: bool) -> Self {
        Self {
            output: String::new(),
            indentation: 0,
            scope: Scope::new(),
            temporaries: 0,
            blocks: 0,
            erasure,
            tail: false,
            current_function: None,
            current_parameters: Vec::new(),
            current_parameter_labels: Vec::new(),
            current_return: None,
            self_tail_recursive: false,
        }
    }

    fn emit_program(&mut self, program: &Program) {
        for statement in &program.statements {
            match statement {
                Statement::Binding(binding) => self.emit_global_binding(binding),
                Statement::Expression(expression) => {
                    let expression = self.emit_expression(expression);
                    self.line(&format!("comptime {{ _ = {}; }}", expression));
                }
            }
        }
    }

    fn emit_global_binding(&mut self, binding: &Binding) {
        if let Target::TypedIdentifier(typed_identifier) = &binding.target {
            if typed_identifier.r#type.is_none() {
                if let BindingValue::Expression(Expression::Primary(Primary::Label(target))) =
                    &binding.value
                {
                    if matches!(
                        self.scope.resolve_alias(target),
                        Some(Meaning::Function { .. })
                    ) {
                        self.scope
                            .define(&typed_identifier.label, Meaning::Alias(target.clone()));
                        return;
                    }
                }
            }
            if self.erasure && erased_float_operation(&typed_identifier.label).is_some() {
                if let BindingValue::FunctionDefinition(function_definition) = &binding.value {
                    self.scope.define(
                        &typed_identifier.label,
                        Meaning::Function {
                            local: false,
                            arity: function_definition.arity(),
                            parameters: function_definition.parameter_types(),
                            r#type: function_definition.r#type.clone(),
                        },
                    );
                }
                return;
            }
        }
        match (&binding.target, &binding.value) {
            (
                Target::TypedIdentifier(typed_identifier),
                BindingValue::FunctionDefinition(function_definition),
            ) => self.emit_global_function(typed_identifier, function_definition),
            (Target::TypedIdentifier(typed_identifier), BindingValue::Expression(expression)) => {
                self.emit_global_value(typed_identifier, expression)
            }
            (Target::Collection(targets), value) => {
                let value_type = match value {
                    BindingValue::Expression(expression) => self.scope.infer_type(expression),
                    BindingValue::FunctionDefinition(_) => None,
                };
                let value = self.emit_binding_value(value);
                let temporary = self.temporary();
                self.line(&format!("const {} = {};", temporary, value));
                self.emit_global_destructuring(targets, &temporary, value_type.as_ref());
            }
            (Target::Reference(typed_identifier), value) => {
                let value = self.emit_binding_value(value);
                self.line(&format!(
                    "{}.* = {};",
                    Self::emit_label(&typed_identifier.label),
                    value
                ));
            }
        }
    }

    fn emit_global_value(&mut self, typed_identifier: &TypedIdentifier, expression: &Expression) {
        if typed_identifier.r#type.is_none() {
            if self.erasure {
                if let Some(word) = erased_label(&typed_identifier.label) {
                    let label = Self::emit_label(&typed_identifier.label);
                    self.line(&format!("pub const {} = {};", label, word));
                    if let Some(r#type) = self.scope.resolve_type(expression) {
                        self.scope
                            .define(&typed_identifier.label, Meaning::Type(r#type));
                    }
                    return;
                }
            }
            if let Some(r#type) = self.scope.resolve_type(expression) {
                let text = self.emit_type(&r#type);
                let label = Self::emit_label(&typed_identifier.label);
                self.line(&format!("pub const {} = {};", label, text));
                self.scope
                    .define(&typed_identifier.label, Meaning::Type(r#type));
                return;
            }
        }
        let value = match &typed_identifier.r#type {
            Some(r#type) => self.emit_expression_typed(expression, r#type),
            None => self.emit_expression(expression),
        };
        let meaning = if let Some(Type::Reference(_)) = &typed_identifier.r#type {
            Meaning::Reference
        } else if let Expression::Primary(Primary::Reference(_)) = expression {
            Meaning::Reference
        } else {
            match self.scope.evaluate(expression).into_value() {
                Some(value) => Meaning::Constant(value),
                None => Meaning::Runtime(self.scope.infer_type(expression)),
            }
        };
        let label = Self::emit_label(&typed_identifier.label);
        match &typed_identifier.r#type {
            Some(r#type) => {
                let text = self.emit_type(r#type);
                self.line(&format!("pub const {}: {} = {};", label, text, value));
            }
            None => {
                self.line(&format!("pub const {} = {};", label, value));
            }
        }
        self.scope.define(&typed_identifier.label, meaning);
    }

    fn emit_global_function(
        &mut self,
        typed_identifier: &TypedIdentifier,
        function_definition: &FunctionDefinition,
    ) {
        let is_main = typed_identifier.label.text() == "main";
        let label = if is_main {
            "main".to_string()
        } else {
            Self::emit_label(&typed_identifier.label)
        };
        let self_tail_recursive = !is_main
            && block_self_tail_calls(&function_definition.block, &typed_identifier.label.text());
        let parameters = if is_main {
            "init: std.process.Init.Minimal".to_string()
        } else if self_tail_recursive {
            self.emit_parameters_suffixed(function_definition, ".param")
        } else {
            self.emit_parameters(function_definition)
        };
        let return_type = if is_main {
            "void".to_string()
        } else {
            self.emit_type(&function_definition.r#type)
        };
        self.line(&format!(
            "pub fn {}({}) {} {{",
            label, parameters, return_type
        ));
        self.indent();
        self.scope.define(
            &typed_identifier.label,
            Meaning::Function {
                local: false,
                arity: function_definition.arity(),
                parameters: function_definition.parameter_types(),
                r#type: function_definition.r#type.clone(),
            },
        );
        let terminal = if is_main {
            Terminal::Discard
        } else {
            Terminal::Return
        };
        if is_main {
            self.line("@\"$lapzt\".initialize(init);");
        }
        if self_tail_recursive {
            if let Some(parameter_list) = &function_definition.parameters {
                for parameter in &parameter_list.parameters {
                    self.line(&format!(
                        "var {} = {};",
                        Self::emit_label(&parameter.label),
                        Self::emit_label_with_suffix(&parameter.label, ".param")
                    ));
                }
            }
            self.line("while (true) {");
            self.indent();
        }
        let previous_function = self.current_function.clone();
        let previous_parameters = std::mem::take(&mut self.current_parameters);
        let previous_parameter_labels = std::mem::take(&mut self.current_parameter_labels);
        let previous_return = self.current_return.take();
        let previous_self_tail_recursive = self.self_tail_recursive;
        self.current_function = Some(typed_identifier.label.text());
        self.current_parameters = function_definition.parameter_types();
        self.current_parameter_labels = function_definition
            .parameters
            .as_ref()
            .map(|parameter_list| {
                parameter_list
                    .parameters
                    .iter()
                    .map(|parameter| parameter.label.clone())
                    .collect()
            })
            .unwrap_or_default();
        self.current_return = Some(function_definition.r#type.clone());
        self.self_tail_recursive = self_tail_recursive;
        self.emit_function_body(function_definition, terminal);
        self.current_function = previous_function;
        self.current_parameters = previous_parameters;
        self.current_parameter_labels = previous_parameter_labels;
        self.current_return = previous_return;
        self.self_tail_recursive = previous_self_tail_recursive;
        if self_tail_recursive {
            self.dedent();
            self.line("}");
        }
        self.dedent();
        self.line("}");
    }

    fn emit_function_body(&mut self, function_definition: &FunctionDefinition, terminal: Terminal) {
        self.tail = false;
        self.scope.push();
        if let Some(parameter_list) = &function_definition.parameters {
            for parameter in &parameter_list.parameters {
                let meaning = if let Type::Reference(_) = parameter.r#type {
                    Meaning::Reference
                } else {
                    Meaning::Runtime(Some(parameter.r#type.clone()))
                };
                self.scope.define(&parameter.label, meaning);
            }
        }
        if let Some(parameter_list) = &function_definition.parameters {
            for parameter in &parameter_list.parameters {
                if !block_uses_label(&function_definition.block, &parameter.label.text()) {
                    self.line(&format!("_ = {};", Self::emit_label(&parameter.label)));
                }
            }
        }
        for statement in self.emit_block_statements(
            &function_definition.block,
            terminal,
            Some(&function_definition.r#type),
        ) {
            self.line(&statement);
        }
        self.scope.pop();
    }

    fn emit_block_statements(
        &mut self,
        block: &Block,
        terminal: Terminal,
        expected_type: Option<&Type>,
    ) -> Vec<String> {
        let mutated = self.collect_mutated(block);
        let used = collect_used_labels(block);
        let tail = self.tail;
        self.tail = false;
        self.scope.push();
        let mut statements = block
            .statements
            .iter()
            .map(|statement| self.emit_statement_string(statement, &mutated, &used))
            .collect::<Vec<String>>();
        let value = match &block.expression {
            Some(expression) => {
                self.tail = match terminal {
                    Terminal::Return => true,
                    Terminal::Break(_) => tail,
                    Terminal::Discard => false,
                };
                let value = match expected_type {
                    Some(expected_type) => self.emit_expression_typed(expression, expected_type),
                    None => self.emit_expression(expression),
                };
                self.tail = false;
                value
            }
            None => ".{}".to_string(),
        };
        self.tail = tail;
        let is_tail_loop = self.self_tail_recursive
            && match terminal {
                Terminal::Return => true,
                Terminal::Break(_) => tail,
                Terminal::Discard => false,
            }
            && block
                .expression
                .as_deref()
                .map(|expression| {
                    expression_self_tail_calls(
                        expression,
                        self.current_function.as_deref().unwrap_or(""),
                    )
                })
                .unwrap_or(false);
        match terminal {
            Terminal::Return => {
                if is_tail_loop {
                    if matches!(block.expression.as_deref(), Some(Expression::Call(_))) {
                        statements.push(value);
                    } else {
                        statements.push(format!("{};", value));
                    }
                } else {
                    statements.push(format!("return {};", value));
                }
            }
            Terminal::Break(label) => {
                if is_tail_loop {
                    if matches!(block.expression.as_deref(), Some(Expression::Call(_))) {
                        statements.push(value);
                    } else {
                        statements.push(format!("{};", value));
                    }
                } else {
                    statements.push(format!("break :{} {};", label, value));
                }
            }
            Terminal::Discard => {
                if value != ".{}" {
                    statements.push(format!("_ = {};", value));
                }
                statements.push("return;".to_string());
            }
        }
        self.scope.pop();
        statements
    }

    fn emit_statement_string(
        &mut self,
        statement: &Statement,
        mutated: &HashSet<String>,
        used: &HashSet<String>,
    ) -> String {
        match statement {
            Statement::Binding(binding) => self.emit_local_binding(binding, mutated, used),
            Statement::Expression(expression) => {
                format!("_ = {};", self.emit_expression(expression))
            }
        }
    }

    fn emit_local_binding(
        &mut self,
        binding: &Binding,
        mutated: &HashSet<String>,
        used: &HashSet<String>,
    ) -> String {
        match (&binding.target, &binding.value) {
            (
                Target::TypedIdentifier(typed_identifier),
                BindingValue::FunctionDefinition(function_definition),
            ) => self.emit_local_function(typed_identifier, function_definition, used),
            (Target::Collection(targets), value) => {
                self.emit_local_destructuring(targets, value, mutated, used)
            }
            (Target::Reference(typed_identifier), value) => {
                let value = self.emit_binding_value(value);
                format!(
                    "{}.* = {};",
                    Self::emit_label(&typed_identifier.label),
                    value
                )
            }
            (Target::TypedIdentifier(typed_identifier), value) => {
                self.emit_local_identifier(typed_identifier, value, mutated, used)
            }
        }
    }

    fn emit_local_identifier(
        &mut self,
        typed_identifier: &TypedIdentifier,
        value: &BindingValue,
        mutated: &HashSet<String>,
        used: &HashSet<String>,
    ) -> String {
        let label = Self::emit_label(&typed_identifier.label);
        let discard = if used.contains(&typed_identifier.label.text()) {
            String::new()
        } else {
            format!(" _ = {};", label)
        };
        if typed_identifier.r#type.is_none() && self.scope.lookup(&typed_identifier.label).is_none()
        {
            if let BindingValue::Expression(expression) = value {
                if let Some(r#type) = self.scope.resolve_type(expression) {
                    let text = self.emit_type(&r#type);
                    self.scope
                        .define(&typed_identifier.label, Meaning::Type(r#type));
                    return format!("const {} = {};{}", label, text, discard);
                }
            }
        }
        let text = match value {
            BindingValue::Expression(expression) => match &typed_identifier.r#type {
                Some(r#type) => self.emit_expression_typed(expression, r#type),
                None => self.emit_expression(expression),
            },
            BindingValue::FunctionDefinition(function_definition) => {
                self.emit_function_value(Some(&typed_identifier.label), function_definition)
            }
        };
        if let Some(r#type) = &typed_identifier.r#type {
            let meaning = if let Type::Reference(_) = r#type {
                Meaning::Reference
            } else {
                Meaning::Runtime(Some(r#type.clone()))
            };
            let keyword = if meaning == Meaning::Reference {
                "const"
            } else if mutated.contains(&typed_identifier.label.text()) {
                "var"
            } else {
                "const"
            };
            let text_type = self.emit_type(r#type);
            self.scope.define(&typed_identifier.label, meaning);
            return format!(
                "{} {}: {} = {};{}",
                keyword, label, text_type, text, discard
            );
        }
        match self.scope.lookup(&typed_identifier.label).cloned() {
            Some(Meaning::Reference) => format!("{}.* = {};", label, text),
            Some(_) => format!("{} = {};", label, text),
            None => {
                let meaning =
                    if let BindingValue::Expression(Expression::Primary(Primary::Reference(_))) =
                        value
                    {
                        Meaning::Reference
                    } else if let BindingValue::Expression(expression) = value {
                        Meaning::Runtime(self.scope.infer_type(expression))
                    } else {
                        Meaning::Runtime(None)
                    };
                let keyword = if meaning == Meaning::Reference {
                    "const"
                } else if mutated.contains(&typed_identifier.label.text()) {
                    "var"
                } else {
                    "const"
                };
                self.scope.define(&typed_identifier.label, meaning);
                format!("{} {} = {};{}", keyword, label, text, discard)
            }
        }
    }

    fn emit_local_function(
        &mut self,
        typed_identifier: &TypedIdentifier,
        function_definition: &FunctionDefinition,
        used: &HashSet<String>,
    ) -> String {
        self.scope.define(
            &typed_identifier.label,
            Meaning::Function {
                local: true,
                arity: function_definition.arity(),
                parameters: function_definition.parameter_types(),
                r#type: function_definition.r#type.clone(),
            },
        );
        let value = self.emit_function_value(Some(&typed_identifier.label), function_definition);
        let discard = if used.contains(&typed_identifier.label.text()) {
            String::new()
        } else {
            format!(" _ = {};", Self::emit_label(&typed_identifier.label))
        };
        format!(
            "const {} = {};{}",
            Self::emit_label(&typed_identifier.label),
            value,
            discard
        )
    }

    fn emit_function_value(
        &mut self,
        label: Option<&Label>,
        function_definition: &FunctionDefinition,
    ) -> String {
        let previous_function = self.current_function.clone();
        let previous_parameters = std::mem::take(&mut self.current_parameters);
        let previous_return = self.current_return.take();
        self.current_function = label.map(|label| label.text());
        self.current_parameters = function_definition.parameter_types();
        self.current_return = Some(function_definition.r#type.clone());
        let parameters = self.emit_parameters(function_definition);
        let return_type = self.emit_type(&function_definition.r#type);
        self.scope.push();
        if let Some(parameter_list) = &function_definition.parameters {
            for parameter in &parameter_list.parameters {
                let meaning = if let Type::Reference(_) = parameter.r#type {
                    Meaning::Reference
                } else {
                    Meaning::Runtime(Some(parameter.r#type.clone()))
                };
                self.scope.define(&parameter.label, meaning);
            }
        }
        let mut statements = Vec::new();
        if let Some(parameter_list) = &function_definition.parameters {
            for parameter in &parameter_list.parameters {
                if !block_uses_label(&function_definition.block, &parameter.label.text()) {
                    statements.push(format!("_ = {};", Self::emit_label(&parameter.label)));
                }
            }
        }
        statements.extend(self.emit_block_statements(
            &function_definition.block,
            Terminal::Return,
            Some(&function_definition.r#type),
        ));
        self.scope.pop();
        let result = format!(
            "struct {{ fn @\"call\"({}) {} {{ {} }} }}",
            parameters,
            return_type,
            statements.join(" ")
        );
        self.current_function = previous_function;
        self.current_parameters = previous_parameters;
        self.current_return = previous_return;
        result
    }

    fn emit_local_destructuring(
        &mut self,
        targets: &[Target],
        value: &BindingValue,
        mutated: &HashSet<String>,
        used: &HashSet<String>,
    ) -> String {
        let value_type = match value {
            BindingValue::Expression(expression) => self.scope.infer_type(expression),
            BindingValue::FunctionDefinition(_) => None,
        };
        let value = self.emit_binding_value(value);
        let temporary = self.temporary();
        let mut statements = vec![format!("const {} = {};", temporary, value)];
        self.emit_local_destructuring_into(
            targets,
            &temporary,
            value_type.as_ref(),
            mutated,
            used,
            &mut statements,
        );
        statements.join(" ")
    }

    fn emit_local_destructuring_into(
        &mut self,
        targets: &[Target],
        value: &str,
        value_type: Option<&Type>,
        mutated: &HashSet<String>,
        used: &HashSet<String>,
        statements: &mut Vec<String>,
    ) {
        let word = value_type.and_then(|r#type| self.erased_word(r#type));
        let element_types = value_type.and_then(|r#type| self.element_types(r#type));
        let source = match word.as_deref() {
            Some(word) if !is_unsigned_word(word) => {
                format!("@as({}, @bitCast({}))", unsigned_word(word), value)
            }
            _ => value.to_string(),
        };
        let mut offset = 0;
        for (index, target) in targets.iter().enumerate() {
            let element_type = element_types.as_ref().and_then(|types| types.get(index));
            let element = match (word.as_ref(), element_type) {
                (Some(_), Some(element_type)) => {
                    let width = self.bit_width(element_type).unwrap_or(0);
                    let element_word = self.emit_type(element_type);
                    let expression = if width == 1 {
                        format!("@as(bool, @bitCast(@as(u1, @truncate({source} >> {offset}))))")
                    } else if is_unsigned_word(&element_word) {
                        format!("@as({element_word}, @truncate({source} >> {offset}))")
                    } else {
                        format!(
                            "@as({element_word}, @bitCast(@as({}, @truncate({source} >> {offset}))))",
                            unsigned_word(&element_word)
                        )
                    };
                    offset += width;
                    expression
                }
                _ => format!("{}[{}]", value, index),
            };
            match target {
                Target::TypedIdentifier(typed_identifier) => {
                    let label = Self::emit_label(&typed_identifier.label);
                    let meaning = match &typed_identifier.r#type {
                        Some(Type::Reference(_)) => Meaning::Reference,
                        Some(r#type) => Meaning::Runtime(Some(r#type.clone())),
                        None => Meaning::Runtime(None),
                    };
                    let keyword = if meaning == Meaning::Reference {
                        "const"
                    } else if mutated.contains(&typed_identifier.label.text()) {
                        "var"
                    } else {
                        "const"
                    };
                    let declaration = match &typed_identifier.r#type {
                        Some(r#type) => {
                            let r#type = self.emit_type(r#type);
                            format!("{} {}: {} = {};", keyword, label, r#type, element)
                        }
                        None => format!("{} {} = {};", keyword, label, element),
                    };
                    self.scope.define(&typed_identifier.label, meaning);
                    let discard = if used.contains(&typed_identifier.label.text()) {
                        String::new()
                    } else {
                        format!(" _ = {};", label)
                    };
                    statements.push(format!("{}{}", declaration, discard));
                }
                Target::Reference(typed_identifier) => {
                    statements.push(format!(
                        "{}.* = {};",
                        Self::emit_label(&typed_identifier.label),
                        element
                    ));
                }
                Target::Collection(nested) => {
                    self.emit_local_destructuring_into(
                        nested,
                        &element,
                        element_type,
                        mutated,
                        used,
                        statements,
                    );
                }
            }
        }
    }

    fn emit_global_destructuring(
        &mut self,
        targets: &[Target],
        value: &str,
        value_type: Option<&Type>,
    ) {
        let word = value_type.and_then(|r#type| self.erased_word(r#type));
        let element_types = value_type.and_then(|r#type| self.element_types(r#type));
        let source = match word.as_deref() {
            Some(word) if !is_unsigned_word(word) => {
                format!("@as({}, @bitCast({}))", unsigned_word(word), value)
            }
            _ => value.to_string(),
        };
        let mut offset = 0;
        for (index, target) in targets.iter().enumerate() {
            let element_type = element_types.as_ref().and_then(|types| types.get(index));
            let element = match (word.as_ref(), element_type) {
                (Some(_), Some(element_type)) => {
                    let width = self.bit_width(element_type).unwrap_or(0);
                    let element_word = self.emit_type(element_type);
                    let expression = if width == 1 {
                        format!("@as(bool, @bitCast(@as(u1, @truncate({source} >> {offset}))))")
                    } else if is_unsigned_word(&element_word) {
                        format!("@as({element_word}, @truncate({source} >> {offset}))")
                    } else {
                        format!(
                            "@as({element_word}, @bitCast(@as({}, @truncate({source} >> {offset}))))",
                            unsigned_word(&element_word)
                        )
                    };
                    offset += width;
                    expression
                }
                _ => format!("{}[{}]", value, index),
            };
            match target {
                Target::TypedIdentifier(typed_identifier) => {
                    let label = Self::emit_label(&typed_identifier.label);
                    let meaning = match &typed_identifier.r#type {
                        Some(Type::Reference(_)) => Meaning::Reference,
                        Some(r#type) => Meaning::Runtime(Some(r#type.clone())),
                        None => Meaning::Runtime(None),
                    };
                    let declaration = match &typed_identifier.r#type {
                        Some(r#type) => {
                            let r#type = self.emit_type(r#type);
                            format!("pub const {}: {} = {};", label, r#type, element)
                        }
                        None => format!("pub const {} = {};", label, element),
                    };
                    self.line(&declaration);
                    self.scope.define(&typed_identifier.label, meaning);
                }
                Target::Reference(typed_identifier) => {
                    self.line(&format!(
                        "{}.* = {};",
                        Self::emit_label(&typed_identifier.label),
                        element
                    ));
                }
                Target::Collection(nested) => {
                    self.emit_global_destructuring(nested, &element, element_type);
                }
            }
        }
    }

    fn emit_binding_value(&mut self, value: &BindingValue) -> String {
        match value {
            BindingValue::Expression(expression) => self.emit_expression(expression),
            BindingValue::FunctionDefinition(function_definition) => {
                self.emit_function_value(None, function_definition)
            }
        }
    }

    fn emit_expression(&mut self, expression: &Expression) -> String {
        let symbolic = self.scope.evaluate(expression);
        if let Some(value) = symbolic.into_value() {
            return self.emit_value(&value);
        }
        if symbolic.is_indeterminate() {
            return "undefined".to_string();
        }
        match expression {
            Expression::Primary(primary) => self.emit_primary(primary),
            Expression::Extern(r#extern) => {
                let tail = self.tail;
                self.tail = false;
                let operation = r#extern.expressions.first();
                let arguments = r#extern
                    .expressions
                    .iter()
                    .skip(1)
                    .map(|expression| {
                        let r#type = self.scope.infer_type(expression);
                        let value = self.emit_expression(expression);
                        if self.erasure {
                            value
                        } else {
                            match r#type {
                                Some(r#type) => self.emit_pack_runtime(&value, &r#type),
                                None => value,
                            }
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                let result = match operation {
                    Some(Expression::Primary(Primary::Label(label))) => {
                        let call =
                            format!("@\"$lapzt\".{}({})", Self::emit_label(label), arguments);
                        if label.text() == "OP.EXIT" {
                            format!("blk: {{ _ = {call}; break :blk .{{}}; }}")
                        } else if self.erasure {
                            call
                        } else {
                            self.emit_extern_result(label, &call)
                        }
                    }
                    _ => {
                        let arguments = r#extern
                            .expressions
                            .iter()
                            .map(|expression| self.emit_expression(expression))
                            .collect::<Vec<String>>()
                            .join(", ");
                        format!("@\"$lapzt\".@\"EXTERN\"({})", arguments)
                    }
                };
                self.tail = tail;
                result
            }
            Expression::Branch(branch) => self.emit_branch(branch),
            Expression::Nand(nand) => {
                let tail = self.tail;
                self.tail = false;
                let left = self.emit_expression(&nand.left);
                let right = self.emit_expression(&nand.right);
                self.tail = tail;
                format!("!({} and {})", left, right)
            }
            Expression::Collection(collection) => {
                let tail = self.tail;
                self.tail = false;
                let elements = collection
                    .expressions
                    .iter()
                    .map(|expression| self.emit_expression(expression))
                    .collect::<Vec<String>>();
                self.tail = tail;
                if elements.is_empty() {
                    ".{}".to_string()
                } else {
                    format!(".{{ {} }}", elements.join(", "))
                }
            }
            Expression::Call(call) => {
                let tail = self.tail;
                self.tail = false;
                let target = self.scope.resolve_alias_label(&call.label);
                let meaning = self.scope.lookup(&target).cloned();
                let parameters = match &meaning {
                    Some(Meaning::Function { parameters, .. }) => parameters.clone(),
                    _ => Vec::new(),
                };
                if self.erasure {
                    if let Some(operation) = erased_float_operation(&target) {
                        return self.emit_float_operation(
                            operation,
                            &call.expressions,
                            &parameters,
                        );
                    }
                    if let Some((operation, signed)) = erased_integer_operation(&target) {
                        return self.emit_integer_operation(
                            operation,
                            signed,
                            &call.expressions,
                            &parameters,
                        );
                    }
                }
                let argument_values = call
                    .expressions
                    .iter()
                    .enumerate()
                    .map(|(index, expression)| match parameters.get(index) {
                        Some(parameter) => self.emit_expression_typed(expression, parameter),
                        None => self.emit_expression(expression),
                    })
                    .collect::<Vec<String>>();
                let arguments = argument_values.join(", ");
                let label = Self::emit_label(&target);
                let same_function =
                    self.current_function.as_deref() == Some(target.text().as_str());
                let call_expression = match meaning {
                    Some(Meaning::Function { local: true, .. }) => {
                        format!("{}.@\"call\"({})", label, arguments)
                    }
                    _ => format!("{}({})", label, arguments),
                };
                if tail && self.self_tail_recursive && same_function {
                    let temporaries = argument_values
                        .iter()
                        .map(|_| self.temporary())
                        .collect::<Vec<String>>();
                    let declarations = temporaries
                        .iter()
                        .zip(argument_values.iter())
                        .map(|(temporary, value)| format!("const {} = {};", temporary, value))
                        .collect::<Vec<String>>()
                        .join(" ");
                    let assignments = self
                        .current_parameter_labels
                        .iter()
                        .zip(temporaries.iter())
                        .map(|(label, temporary)| {
                            format!("{} = {};", Self::emit_label(label), temporary)
                        })
                        .collect::<Vec<String>>()
                        .join(" ");
                    format!("{{ {} {} continue; }}", declarations, assignments)
                } else {
                    call_expression
                }
            }
        }
    }

    fn emit_integer_operation(
        &mut self,
        operation: IntegerOperation,
        signed: bool,
        expressions: &[Expression],
        parameters: &[Type],
    ) -> String {
        let arguments = expressions
            .iter()
            .enumerate()
            .map(|(index, expression)| match parameters.get(index) {
                Some(parameter) => self.emit_expression_typed(expression, parameter),
                None => self.emit_expression(expression),
            })
            .collect::<Vec<String>>();
        match operation {
            IntegerOperation::Add => format!("({} + {})", arguments[0], arguments[1]),
            IntegerOperation::Sub => format!("({} - {})", arguments[0], arguments[1]),
            IntegerOperation::Mul => format!("({} * {})", arguments[0], arguments[1]),
            IntegerOperation::Div => format!("({} / {})", arguments[0], arguments[1]),
            IntegerOperation::Mod => format!("@rem({}, {})", arguments[0], arguments[1]),
            IntegerOperation::And => format!("({} & {})", arguments[0], arguments[1]),
            IntegerOperation::Or => format!("({} | {})", arguments[0], arguments[1]),
            IntegerOperation::Xor => format!("({} ^ {})", arguments[0], arguments[1]),
            IntegerOperation::Not => format!("(~{})", arguments[0]),
            IntegerOperation::Nand => format!("(~({} & {}))", arguments[0], arguments[1]),
            IntegerOperation::Nor => format!("(~({} | {}))", arguments[0], arguments[1]),
            IntegerOperation::Xnor => format!("(~({} ^ {}))", arguments[0], arguments[1]),
            IntegerOperation::Eq => format!("({} == {})", arguments[0], arguments[1]),
            IntegerOperation::Ne => format!("({} != {})", arguments[0], arguments[1]),
            IntegerOperation::Lt => format!("({} < {})", arguments[0], arguments[1]),
            IntegerOperation::Le => format!("({} <= {})", arguments[0], arguments[1]),
            IntegerOperation::Gt => format!("({} > {})", arguments[0], arguments[1]),
            IntegerOperation::Ge => format!("({} >= {})", arguments[0], arguments[1]),
            IntegerOperation::Min => format!("@min({}, {})", arguments[0], arguments[1]),
            IntegerOperation::Max => format!("@max({}, {})", arguments[0], arguments[1]),
            IntegerOperation::Select => format!(
                "(if ({}) {} else {})",
                arguments[0], arguments[1], arguments[2]
            ),
            IntegerOperation::Inc => format!("({} + 1)", arguments[0]),
            IntegerOperation::Dec => format!("({} - 1)", arguments[0]),
            IntegerOperation::IsZero => format!("({} == 0)", arguments[0]),
            IntegerOperation::IsOne => format!("({} == 1)", arguments[0]),
            IntegerOperation::ToBool => format!("({} != 0)", arguments[0]),
            IntegerOperation::FromBool => format!("@intFromBool({})", arguments[0]),
            IntegerOperation::Neg => format!("(-{})", arguments[0]),
            IntegerOperation::Abs => format!("@abs({})", arguments[0]),
            IntegerOperation::Sign => {
                if signed {
                    format!("({} < 0)", arguments[0])
                } else {
                    "false".to_string()
                }
            }
        }
    }

    fn emit_float_operation(
        &mut self,
        operation: FloatOperation,
        expressions: &[Expression],
        parameters: &[Type],
    ) -> String {
        let arguments = expressions
            .iter()
            .enumerate()
            .map(|(index, expression)| match parameters.get(index) {
                Some(parameter) => self.emit_expression_typed(expression, parameter),
                None => self.emit_expression(expression),
            })
            .collect::<Vec<String>>();
        match operation {
            FloatOperation::Binary(operator) => {
                format!("({} {} {})", arguments[0], operator, arguments[1])
            }
            FloatOperation::Modulo => format!("@mod({}, {})", arguments[0], arguments[1]),
            FloatOperation::Negate => format!("(-{})", arguments[0]),
            FloatOperation::Absolute => format!("@abs({})", arguments[0]),
            FloatOperation::Minimum => format!("@min({}, {})", arguments[0], arguments[1]),
            FloatOperation::Maximum => format!("@max({}, {})", arguments[0], arguments[1]),
            FloatOperation::Select => format!(
                "(if ({}) {} else {})",
                arguments[0], arguments[1], arguments[2]
            ),
            FloatOperation::Increment => format!("({} + 1)", arguments[0]),
            FloatOperation::Decrement => format!("({} - 1)", arguments[0]),
            FloatOperation::IsZero => format!("({} == 0)", arguments[0]),
            FloatOperation::IsOne => format!("({} == 1)", arguments[0]),
            FloatOperation::IsNan => format!("std.math.isNan({})", arguments[0]),
            FloatOperation::IsInfinite => format!("std.math.isInf({})", arguments[0]),
            FloatOperation::IsNegative => format!("std.math.signbit({})", arguments[0]),
            FloatOperation::ToBool => format!("({} != 0)", arguments[0]),
            FloatOperation::FromBool => {
                format!("@floatFromInt(@intFromBool({}))", arguments[0])
            }
            FloatOperation::ToInteger => format!("@intFromFloat({})", arguments[0]),
            FloatOperation::FromInteger => format!("@floatFromInt({})", arguments[0]),
            FloatOperation::ToFloat | FloatOperation::FromFloat => {
                format!("@floatCast({})", arguments[0])
            }
        }
    }

    fn emit_expression_typed(&mut self, expression: &Expression, expected_type: &Type) -> String {
        if let Some(value) = self.scope.evaluate(expression).into_value() {
            return self.emit_value_typed(&value, expected_type);
        }
        if !self.erasure {
            if let Expression::Primary(Primary::Label(label)) = expression {
                if is_runtime_label(label) {
                    let value = format!("@\"$lapzt\".{}", Self::emit_label(label));
                    return self.emit_unpack_runtime(&value, expected_type);
                }
            }
        }
        let resolved = self.resolved_type(expected_type);
        let erased = self.erased_word(expected_type);
        let element_types = self
            .element_types(expected_type)
            .or_else(|| match &resolved {
                Type::Collection(types) => Some(types.clone()),
                _ => None,
            });
        match expression {
            Expression::Collection(collection) => {
                let tail = self.tail;
                self.tail = false;
                let result =
                    if let (Some(word), Some(types)) = (erased.as_ref(), element_types.as_ref()) {
                        self.emit_packed(collection, types, word)
                    } else {
                        let elements = collection
                            .expressions
                            .iter()
                            .enumerate()
                            .map(|(index, expression)| {
                                let element_type =
                                    element_types.as_ref().and_then(|types| types.get(index));
                                match element_type {
                                    Some(element_type) => {
                                        self.emit_expression_typed(expression, element_type)
                                    }
                                    None => self.emit_expression(expression),
                                }
                            })
                            .collect::<Vec<String>>();
                        if elements.is_empty() {
                            ".{}".to_string()
                        } else {
                            format!(".{{ {} }}", elements.join(", "))
                        }
                    };
                self.tail = tail;
                result
            }
            Expression::Branch(branch) => {
                let tail = self.tail;
                self.tail = false;
                let condition = self.emit_expression(&branch.expression);
                self.tail = tail;
                let left_label = format!("blk{}", self.blocks);
                self.blocks += 1;
                let right_label = format!("blk{}", self.blocks);
                self.blocks += 1;
                let left = if tail && self.self_tail_recursive {
                    self.emit_tail_block_expression(&branch.left, Some(expected_type))
                } else {
                    self.emit_block_expression(&branch.left, &left_label, Some(expected_type))
                };
                let right = if tail && self.self_tail_recursive {
                    self.emit_tail_block_expression(&branch.right, Some(expected_type))
                } else {
                    self.emit_block_expression(&branch.right, &right_label, Some(expected_type))
                };
                self.tail = false;
                format!("(if ({}) {} else {})", condition, left, right)
            }
            _ => self.emit_expression(expression),
        }
    }

    fn emit_packed(
        &mut self,
        collection: &CollectionExpression,
        types: &[Type],
        word: &str,
    ) -> String {
        let unsigned = unsigned_word(word);
        let mut parts = Vec::new();
        let mut offset = 0;
        for (expression, element_type) in collection.expressions.iter().zip(types) {
            let width = self.bit_width(element_type).unwrap_or(0);
            let value = self.emit_expression_typed(expression, element_type);
            let value = match self.erased_word(element_type) {
                Some(element_word) if !is_unsigned_word(&element_word) => {
                    format!("@as({}, @bitCast({}))", unsigned_word(&element_word), value)
                }
                _ => value,
            };
            let part = if width == 1 {
                format!("(@as({unsigned}, @intFromBool({value})) << {offset})")
            } else {
                format!("(@as({unsigned}, {value}) << {offset})")
            };
            parts.push(part);
            offset += width;
        }
        let packed = if parts.is_empty() {
            format!("@as({unsigned}, 0)")
        } else {
            parts.join(" | ")
        };
        if unsigned == word {
            packed
        } else {
            format!("@as({word}, @bitCast({packed}))")
        }
    }

    fn emit_pack_runtime(&self, value: &str, r#type: &Type) -> String {
        let element_types = match self.element_types(r#type) {
            Some(types) => types,
            None => return value.to_string(),
        };
        let mut parts = Vec::new();
        let mut offset = 0;
        for (index, element_type) in element_types.iter().enumerate() {
            let width = self.bit_width(element_type).unwrap_or(0);
            let element = format!("{}[{}]", value, index);
            let part = if width == 1 {
                format!("(@as(u64, @intFromBool({element})) << {offset})")
            } else {
                let packed = self.emit_pack_runtime(&element, element_type);
                format!("(({packed}) << {offset})")
            };
            parts.push(part);
            offset += width;
        }
        if parts.is_empty() {
            "@as(u64, 0)".to_string()
        } else {
            parts.join(" | ")
        }
    }

    fn emit_unpack_runtime(&self, value: &str, r#type: &Type) -> String {
        let element_types = match self.element_types(r#type) {
            Some(types) => types,
            None => return value.to_string(),
        };
        let mut elements = Vec::new();
        let mut offset = 0;
        for element_type in element_types {
            let width = self.bit_width(&element_type).unwrap_or(0);
            let element = if width == 1 {
                format!("@as(bool, @bitCast(@as(u1, @truncate({value} >> {offset}))))")
            } else {
                let inner = format!("@as(u64, @truncate({value} >> {offset}))");
                self.emit_unpack_runtime(&inner, &element_type)
            };
            elements.push(element);
            offset += width;
        }
        if elements.is_empty() {
            ".{}".to_string()
        } else {
            format!(".{{ {} }}", elements.join(", "))
        }
    }

    fn emit_extern_result(&mut self, label: &Label, call: &str) -> String {
        match label.text().as_str() {
            "OP.MEMORY.ACQUIRE" | "OP.READ" | "OP.FILE.OPEN" => {
                let temporary = self.temporary();
                let word = Type::Label(Label::parse("liblapzt.u64").unwrap());
                let unpacked = self.emit_unpack_runtime(&format!("{temporary}[1]"), &word);
                format!(
                    "blk: {{ const {temporary} = {call}; break :blk .{{ {temporary}[0], {unpacked} }}; }}"
                )
            }
            "OP.MEMORY.RELEASE" | "OP.WRITE" | "OP.WRITE.BYTES" | "OP.FILE.CLOSE"
            | "OP.FILE.SEEK" => {
                format!(".{{ {call}[0] }}")
            }
            "OP.TIME.NOW" | "OP.RANDOM" => {
                let temporary = self.temporary();
                let word = Type::Label(Label::parse("liblapzt.u64").unwrap());
                let unpacked = self.emit_unpack_runtime(&temporary, &word);
                format!("blk: {{ const {temporary} = {call}; break :blk {unpacked}; }}")
            }
            _ => call.to_string(),
        }
    }

    fn emit_branch(&mut self, branch: &BranchExpression) -> String {
        let tail = self.tail;
        self.tail = false;
        let condition = self.emit_expression(&branch.expression);
        self.tail = tail;
        let left_label = format!("blk{}", self.blocks);
        self.blocks += 1;
        let right_label = format!("blk{}", self.blocks);
        self.blocks += 1;
        let left = if tail && self.self_tail_recursive {
            self.emit_tail_block_expression(&branch.left, None)
        } else {
            self.emit_block_expression(&branch.left, &left_label, None)
        };
        let right = if tail && self.self_tail_recursive {
            self.emit_tail_block_expression(&branch.right, None)
        } else {
            self.emit_block_expression(&branch.right, &right_label, None)
        };
        self.tail = false;
        format!("(if ({}) {} else {})", condition, left, right)
    }

    fn emit_tail_block_expression(
        &mut self,
        block: &Block,
        expected_type: Option<&Type>,
    ) -> String {
        let statements = self.emit_block_statements(block, Terminal::Return, expected_type);
        format!("{{ {} }}", statements.join(" "))
    }

    fn emit_block_expression(
        &mut self,
        block: &Block,
        label: &str,
        expected_type: Option<&Type>,
    ) -> String {
        let statements =
            self.emit_block_statements(block, Terminal::Break(label.to_string()), expected_type);
        format!("{}: {{ {} }}", label, statements.join(" "))
    }

    fn emit_primary(&mut self, primary: &Primary) -> String {
        match primary {
            Primary::Bit(bit) => match bit.value {
                Some(value) => value.to_string(),
                None => "undefined".to_string(),
            },
            Primary::Label(label) => {
                let target = self.scope.resolve_alias_label(label);
                match self.scope.lookup(&target) {
                    Some(Meaning::Reference) => format!("{}.*", Self::emit_label(&target)),
                    Some(_) => Self::emit_label(&target),
                    None if is_runtime_label(&target) => {
                        format!("@\"$lapzt\".{}", Self::emit_label(&target))
                    }
                    None => Self::emit_label(&target),
                }
            }
            Primary::Reference(label) => self.emit_reference(label),
        }
    }

    fn emit_reference(&self, label: &Label) -> String {
        match self.scope.lookup(label) {
            Some(Meaning::Reference) => Self::emit_label(label),
            _ => format!("&{}", Self::emit_label(label)),
        }
    }

    fn emit_label(label: &Label) -> String {
        format!("@\"{}\"", label.text())
    }

    fn bit_width(&self, r#type: &Type) -> Option<usize> {
        match r#type {
            Type::Bit(_) => Some(1),
            Type::Label(label) => match self.scope.lookup(label) {
                Some(Meaning::Type(r#type)) => self.bit_width(r#type),
                _ => None,
            },
            Type::Collection(types) => {
                let mut total = 0;
                for r#type in types {
                    total += self.bit_width(r#type)?;
                }
                Some(total)
            }
            Type::Reference(_) => None,
        }
    }

    fn element_types(&self, r#type: &Type) -> Option<Vec<Type>> {
        match r#type {
            Type::Collection(types) => Some(types.clone()),
            Type::Label(label) => match self.scope.lookup(label) {
                Some(Meaning::Type(r#type)) => self.element_types(r#type),
                _ => None,
            },
            _ => None,
        }
    }

    fn erased_word(&self, r#type: &Type) -> Option<String> {
        if !self.erasure {
            return None;
        }
        match r#type {
            Type::Label(label) => {
                if let Some(word) = erased_label(label) {
                    return Some(word.to_string());
                }
                match self.scope.lookup(label) {
                    Some(Meaning::Type(r#type)) => self.erased_word(r#type),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn resolved_type(&self, r#type: &Type) -> Type {
        match r#type {
            Type::Label(label) => match self.scope.lookup(label) {
                Some(Meaning::Type(r#type)) => self.resolved_type(r#type),
                _ => r#type.clone(),
            },
            Type::Collection(types) => Type::Collection(
                types
                    .iter()
                    .map(|r#type| self.resolved_type(r#type))
                    .collect(),
            ),
            Type::Reference(r#type) => Type::Reference(Box::new(self.resolved_type(r#type))),
            Type::Bit(_) => r#type.clone(),
        }
    }

    fn emit_type(&self, r#type: &Type) -> String {
        match r#type {
            Type::Bit(_) => "bool".to_string(),
            Type::Label(label) => {
                if self.erasure {
                    if let Some(word) = erased_label(label) {
                        return word.to_string();
                    }
                }
                match self.scope.lookup(label) {
                    Some(Meaning::Type(r#type)) => self.emit_type(r#type),
                    _ if is_runtime_type(label) => "u64".to_string(),
                    _ => Self::emit_label(label),
                }
            }
            Type::Reference(r#type) => format!("*{}", self.emit_type(r#type)),
            Type::Collection(types) => {
                if types.is_empty() {
                    "@\"$lapzt\".@\"unit\"".to_string()
                } else {
                    format!(
                        "struct {{ {} }}",
                        types
                            .iter()
                            .map(|r#type| self.emit_type(r#type))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
        }
    }

    fn emit_parameters(&self, function_definition: &FunctionDefinition) -> String {
        self.emit_parameters_suffixed(function_definition, "")
    }

    fn emit_parameters_suffixed(
        &self,
        function_definition: &FunctionDefinition,
        suffix: &str,
    ) -> String {
        function_definition
            .parameters
            .as_ref()
            .map(|parameter_list| {
                parameter_list
                    .parameters
                    .iter()
                    .map(|parameter| {
                        format!(
                            "{}: {}",
                            Self::emit_label_with_suffix(&parameter.label, suffix),
                            self.emit_type(&parameter.r#type)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .unwrap_or_default()
    }

    fn emit_label_with_suffix(label: &Label, suffix: &str) -> String {
        format!("@\"{}{}\"", label.text(), suffix)
    }

    fn emit_value_typed(&self, value: &Value, expected_type: &Type) -> String {
        if !self.erasure {
            return self.emit_value(value);
        }
        if let Some(word) = self.erased_word(expected_type) {
            if let Some(width) = value_width(value) {
                let unsigned = unsigned_word(&word);
                if word_name(width) == Some(unsigned) {
                    let packed = format!("@as({}, {})", unsigned, pack_value(value));
                    if unsigned == word {
                        return packed;
                    }
                    return format!("@as({}, @bitCast({}))", word, packed);
                }
            }
        }
        match (value, self.element_types(expected_type)) {
            (Value::Collection(elements), Some(types)) => {
                let elements = elements
                    .iter()
                    .enumerate()
                    .map(|(index, element)| match types.get(index) {
                        Some(element_type) => self.emit_value_typed(element, element_type),
                        None => self.emit_value(element),
                    })
                    .collect::<Vec<String>>();
                if elements.is_empty() {
                    ".{}".to_string()
                } else {
                    format!(".{{ {} }}", elements.join(", "))
                }
            }
            _ => self.emit_value(value),
        }
    }

    fn emit_value(&self, value: &Value) -> String {
        match value {
            Value::Bit(bit) => bit.to_string(),
            Value::Collection(elements) => {
                if elements.is_empty() {
                    ".{}".to_string()
                } else {
                    format!(
                        ".{{ {} }}",
                        elements
                            .iter()
                            .map(|element| self.emit_value(element))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
        }
    }

    fn temporary(&mut self) -> String {
        let temporary = format!("@\"${}\"", self.temporaries);
        self.temporaries += 1;
        temporary
    }

    fn line(&mut self, text: &str) {
        for _ in 0..self.indentation {
            self.output.push_str("    ");
        }
        self.output.push_str(text);
        self.output.push('\n');
    }

    fn indent(&mut self) {
        self.indentation += 1;
    }

    fn dedent(&mut self) {
        self.indentation -= 1;
    }

    fn collect_mutated(&self, block: &Block) -> HashSet<String> {
        let mut defined = self.scope.visible_labels();
        let mut mutated = HashSet::new();
        for statement in &block.statements {
            self.collect_statement_mutations(statement, &mut defined, &mut mutated);
        }
        if let Some(expression) = &block.expression {
            self.collect_expression_mutations(expression, &mut defined, &mut mutated);
        }
        mutated
    }

    fn collect_statement_mutations(
        &self,
        statement: &Statement,
        defined: &mut HashSet<String>,
        mutated: &mut HashSet<String>,
    ) {
        match statement {
            Statement::Binding(binding) => {
                self.collect_target_mutations(&binding.target, defined, mutated);
                self.collect_binding_value_mutations(&binding.value, defined, mutated);
            }
            Statement::Expression(expression) => {
                self.collect_expression_mutations(expression, defined, mutated);
            }
        }
    }

    fn collect_target_mutations(
        &self,
        target: &Target,
        defined: &mut HashSet<String>,
        mutated: &mut HashSet<String>,
    ) {
        match target {
            Target::TypedIdentifier(typed_identifier) => {
                let text = typed_identifier.label.text();
                if typed_identifier.r#type.is_none() && defined.contains(&text) {
                    mutated.insert(text);
                } else {
                    defined.insert(text);
                }
            }
            Target::Reference(typed_identifier) => {
                mutated.insert(typed_identifier.label.text());
            }
            Target::Collection(targets) => {
                for target in targets {
                    self.collect_target_mutations(target, defined, mutated);
                }
            }
        }
    }

    fn collect_binding_value_mutations(
        &self,
        value: &BindingValue,
        defined: &mut HashSet<String>,
        mutated: &mut HashSet<String>,
    ) {
        match value {
            BindingValue::Expression(expression) => {
                self.collect_expression_mutations(expression, defined, mutated);
            }
            BindingValue::FunctionDefinition(function_definition) => {
                self.collect_function_mutations(function_definition, defined, mutated);
            }
        }
    }

    fn collect_function_mutations(
        &self,
        function_definition: &FunctionDefinition,
        defined: &mut HashSet<String>,
        mutated: &mut HashSet<String>,
    ) {
        let mut function_defined = defined.clone();
        if let Some(parameter_list) = &function_definition.parameters {
            for parameter in &parameter_list.parameters {
                function_defined.insert(parameter.label.text());
            }
        }
        self.collect_block_mutations(&function_definition.block, &mut function_defined, mutated);
    }

    fn collect_block_mutations(
        &self,
        block: &Block,
        defined: &mut HashSet<String>,
        mutated: &mut HashSet<String>,
    ) {
        let mut block_defined = defined.clone();
        for statement in &block.statements {
            self.collect_statement_mutations(statement, &mut block_defined, mutated);
        }
        if let Some(expression) = &block.expression {
            self.collect_expression_mutations(expression, &mut block_defined, mutated);
        }
    }

    fn collect_expression_mutations(
        &self,
        expression: &Expression,
        defined: &mut HashSet<String>,
        mutated: &mut HashSet<String>,
    ) {
        match expression {
            Expression::Primary(primary) => {
                if let Primary::Reference(label) = primary {
                    mutated.insert(label.text());
                }
            }
            Expression::Extern(r#extern) => {
                for expression in &r#extern.expressions {
                    self.collect_expression_mutations(expression, defined, mutated);
                }
            }
            Expression::Branch(branch) => {
                self.collect_expression_mutations(&branch.expression, defined, mutated);
                self.collect_block_mutations(&branch.left, defined, mutated);
                self.collect_block_mutations(&branch.right, defined, mutated);
            }
            Expression::Nand(nand) => {
                self.collect_expression_mutations(&nand.left, defined, mutated);
                self.collect_expression_mutations(&nand.right, defined, mutated);
            }
            Expression::Collection(collection) => {
                for expression in &collection.expressions {
                    self.collect_expression_mutations(expression, defined, mutated);
                }
            }
            Expression::Call(call) => {
                for expression in &call.expressions {
                    self.collect_expression_mutations(expression, defined, mutated);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one() -> Expression {
        Expression::Primary(Primary::Bit(BitKeyword { value: Some(true) }))
    }

    fn with_one(source: &str) -> Program {
        let mut program = Program::parse(source).unwrap();
        program.statements.insert(
            0,
            Statement::Binding(Binding {
                target: Target::parse("ONE: BIT").unwrap(),
                value: BindingValue::Expression(one()),
            }),
        );
        program
    }

    fn emitted(program: &Program) -> Result<String, Vec<ProgramError>> {
        program
            .emit()
            .map(|output| output.strip_prefix(RUNTIME).unwrap().to_string())
    }

    fn emitted_without_erasure(program: &Program) -> Result<String, Vec<ProgramError>> {
        program
            .emit_without_erasure()
            .map(|output| output.strip_prefix(RUNTIME).unwrap().to_string())
    }

    fn word_program(definition: &str) -> String {
        let bits = vec!["BIT"; 64].join(", ");
        format!("liblapzt.u64.erased = [{bits}]\nliblapzt.u64 = liblapzt.u64.erased\n{definition}")
    }

    #[test]
    pub fn test_comment_parse() {
        assert_eq!(
            Comment {
                comment: "//".to_string()
            },
            Comment::parse("//").unwrap()
        );
        assert_eq!(
            Comment {
                comment: "//A".to_string()
            },
            Comment::parse("//A").unwrap()
        );
        assert_eq!(
            Comment {
                comment: "// ".to_string()
            },
            Comment::parse("// ").unwrap()
        );
        assert_eq!(
            Comment {
                comment: "// \n".to_string()
            },
            Comment::parse("// \n").unwrap()
        );
        assert_eq!(
            Comment {
                comment: "// A ".to_string()
            },
            Comment::parse("// A ").unwrap()
        );
        assert!(Comment::parse("").is_err());
        assert!(Comment::parse(" ").is_err());
        assert!(Comment::parse("/").is_err());
        assert!(Comment::parse(" //").is_err());
    }

    #[test]
    pub fn test_bit_parse() {
        assert_eq!(
            BitKeyword { value: None },
            BitKeyword::parse("BIT").unwrap()
        );
        assert!(BitKeyword::parse("bit").is_err());
        assert!(BitKeyword::parse("").is_err());
    }

    #[test]
    pub fn test_nand_parse() {
        assert_eq!(NandKeyword {}, NandKeyword::parse("NAND").unwrap());
        assert!(NandKeyword::parse("nand").is_err());
        assert!(NandKeyword::parse("").is_err());
    }

    #[test]
    pub fn test_branch_parse() {
        assert_eq!(BranchKeyword {}, BranchKeyword::parse("BRANCH").unwrap());
        assert!(BranchKeyword::parse("branch").is_err());
        assert!(BranchKeyword::parse("").is_err());
    }

    #[test]
    pub fn test_extern_parse() {
        assert_eq!(ExternKeyword {}, ExternKeyword::parse("EXTERN").unwrap());
        assert!(ExternKeyword::parse("extern").is_err());
        assert!(ExternKeyword::parse("").is_err());
    }

    #[test]
    pub fn test_keyword_parse() {
        assert_eq!(
            Keyword::Bit(BitKeyword { value: None }),
            Keyword::parse("BIT").unwrap()
        );
        assert_eq!(
            Keyword::Nand(NandKeyword {}),
            Keyword::parse("NAND").unwrap()
        );
        assert_eq!(
            Keyword::Branch(BranchKeyword {}),
            Keyword::parse("BRANCH").unwrap()
        );
        assert_eq!(
            Keyword::Extern(ExternKeyword {}),
            Keyword::parse("EXTERN").unwrap()
        );
        assert!(Keyword::parse("a").is_err());
        assert!(Keyword::parse(" ").is_err());
        assert!(Keyword::parse("").is_err());
    }

    #[test]
    pub fn test_label_segement_parse() {
        assert_eq!(
            LabelSegement {
                segment: "u8".to_string()
            },
            LabelSegement::parse("u8").unwrap()
        );
        assert_eq!(
            LabelSegement {
                segment: "_u8".to_string()
            },
            LabelSegement::parse("_u8").unwrap()
        );
        assert_eq!(
            LabelSegement {
                segment: "u8_".to_string()
            },
            LabelSegement::parse("u8_").unwrap()
        );
        assert!(LabelSegement::parse("__").is_err());
        assert!(LabelSegement::parse("_").is_err());
        assert!(LabelSegement::parse("").is_err());
    }

    #[test]
    pub fn test_label_parse() {
        assert_eq!(
            Label {
                segments: vec![LabelSegement {
                    segment: "u8".to_string()
                }]
            },
            Label::parse("u8").unwrap()
        );
        assert_eq!(
            Label {
                segments: vec![
                    LabelSegement {
                        segment: "a".to_string()
                    },
                    LabelSegement {
                        segment: "b".to_string()
                    }
                ]
            },
            Label::parse("a.b").unwrap()
        );
        assert!(Label::parse("").is_err());
        assert!(Label::parse(".").is_err());
        assert!(Label::parse("a.").is_err());
        assert!(Label::parse(".a").is_err());
        assert!(Label::parse("a..b").is_err());
        assert!(Label::parse("a._").is_err());
    }

    #[test]
    pub fn test_type_parse() {
        assert_eq!(
            Type::Bit(BitKeyword { value: None }),
            Type::parse("BIT").unwrap()
        );
        assert_eq!(
            Type::Label(Label {
                segments: vec![LabelSegement {
                    segment: "u8".to_string()
                }]
            }),
            Type::parse("u8").unwrap()
        );
        assert_eq!(
            Type::Reference(Box::new(Type::Bit(BitKeyword { value: None }))),
            Type::parse("*BIT").unwrap()
        );
        assert_eq!(
            Type::Reference(Box::new(Type::Label(Label {
                segments: vec![LabelSegement {
                    segment: "u8".to_string()
                }]
            }))),
            Type::parse("*u8").unwrap()
        );
        assert_eq!(Type::Collection(vec![]), Type::parse("[]").unwrap());
        assert_eq!(
            Type::Collection(vec![
                Type::Bit(BitKeyword { value: None }),
                Type::Label(Label {
                    segments: vec![LabelSegement {
                        segment: "u8".to_string()
                    }]
                })
            ]),
            Type::parse("[BIT, u8]").unwrap()
        );
        assert!(Type::parse("").is_err());
        assert!(Type::parse("*").is_err());
        assert!(Type::parse("[BIT").is_err());
        assert!(Type::parse("BIT]").is_err());
    }

    #[test]
    pub fn test_typed_identifier_parse() {
        assert_eq!(
            TypedIdentifier {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                },
                r#type: None
            },
            TypedIdentifier::parse("a").unwrap()
        );
        assert_eq!(
            TypedIdentifier {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                },
                r#type: Some(Type::Bit(BitKeyword { value: None }))
            },
            TypedIdentifier::parse("a: BIT").unwrap()
        );
        assert_eq!(
            TypedIdentifier {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                },
                r#type: Some(Type::Label(Label {
                    segments: vec![LabelSegement {
                        segment: "u8".to_string()
                    }]
                }))
            },
            TypedIdentifier::parse("a:u8").unwrap()
        );
        assert!(TypedIdentifier::parse("").is_err());
        assert!(TypedIdentifier::parse(":").is_err());
        assert!(TypedIdentifier::parse("a:").is_err());
        assert!(TypedIdentifier::parse(":BIT").is_err());
    }

    #[test]
    pub fn test_parameter_parse() {
        assert_eq!(
            Parameter {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                },
                r#type: Type::Bit(BitKeyword { value: None })
            },
            Parameter::parse("a: BIT").unwrap()
        );
        assert_eq!(
            Parameter {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                },
                r#type: Type::Label(Label {
                    segments: vec![LabelSegement {
                        segment: "u8".to_string()
                    }]
                })
            },
            Parameter::parse("a:u8").unwrap()
        );
        assert!(Parameter::parse("a").is_err());
        assert!(Parameter::parse("").is_err());
        assert!(Parameter::parse(":").is_err());
        assert!(Parameter::parse("a:").is_err());
        assert!(Parameter::parse(":BIT").is_err());
    }

    #[test]
    pub fn test_target_parse() {
        assert_eq!(
            Target::TypedIdentifier(TypedIdentifier {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                },
                r#type: None
            }),
            Target::parse("a").unwrap()
        );
        assert_eq!(
            Target::TypedIdentifier(TypedIdentifier {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                },
                r#type: Some(Type::Bit(BitKeyword { value: None }))
            }),
            Target::parse("a: BIT").unwrap()
        );
        assert_eq!(
            Target::Reference(TypedIdentifier {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                },
                r#type: None
            }),
            Target::parse("*a").unwrap()
        );
        assert_eq!(
            Target::Reference(TypedIdentifier {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                },
                r#type: Some(Type::Bit(BitKeyword { value: None }))
            }),
            Target::parse("*a: BIT").unwrap()
        );
        assert_eq!(
            Target::Collection(vec![
                Target::TypedIdentifier(TypedIdentifier {
                    label: Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    },
                    r#type: None
                }),
                Target::Reference(TypedIdentifier {
                    label: Label {
                        segments: vec![LabelSegement {
                            segment: "b".to_string()
                        }]
                    },
                    r#type: Some(Type::Bit(BitKeyword { value: None }))
                })
            ]),
            Target::parse("[a, *b: BIT]").unwrap()
        );
        assert!(Target::parse("").is_err());
        assert!(Target::parse("*").is_err());
        assert!(Target::parse("**a").is_err());
        assert!(Target::parse("[]").is_err());
        assert!(Target::parse("[a").is_err());
        assert!(Target::parse("a]").is_err());
    }

    #[test]
    pub fn test_primary_parse() {
        assert_eq!(
            Primary::Bit(BitKeyword { value: None }),
            Primary::parse("BIT").unwrap()
        );
        assert_eq!(
            Primary::Label(Label {
                segments: vec![LabelSegement {
                    segment: "a".to_string()
                }]
            }),
            Primary::parse("a").unwrap()
        );
        assert_eq!(
            Primary::Reference(Label {
                segments: vec![LabelSegement {
                    segment: "a".to_string()
                }]
            }),
            Primary::parse("*a").unwrap()
        );
        assert!(Primary::parse("").is_err());
        assert!(Primary::parse("*").is_err());
        assert!(Primary::parse("*BIT").is_err());
        assert!(Primary::parse("NAND").is_err());
    }

    #[test]
    pub fn test_nand_expression_parse() {
        assert_eq!(
            NandExpression {
                left: Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                    value: None
                }))),
                right: Box::new(Expression::Primary(Primary::Label(Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                })))
            },
            NandExpression::parse("NAND(BIT, a)").unwrap()
        );
        assert!(NandExpression::parse("").is_err());
        assert!(NandExpression::parse("NAND()").is_err());
        assert!(NandExpression::parse("NAND(BIT)").is_err());
        assert!(NandExpression::parse("NAND(BIT, a, b)").is_err());
    }

    #[test]
    pub fn test_branch_expression_parse() {
        assert_eq!(
            BranchExpression {
                expression: Box::new(Expression::Primary(Primary::Label(Label {
                    segments: vec![LabelSegement {
                        segment: "flag".to_string()
                    }]
                }))),
                left: Block {
                    statements: vec![],
                    expression: Some(Box::new(Expression::Primary(Primary::Label(Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    }))))
                },
                right: Block {
                    statements: vec![],
                    expression: Some(Box::new(Expression::Primary(Primary::Label(Label {
                        segments: vec![LabelSegement {
                            segment: "b".to_string()
                        }]
                    }))))
                }
            },
            BranchExpression::parse("BRANCH (flag) { a } { b }").unwrap()
        );
        assert!(BranchExpression::parse("").is_err());
        assert!(BranchExpression::parse("BRANCH").is_err());
        assert!(BranchExpression::parse("BRANCH (flag) { a }").is_err());
    }

    #[test]
    pub fn test_extern_expression_parse() {
        assert_eq!(
            ExternExpression {
                expressions: vec![]
            },
            ExternExpression::parse("EXTERN()").unwrap()
        );
        assert_eq!(
            ExternExpression {
                expressions: vec![
                    Expression::Primary(Primary::Bit(BitKeyword { value: None })),
                    Expression::Primary(Primary::Label(Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    }))
                ]
            },
            ExternExpression::parse("EXTERN(BIT, a)").unwrap()
        );
        assert!(ExternExpression::parse("").is_err());
        assert!(ExternExpression::parse("EXTERN").is_err());
        assert!(ExternExpression::parse("EXTERN(BIT").is_err());
    }

    #[test]
    pub fn test_collection_expression_parse() {
        assert_eq!(
            CollectionExpression {
                expressions: vec![]
            },
            CollectionExpression::parse("[]").unwrap()
        );
        assert_eq!(
            CollectionExpression {
                expressions: vec![
                    Expression::Primary(Primary::Bit(BitKeyword { value: None })),
                    Expression::Primary(Primary::Label(Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    }))
                ]
            },
            CollectionExpression::parse("[BIT, a]").unwrap()
        );
        assert!(CollectionExpression::parse("").is_err());
        assert!(CollectionExpression::parse("[BIT").is_err());
        assert!(CollectionExpression::parse("BIT]").is_err());
    }

    #[test]
    pub fn test_call_expression_parse() {
        assert_eq!(
            CallExpression {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "foo".to_string()
                    }]
                },
                expressions: vec![]
            },
            CallExpression::parse("foo()").unwrap()
        );
        assert_eq!(
            CallExpression {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "foo".to_string()
                    }]
                },
                expressions: vec![
                    Expression::Primary(Primary::Bit(BitKeyword { value: None })),
                    Expression::Primary(Primary::Label(Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    }))
                ]
            },
            CallExpression::parse("foo(BIT, a)").unwrap()
        );
        assert!(CallExpression::parse("").is_err());
        assert!(CallExpression::parse("foo").is_err());
        assert!(CallExpression::parse("NAND(BIT)").is_err());
        assert!(CallExpression::parse("EXTERN()").is_err());
    }

    #[test]
    pub fn test_expression_parse() {
        assert_eq!(
            Expression::Primary(Primary::Bit(BitKeyword { value: None })),
            Expression::parse("BIT").unwrap()
        );
        assert_eq!(
            Expression::Primary(Primary::Label(Label {
                segments: vec![LabelSegement {
                    segment: "a".to_string()
                }]
            })),
            Expression::parse("a").unwrap()
        );
        assert_eq!(
            Expression::Primary(Primary::Reference(Label {
                segments: vec![LabelSegement {
                    segment: "a".to_string()
                }]
            })),
            Expression::parse("*a").unwrap()
        );
        assert_eq!(
            Expression::Extern(ExternExpression {
                expressions: vec![Expression::Primary(Primary::Bit(BitKeyword {
                    value: None
                }))]
            }),
            Expression::parse("EXTERN(BIT)").unwrap()
        );
        assert_eq!(
            Expression::Branch(BranchExpression {
                expression: Box::new(Expression::Primary(Primary::Label(Label {
                    segments: vec![LabelSegement {
                        segment: "flag".to_string()
                    }]
                }))),
                left: Block {
                    statements: vec![],
                    expression: Some(Box::new(Expression::Primary(Primary::Label(Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    }))))
                },
                right: Block {
                    statements: vec![],
                    expression: Some(Box::new(Expression::Primary(Primary::Label(Label {
                        segments: vec![LabelSegement {
                            segment: "b".to_string()
                        }]
                    }))))
                }
            }),
            Expression::parse("BRANCH (flag) { a } { b }").unwrap()
        );
        assert_eq!(
            Expression::Nand(NandExpression {
                left: Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                    value: None
                }))),
                right: Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                    value: None
                })))
            }),
            Expression::parse("NAND(BIT, BIT)").unwrap()
        );
        assert_eq!(
            Expression::Collection(CollectionExpression {
                expressions: vec![Expression::Primary(Primary::Bit(BitKeyword {
                    value: None
                }))]
            }),
            Expression::parse("[BIT]").unwrap()
        );
        assert_eq!(
            Expression::Call(CallExpression {
                label: Label {
                    segments: vec![LabelSegement {
                        segment: "foo".to_string()
                    }]
                },
                expressions: vec![Expression::Primary(Primary::Bit(BitKeyword {
                    value: None
                }))]
            }),
            Expression::parse("foo(BIT)").unwrap()
        );
        assert!(Expression::parse("").is_err());
        assert!(Expression::parse("NAND").is_err());
    }

    #[test]
    pub fn test_parameter_list_parse() {
        assert_eq!(
            ParameterList {
                parameters: vec![
                    Parameter {
                        label: Label {
                            segments: vec![LabelSegement {
                                segment: "a".to_string()
                            }]
                        },
                        r#type: Type::Bit(BitKeyword { value: None })
                    },
                    Parameter {
                        label: Label {
                            segments: vec![LabelSegement {
                                segment: "b".to_string()
                            }]
                        },
                        r#type: Type::Label(Label {
                            segments: vec![LabelSegement {
                                segment: "u8".to_string()
                            }]
                        })
                    }
                ]
            },
            ParameterList::parse("a: BIT, b: u8").unwrap()
        );
        assert!(ParameterList::parse("").is_err());
        assert!(ParameterList::parse("a").is_err());
    }

    #[test]
    pub fn test_function_definition_parse() {
        assert_eq!(
            FunctionDefinition {
                parameters: None,
                r#type: Type::Bit(BitKeyword { value: None }),
                block: Block {
                    statements: vec![],
                    expression: Some(Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                        value: None
                    }))))
                }
            },
            FunctionDefinition::parse("() BIT { BIT }").unwrap()
        );
        assert_eq!(
            FunctionDefinition {
                parameters: Some(ParameterList {
                    parameters: vec![Parameter {
                        label: Label {
                            segments: vec![LabelSegement {
                                segment: "a".to_string()
                            }]
                        },
                        r#type: Type::Bit(BitKeyword { value: None })
                    }]
                }),
                r#type: Type::Bit(BitKeyword { value: None }),
                block: Block {
                    statements: vec![],
                    expression: Some(Box::new(Expression::Primary(Primary::Label(Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    }))))
                }
            },
            FunctionDefinition::parse("(a: BIT) BIT { a }").unwrap()
        );
        assert!(FunctionDefinition::parse("").is_err());
        assert!(FunctionDefinition::parse("() { BIT }").is_err());
        assert!(FunctionDefinition::parse("() BIT").is_err());
    }

    #[test]
    pub fn test_binding_value_parse() {
        assert_eq!(
            BindingValue::Expression(Expression::Primary(Primary::Bit(BitKeyword {
                value: None
            }))),
            BindingValue::parse("BIT").unwrap()
        );
        assert_eq!(
            BindingValue::FunctionDefinition(FunctionDefinition {
                parameters: None,
                r#type: Type::Bit(BitKeyword { value: None }),
                block: Block {
                    statements: vec![],
                    expression: Some(Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                        value: None
                    }))))
                }
            }),
            BindingValue::parse("() BIT { BIT }").unwrap()
        );
        assert!(BindingValue::parse("").is_err());
    }

    #[test]
    pub fn test_binding_parse() {
        assert_eq!(
            Binding {
                target: Target::TypedIdentifier(TypedIdentifier {
                    label: Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    },
                    r#type: Some(Type::Bit(BitKeyword { value: None }))
                }),
                value: BindingValue::Expression(Expression::Primary(Primary::Bit(BitKeyword {
                    value: None
                })))
            },
            Binding::parse("a: BIT = BIT").unwrap()
        );
        assert!(Binding::parse("").is_err());
        assert!(Binding::parse("a =").is_err());
        assert!(Binding::parse("= BIT").is_err());
    }

    #[test]
    pub fn test_statement_parse() {
        assert_eq!(
            Statement::Binding(Binding {
                target: Target::TypedIdentifier(TypedIdentifier {
                    label: Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    },
                    r#type: None
                }),
                value: BindingValue::Expression(Expression::Primary(Primary::Bit(BitKeyword {
                    value: None
                })))
            }),
            Statement::parse("a = BIT").unwrap()
        );
        assert_eq!(
            Statement::Expression(Expression::Primary(Primary::Bit(BitKeyword {
                value: None
            }))),
            Statement::parse("BIT").unwrap()
        );
        assert!(Statement::parse("").is_err());
    }

    #[test]
    pub fn test_block_parse() {
        assert_eq!(
            Block {
                statements: vec![],
                expression: None
            },
            Block::parse("{}").unwrap()
        );
        assert_eq!(
            Block {
                statements: vec![],
                expression: Some(Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                    value: None
                }))))
            },
            Block::parse("{ BIT }").unwrap()
        );
        assert_eq!(
            Block {
                statements: vec![Statement::Binding(Binding {
                    target: Target::TypedIdentifier(TypedIdentifier {
                        label: Label {
                            segments: vec![LabelSegement {
                                segment: "a".to_string()
                            }]
                        },
                        r#type: None
                    }),
                    value: BindingValue::Expression(Expression::Primary(Primary::Bit(
                        BitKeyword { value: None }
                    )))
                })],
                expression: Some(Box::new(Expression::Primary(Primary::Label(Label {
                    segments: vec![LabelSegement {
                        segment: "a".to_string()
                    }]
                }))))
            },
            Block::parse("{\n a = BIT\n a\n}").unwrap()
        );
        assert!(Block::parse("").is_err());
        assert!(Block::parse("{").is_err());
        assert!(Block::parse("}").is_err());
    }

    #[test]
    pub fn test_program_parse() {
        assert_eq!(Program { statements: vec![] }, Program::parse("").unwrap());
        assert_eq!(
            Program {
                statements: vec![
                    Statement::Binding(Binding {
                        target: Target::TypedIdentifier(TypedIdentifier {
                            label: Label {
                                segments: vec![LabelSegement {
                                    segment: "a".to_string()
                                }]
                            },
                            r#type: None
                        }),
                        value: BindingValue::Expression(Expression::Primary(Primary::Bit(
                            BitKeyword { value: None }
                        )))
                    }),
                    Statement::Expression(Expression::Primary(Primary::Label(Label {
                        segments: vec![LabelSegement {
                            segment: "a".to_string()
                        }]
                    })))
                ]
            },
            Program::parse("// comment\na = BIT // inline\na").unwrap()
        );
        assert!(Program::parse("a =").is_err());
        assert!(Program::parse("= BIT").is_err());
    }

    #[test]
    pub fn test_program_parse_multiline() {
        assert_eq!(
            Program {
                statements: vec![Statement::Binding(Binding {
                    target: Target::parse("main").unwrap(),
                    value: BindingValue::FunctionDefinition(
                        FunctionDefinition::parse("() [] {\n []\n}").unwrap()
                    )
                })]
            },
            Program::parse("main = () [] {\n []\n}").unwrap()
        );
    }

    #[test]
    pub fn test_program_main() {
        assert_eq!(
            vec![ProgramError::InvalidMain("main must return []".to_string())],
            with_one("main = () BIT { ONE }").errors()
        );
        assert_eq!(
            vec![ProgramError::InvalidMain(
                "main must take no parameters".to_string()
            )],
            Program::parse("main = (a: BIT) [] { [] }")
                .unwrap()
                .errors()
        );
        assert_eq!(
            vec![ProgramError::InvalidMain(
                "main must be a function".to_string()
            )],
            Program::parse("main = BIT").unwrap().errors()
        );
        assert_eq!(
            Vec::<ProgramError>::new(),
            Program::parse("main = () [] { [] }").unwrap().errors()
        );
    }

    #[test]
    pub fn test_program_errors() {
        assert_eq!(Vec::<ProgramError>::new(), with_one("").errors());
        assert_eq!(Vec::<ProgramError>::new(), with_one("a = BIT").errors());
        assert_eq!(
            vec![ProgramError::UndefinedLabel("a".to_string())],
            with_one("a").errors()
        );
        assert_eq!(
            vec![ProgramError::UndefinedLabel("a".to_string())],
            with_one("*a").errors()
        );
        assert_eq!(
            vec![ProgramError::UndefinedLabel("a".to_string())],
            with_one("*a = ONE").errors()
        );
        assert_eq!(
            vec![ProgramError::UndefinedLabel("U2".to_string())],
            with_one("a: U2 = ONE").errors()
        );
        assert_eq!(
            vec![ProgramError::UndefinedLabel("f".to_string())],
            with_one("f()").errors()
        );
        assert_eq!(
            vec![ProgramError::InvalidExtern(
                "unknown operation: a".to_string()
            )],
            with_one("EXTERN(a)").errors()
        );
        assert_eq!(
            vec![ProgramError::InvalidExtern(
                "first argument must be an operation constant".to_string()
            )],
            with_one("EXTERN()").errors()
        );
        assert_eq!(
            vec![ProgramError::InvalidExtern(
                "OP.TIME.NOW takes 0 arguments".to_string()
            )],
            with_one("EXTERN(OP.TIME.NOW, ONE)").errors()
        );
        assert_eq!(
            vec![ProgramError::InvalidExtern(
                "OP.FILE.OPEN takes 4 arguments".to_string()
            )],
            with_one("EXTERN(OP.FILE.OPEN, ONE)").errors()
        );
        assert_eq!(
            vec![ProgramError::DuplicateLabel("a".to_string())],
            with_one("a: BIT = ONE\na: BIT = ONE").errors()
        );
        assert_eq!(
            vec![ProgramError::DuplicateLabel("a".to_string())],
            with_one("a = BIT\na: BIT = ONE").errors()
        );
        assert_eq!(
            vec![ProgramError::DuplicateLabel("a".to_string())],
            with_one("a = BIT\na = ONE").errors()
        );
        assert_eq!(
            vec![ProgramError::DuplicateLabel("a".to_string())],
            with_one("a: BIT = ONE\na = ONE").errors()
        );
        assert_eq!(
            vec![ProgramError::DuplicateLabel("a".to_string())],
            with_one("a = BIT\nBRANCH (ONE) { a: BIT = ONE } { ONE }").errors()
        );
        let mut program = Program {
            statements: vec![Statement::Expression(
                Expression::parse("BRANCH (ONE) {\n a: BIT = ONE\n a: BIT = ONE\n ONE\n} { ONE }")
                    .unwrap(),
            )],
        };
        program.statements.insert(
            0,
            Statement::Binding(Binding {
                target: Target::parse("ONE: BIT").unwrap(),
                value: BindingValue::Expression(one()),
            }),
        );
        assert_eq!(
            vec![ProgramError::DuplicateLocal("a".to_string())],
            program.errors()
        );
        assert_eq!(
            vec![ProgramError::NotCallable("a".to_string())],
            with_one("a = BIT\na()").errors()
        );
        assert_eq!(
            vec![ProgramError::ArityMismatch("f".to_string())],
            with_one("f = (a: BIT) BIT { a }\nf()").errors()
        );
        assert_eq!(
            vec![ProgramError::ReferenceReturnType("*BIT".to_string())],
            with_one("f = () *BIT { ONE }").errors()
        );
        assert_eq!(
            vec![ProgramError::ReferenceInCollection("a".to_string())],
            with_one("a = BIT\n[*a]").errors()
        );
        assert_eq!(
            Vec::<ProgramError>::new(),
            with_one("a = BIT\n[a]").errors()
        );
        assert_eq!(
            Vec::<ProgramError>::new(),
            with_one("a: BIT = ONE\nalias: *BIT = *a").errors()
        );
        assert_eq!(
            Vec::<ProgramError>::new(),
            with_one("a: BIT = ONE\nalias: *BIT = *a\nalias = ONE").errors()
        );
        assert_eq!(
            Vec::<ProgramError>::new(),
            with_one("f = (a: BIT) BIT { a }\nf(ONE)").errors()
        );
    }

    #[test]
    pub fn test_program_emit_constants() {
        let one = Expression::Nand(NandExpression {
            left: Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                value: None,
            }))),
            right: Box::new(Expression::parse("NAND(BIT, BIT)").unwrap()),
        });
        let program = Program {
            statements: vec![
                Statement::Binding(Binding {
                    target: Target::parse("BIT.ONE").unwrap(),
                    value: BindingValue::Expression(one),
                }),
                Statement::parse("BIT.ZERO = NAND(BIT.ONE, BIT.ONE)").unwrap(),
            ],
        };
        assert_eq!(
            Ok("pub const @\"BIT.ONE\" = true;\npub const @\"BIT.ZERO\" = false;\n".to_string()),
            emitted(&program)
        );
    }

    #[test]
    pub fn test_program_emit_types() {
        assert_eq!(
            Ok("pub const @\"U2\" = struct { bool, bool };\npub const @\"PAIR\" = struct { bool, bool };\n".to_string()),
            emitted(&Program::parse("U2 = [BIT, BIT]\nPAIR = U2").unwrap())
        );
    }

    #[test]
    pub fn test_program_emit_collections() {
        let mut program = Program::parse("bundle = [one, one]").unwrap();
        program.statements.insert(
            0,
            Statement::Binding(Binding {
                target: Target::parse("one").unwrap(),
                value: BindingValue::Expression(Expression::Nand(NandExpression {
                    left: Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                        value: None,
                    }))),
                    right: Box::new(Expression::parse("NAND(BIT, BIT)").unwrap()),
                })),
            }),
        );
        assert_eq!(
            Ok(
                "pub const @\"one\" = true;\npub const @\"bundle\" = .{ true, true };\n"
                    .to_string()
            ),
            emitted(&program)
        );
    }

    #[test]
    pub fn test_program_emit_functions() {
        assert_eq!(
            Ok(
                "pub fn @\"bit.not\"(@\"a\": bool) bool {\n    return !(@\"a\" and @\"a\");\n}\n"
                    .to_string()
            ),
            emitted(&Program::parse("bit.not = (a: BIT) BIT { NAND(a, a) }").unwrap())
        );
    }

    #[test]
    pub fn test_program_emit_references() {
        assert_eq!(
            Ok("pub fn @\"bit.not\"(@\"a\": bool) bool {\n    return !(@\"a\" and @\"a\");\n}\npub fn @\"bit.toggle\"(@\"target\": *bool) @\"$lapzt\".@\"unit\" {\n    return @\"bit.not\"(@\"target\".*);\n}\n".to_string()),
            emitted(&Program::parse("bit.not = (a: BIT) BIT { NAND(a, a) }\nbit.toggle = (target: *BIT) [] { bit.not(target) }").unwrap())
        );
    }

    #[test]
    pub fn test_program_emit_branch() {
        let mut program = Program::parse(
            "BIT.ZERO = NAND(BIT.ONE, BIT.ONE)\nselect = (flag: BIT) BIT { BRANCH (flag) { BIT.ONE } { BIT.ZERO } }",
        )
        .unwrap();
        program.statements.insert(
            0,
            Statement::Binding(Binding {
                target: Target::parse("BIT.ONE").unwrap(),
                value: BindingValue::Expression(Expression::Nand(NandExpression {
                    left: Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                        value: None,
                    }))),
                    right: Box::new(Expression::parse("NAND(BIT, BIT)").unwrap()),
                })),
            }),
        );
        assert_eq!(
            Ok("pub const @\"BIT.ONE\" = true;\npub const @\"BIT.ZERO\" = false;\npub fn @\"select\"(@\"flag\": bool) bool {\n    return (if (@\"flag\") blk0: { break :blk0 true; } else blk1: { break :blk1 false; });\n}\n".to_string()),
            emitted(&program)
        );
    }

    #[test]
    pub fn test_program_emit_destructuring() {
        let mut program = Program::parse("[head, last] = [one, one]").unwrap();
        program.statements.insert(
            0,
            Statement::Binding(Binding {
                target: Target::parse("one").unwrap(),
                value: BindingValue::Expression(Expression::Nand(NandExpression {
                    left: Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                        value: None,
                    }))),
                    right: Box::new(Expression::parse("NAND(BIT, BIT)").unwrap()),
                })),
            }),
        );
        assert_eq!(
            Ok("pub const @\"one\" = true;\nconst @\"$0\" = .{ true, true };\npub const @\"head\" = @\"$0\"[0];\npub const @\"last\" = @\"$0\"[1];\n".to_string()),
            emitted(&program)
        );
    }

    #[test]
    pub fn test_program_emit_extern() {
        assert_eq!(
            Ok("pub fn @\"heap.alloc\"(@\"size\": u64) struct { bool, u64 } {\n    return @\"$lapzt\".@\"OP.MEMORY.ACQUIRE\"(@\"size\");\n}\n".to_string()),
            emitted(&Program::parse("heap.alloc = (size: U64) [BIT, U64] { EXTERN(OP.MEMORY.ACQUIRE, size) }").unwrap())
        );
    }

    #[test]
    pub fn test_program_emit_extern_file_open() {
        let output = emitted_without_erasure(&Program::parse(&word_program(
            "f = (path: liblapzt.u64, mode: liblapzt.u64) [BIT, liblapzt.u64] { EXTERN(OP.FILE.OPEN, path, path, path, mode) }",
        ))
        .unwrap())
        .unwrap();
        assert!(output.contains("@\"$lapzt\".@\"OP.FILE.OPEN\""));
        assert!(output.contains("break :blk .{ @\"$0\"[0], .{"));
    }

    #[test]
    pub fn test_program_emit_extern_file_close() {
        let output = emitted_without_erasure(
            &Program::parse(&word_program(
                "f = (handle: liblapzt.u64) [BIT] { EXTERN(OP.FILE.CLOSE, handle) }",
            ))
            .unwrap(),
        )
        .unwrap();
        assert!(output.contains("@\"$lapzt\".@\"OP.FILE.CLOSE\"("));
        assert!(output.contains(")[0] };"));
    }

    #[test]
    pub fn test_program_emit_extern_file_seek() {
        let output = emitted_without_erasure(&Program::parse(&word_program(
            "f = (handle: liblapzt.u64, offset: liblapzt.u64, whence: liblapzt.u64) [BIT] { EXTERN(OP.FILE.SEEK, handle, offset, whence) }",
        ))
        .unwrap())
        .unwrap();
        assert!(output.contains("@\"$lapzt\".@\"OP.FILE.SEEK\"("));
        assert!(output.contains(")[0] };"));
    }

    #[test]
    pub fn test_program_emit_extern_time_now() {
        let output = emitted_without_erasure(
            &Program::parse(&word_program("f = () liblapzt.u64 { EXTERN(OP.TIME.NOW) }")).unwrap(),
        )
        .unwrap();
        assert!(output.contains("const @\"$0\" = @\"$lapzt\".@\"OP.TIME.NOW\"();"));
        assert!(output.contains("break :blk .{"));
    }

    #[test]
    pub fn test_program_emit_extern_time_sleep() {
        let output = emitted_without_erasure(
            &Program::parse(&word_program(
                "f = (nanoseconds: liblapzt.u64) [] { EXTERN(OP.TIME.SLEEP, nanoseconds) }",
            ))
            .unwrap(),
        )
        .unwrap();
        assert!(output.contains("return @\"$lapzt\".@\"OP.TIME.SLEEP\"("));
    }

    #[test]
    pub fn test_program_emit_extern_random() {
        let output = emitted_without_erasure(
            &Program::parse(&word_program("f = () liblapzt.u64 { EXTERN(OP.RANDOM) }")).unwrap(),
        )
        .unwrap();
        assert!(output.contains("const @\"$0\" = @\"$lapzt\".@\"OP.RANDOM\"();"));
        assert!(output.contains("break :blk .{"));
    }

    #[test]
    pub fn test_program_emit_erased_word() {
        let bits = vec!["BIT"; 64].join(", ");
        let source = format!(
            "liblapzt.u64.erased = [{bits}]\nliblapzt.u64 = liblapzt.u64.erased\nf = (word: liblapzt.u64) [BIT, liblapzt.u64] {{ EXTERN(OP.READ, word, word) }}"
        );
        assert_eq!(
            Ok("pub const @\"liblapzt.u64.erased\" = u64;\npub const @\"liblapzt.u64\" = u64;\npub fn @\"f\"(@\"word\": u64) struct { bool, u64 } {\n    return @\"$lapzt\".@\"OP.READ\"(@\"word\", @\"word\");\n}\n".to_string()),
            emitted(&Program::parse(&source).unwrap())
        );
    }

    #[test]
    pub fn test_program_emit_signed_erasure() {
        let source = "ONE: BIT = NAND(BIT, NAND(BIT, BIT))\nliblapzt.u1.erased = BIT\nliblapzt.u1 = liblapzt.u1.erased\nliblapzt.u1.zero.erased = NAND(ONE, ONE)\nliblapzt.u1.one.erased = NAND(BIT, NAND(BIT, BIT))\nliblapzt.u1.zero = liblapzt.u1.zero.erased\nliblapzt.u1.one = liblapzt.u1.one.erased\nliblapzt.i2.erased = [liblapzt.u1.erased, liblapzt.u1.erased]\nliblapzt.i2 = liblapzt.i2.erased\nliblapzt.i2.zero.erased: liblapzt.i2.erased = [liblapzt.u1.zero.erased, liblapzt.u1.zero.erased]\nliblapzt.i2.zero: liblapzt.i2 = liblapzt.i2.zero.erased\nmy.int = liblapzt.i2\nf = (a: my.int) my.int {\n    [low, high] = a\n    [low, high]\n}";
        assert_eq!(
            Ok("pub const @\"ONE\": bool = true;\npub const @\"liblapzt.u1.erased\" = bool;\npub const @\"liblapzt.u1\" = bool;\npub const @\"liblapzt.u1.zero.erased\" = false;\npub const @\"liblapzt.u1.one.erased\" = true;\npub const @\"liblapzt.u1.zero\" = false;\npub const @\"liblapzt.u1.one\" = true;\npub const @\"liblapzt.i2.erased\" = i2;\npub const @\"liblapzt.i2\" = i2;\npub const @\"liblapzt.i2.zero.erased\": i2 = @as(i2, @bitCast(@as(u2, 0)));\npub const @\"liblapzt.i2.zero\": i2 = @as(i2, @bitCast(@as(u2, 0)));\npub const @\"my.int\" = i2;\npub fn @\"f\"(@\"a\": i2) i2 {\n    const @\"$0\" = @\"a\"; const @\"low\" = @as(bool, @bitCast(@as(u1, @truncate(@as(u2, @bitCast(@\"$0\")) >> 0)))); const @\"high\" = @as(bool, @bitCast(@as(u1, @truncate(@as(u2, @bitCast(@\"$0\")) >> 1))));\n    return @as(i2, @bitCast((@as(u2, @intFromBool(@\"low\")) << 0) | (@as(u2, @intFromBool(@\"high\")) << 1)));\n}\n".to_string()),
            emitted(&Program::parse(source).unwrap())
        );
    }

    #[test]
    pub fn test_program_emit_float_erasure() {
        let source = "ONE: BIT = NAND(BIT, NAND(BIT, BIT))\nZERO: BIT = NAND(ONE, ONE)\nliblapzt.bool.erased = BIT\nliblapzt.bool = liblapzt.bool.erased\nliblapzt.bool.false = ZERO\nliblapzt.u8.erased = [BIT, BIT, BIT, BIT, BIT, BIT, BIT, BIT]\nliblapzt.u8 = liblapzt.u8.erased\nliblapzt.f16.erased = [liblapzt.u8.erased, liblapzt.u8.erased]\nliblapzt.f16 = liblapzt.f16.erased\nliblapzt.f16.one.erased: liblapzt.f16.erased = [ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ONE, ONE, ONE, ZERO, ZERO]\nliblapzt.f16.one: liblapzt.f16 = liblapzt.f16.one.erased\nliblapzt.f16.add.erased = (a: liblapzt.f16, b: liblapzt.f16) liblapzt.f16 {\n    liblapzt.f16.one\n}\nliblapzt.f16.add = liblapzt.f16.add.erased\nliblapzt.f16.eq.erased = (a: liblapzt.f16, b: liblapzt.f16) liblapzt.bool {\n    liblapzt.bool.false\n}\nliblapzt.f16.eq = liblapzt.f16.eq.erased\nf = (a: liblapzt.f16, b: liblapzt.f16) liblapzt.bool {\n    liblapzt.f16.eq(liblapzt.f16.add(a, b), liblapzt.f16.one)\n}";
        assert_eq!(
            Ok("pub const @\"ONE\": bool = true;\npub const @\"ZERO\": bool = false;\npub const @\"liblapzt.bool.erased\" = bool;\npub const @\"liblapzt.bool\" = bool;\npub const @\"liblapzt.bool.false\" = false;\npub const @\"liblapzt.u8.erased\" = u8;\npub const @\"liblapzt.u8\" = u8;\npub const @\"liblapzt.f16.erased\" = f16;\npub const @\"liblapzt.f16\" = f16;\npub const @\"liblapzt.f16.one.erased\": f16 = @as(f16, @bitCast(@as(u16, 15360)));\npub const @\"liblapzt.f16.one\": f16 = @as(f16, @bitCast(@as(u16, 15360)));\npub fn @\"f\"(@\"a\": f16, @\"b\": f16) bool {\n    return ((@\"a\" + @\"b\") == @as(f16, @bitCast(@as(u16, 15360))));\n}\n".to_string()),
            emitted(&Program::parse(source).unwrap())
        );
    }

    #[test]
    pub fn test_program_emit_locals() {
        let mut program = Program {
            statements: vec![Statement::Binding(Binding {
                target: Target::TypedIdentifier(TypedIdentifier {
                    label: Label {
                        segments: vec![LabelSegement {
                            segment: "run".to_string()
                        }]
                    },
                    r#type: None
                }),
                value: BindingValue::FunctionDefinition(
                    FunctionDefinition::parse(
                        "() BIT {\n state: BIT = one\n alias: *BIT = *state\n alias = NAND(state, state)\n state\n}"
                    )
                    .unwrap()
                )
            })]
        };
        program.statements.insert(
            0,
            Statement::Binding(Binding {
                target: Target::parse("one").unwrap(),
                value: BindingValue::Expression(Expression::Nand(NandExpression {
                    left: Box::new(Expression::Primary(Primary::Bit(BitKeyword {
                        value: None,
                    }))),
                    right: Box::new(Expression::parse("NAND(BIT, BIT)").unwrap()),
                })),
            }),
        );
        assert_eq!(
            Ok("pub const @\"one\" = true;\npub fn @\"run\"() bool {\n    var @\"state\": bool = true;\n    const @\"alias\": *bool = &@\"state\";\n    @\"alias\".* = !(@\"state\" and @\"state\");\n    return @\"state\";\n}\n".to_string()),
            emitted(&program)
        );
    }

    #[test]
    pub fn test_program_emit_main() {
        assert_eq!(
            Ok("pub fn main(init: std.process.Init.Minimal) void {\n    @\"$lapzt\".initialize(init);\n    return;\n}\n".to_string()),
            emitted(&Program::parse("main = () [] { [] }").unwrap())
        );
        assert_eq!(
            Ok("pub const @\"ONE\": bool = true;\npub fn @\"bit.not\"(@\"a\": bool) bool {\n    return !(@\"a\" and @\"a\");\n}\npub fn main(init: std.process.Init.Minimal) void {\n    @\"$lapzt\".initialize(init);\n    _ = @\"bit.not\"(true);\n    return;\n}\n".to_string()),
            emitted(&with_one("bit.not = (a: BIT) BIT { NAND(a, a) }\nmain = () [] {\n bit.not(ONE)\n []\n}"))
        );
    }

    #[test]
    pub fn test_liblapzt() {
        let program = Program::parse(&std()).unwrap();
        assert_eq!(Vec::<ProgramError>::new(), program.errors());
    }

    #[test]
    pub fn test_liblapzt_extensions() {
        let source = format!(
            "{}\n{}",
            std(),
            "main = () [] {\n    a: liblapzt.f64 = liblapzt.f64.from.u64(liblapzt.constants.one)\n    b: liblapzt.f32 = liblapzt.f64.to.f32(a)\n    c: liblapzt.f64 = liblapzt.f32.to.f64(b)\n    liblapzt.print.f64(c)\n    liblapzt.print.newline()\n    o: liblapzt.ordering = liblapzt.compare.f64(a, c)\n    liblapzt.print.bool(liblapzt.ordering.is.equal(o))\n    liblapzt.print.newline()\n    []\n}\n"
        );
        let program = Program::parse(&source).unwrap();
        assert_eq!(Vec::<ProgramError>::new(), program.errors());
    }

    #[test]
    pub fn test_program_emit_indeterminate() {
        assert_eq!(
            Err(vec![ProgramError::Indeterminate(
                "NAND(BIT, BIT)".to_string()
            )]),
            Program::parse("a = NAND(BIT, BIT)").unwrap().emit()
        );
        assert_eq!(
            Err(vec![ProgramError::Indeterminate("BIT".to_string())]),
            Program::parse("a: BIT = BIT").unwrap().emit()
        );
    }
}
