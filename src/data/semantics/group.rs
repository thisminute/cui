use {
	data::{ast::Value, semantics::properties::Properties},
	std::collections::HashMap,
};

#[derive(Clone)]
pub struct Group {
	pub name: Option<String>,
	pub selector: Option<String>,
	pub class_names: Vec<String>,
	pub listener_scope: Option<usize>,
	pub variables: HashMap<String, Value>,

	pub properties: Properties,
	pub elements: Vec<usize>,
	pub classes: HashMap<String, Vec<usize>>,
	pub listeners: Vec<usize>,

	pub members: Vec<usize>,
	pub member_of: Vec<usize>,

	pub tag: &'static str,
}
impl Group {
	pub fn new(
		name: Option<String>,
		listener_scope: Option<usize>,
		variables: HashMap<String, Value>,
	) -> Self {
		Self {
			name,
			selector: None,
			class_names: Vec::new(),
			listener_scope,
			variables,

			properties: Properties::default(),
			elements: Vec::new(),
			classes: HashMap::new(),
			listeners: Vec::new(),

			members: Vec::new(),
			member_of: Vec::new(),

			tag: "div",
		}
	}

	pub fn class_to_new_static_element(&mut self, source_id: usize) -> Self {
		Group {
			name: Some(
				self
					.name
					.clone()
					.expect("should never try to make an instance of a class with no name"),
			),
			selector: None,
			class_names: Vec::new(),
			listener_scope: None,
			variables: HashMap::new(),

			properties: Properties {
				cui: self.properties.cui.clone(),
				css: HashMap::new(),
				page: HashMap::new(),
			},
			elements: self.elements.clone(),
			classes: HashMap::new(),
			listeners: self.listeners.clone(),

			members: Vec::new(),
			member_of: vec![source_id],

			tag: "div",
		}
	}

	pub fn is_static(&self) -> bool {
		self.listener_scope.is_none()
	}
}
