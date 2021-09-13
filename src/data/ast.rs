use {
	proc_macro2::TokenStream,
	quote::ToTokens,
	std::{collections::HashMap, fmt},
	syn::Ident,
};

pub struct Document {
	pub root: Block,
}

#[derive(Debug)]
pub enum Prefix {
	Element,
	Class,
	Listener,
}

pub struct Block {
	pub prefix: Prefix,
	pub identifier: Ident,
	pub properties: Vec<Property>,
	pub elements: Vec<Block>,
	pub classes: Vec<Block>,
	pub listeners: Vec<Block>,
	pub variables: HashMap<String, Value>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Variable(pub Ident);

#[derive(Clone, Debug)]
pub enum Value {
	Number(i32),
	String(String),
	Variable(Variable),
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Value::Number(value) => value.to_string(),
				Value::String(value) => value.clone(),
				Value::Variable(_) => "@variable".to_string(),
			}
		)
	}
}

impl ToTokens for Value {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.to_string().to_tokens(tokens)
	}
}

pub struct Property {
	pub property: Ident,
	pub value: Value,
}

pub struct Assignment {
	pub variable: Variable,
	pub value: Value,
}
