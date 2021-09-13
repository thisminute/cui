mod peek;

use {
	self::peek::Peek,
	data::ast::{Assignment, Block, Document, Prefix, Property, Value, Variable},
	proc_macro2::Span,
	std::collections::HashMap,
	syn::{
		braced,
		parse::{Parse, ParseStream},
		Ident, LitInt, LitStr, Token,
	},
};

impl Parse for Document {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		Ok(Self {
			root: parse_content(input, Ident::new("_", Span::call_site()), Prefix::Element)?,
		})
	}
}

fn parse_content(
	input: ParseStream,
	identifier: Ident,
	prefix: Prefix,
) -> Result<Block, syn::Error> {
	let mut block = Block {
		identifier,
		prefix,
		properties: Vec::new(),
		elements: Vec::new(),
		classes: Vec::new(),
		listeners: Vec::new(),
		variables: HashMap::new(),
	};
	loop {
		if input.peek_property() {
			block.properties.push(input.parse()?);
		} else if input.peek_element_block() {
			block.elements.push(input.parse()?);
		} else if input.peek_class_block() {
			block.classes.push(input.parse()?);
		} else if input.peek_listener_block() {
			block.listeners.push(input.parse()?);
		} else if input.peek_assignment() {
			let assignment = input.parse::<Assignment>()?;
			block
				.variables
				.insert(assignment.variable.0.to_string(), assignment.value);
		} else {
			break;
		}
	}
	Ok(block)
}

// impl Parse for IdentVec {
// 	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
// 		let mut list = Vec::new();
// 		list.push(input.parse()?);
// 		while input.peek(Token![,]) {
// 			input.parse::<Token![,]>()?;
// 			list.push(input.parse()?);
// 		}
// 		Ok(IdentVec(list))
// 	}
// }

impl Parse for Block {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let prefix = if input.peek(Token![.]) {
			input.parse::<Token![.]>()?;
			Prefix::Class
		} else if input.peek(Token![?]) {
			input.parse::<Token![?]>()?;
			Prefix::Listener
		} else {
			Prefix::Element
		};

		let identifier = input.parse()?;

		let content;
		braced!(content in input);

		parse_content(&content, identifier, prefix)
	}
}

impl Parse for Property {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let property = input.parse()?;
		input.parse::<Token![:]>()?;
		let value = input.parse()?;
		input.parse::<Token![;]>()?;
		Ok(Self { property, value })
	}
}

impl Parse for Assignment {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let variable = input.parse()?;
		input.parse::<Token![:]>()?;
		let value = input.parse()?;
		input.parse::<Token![;]>()?;
		Ok(Self { variable, value })
	}
}

impl Parse for Value {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		Ok(if input.peek_variable() {
			Self::Variable(input.parse::<Variable>()?)
		} else if input.peek(LitStr) {
			Self::String(input.parse::<LitStr>()?.value())
		} else {
			Self::Number(input.parse::<LitInt>()?.base10_parse::<i32>()?)
		})
	}
}

impl Parse for Variable {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		input.parse::<Token![$]>()?;
		Ok(Self(input.parse::<Ident>()?))
	}
}
