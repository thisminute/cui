use {
	data::semantics::{properties::CuiProperty, Group, Semantics},
	std::collections::HashMap,
	transform::compile::css::Css,
};

impl Semantics {
	pub fn html(&self) -> (String, HashMap<String, String>) {
		log::debug!("...Writing HTML...");
		let (contents, styles) = self.html_parts();
		let homepage = contents.get("/").unwrap();
		let root = &self.pages[self.pages[0].root_id];
		(
			format!(
				"<html>{}{}</html>",
				format!("<head>{}{}</head>", 
					format!("<title>{}</title>", root.title),
					format!("<style>{}</style>", styles)
				),
				format!(
					"<body>{}{}{}</body>",
					homepage,
					"<noscript>This page contains Webassembly and Javascript content. Please make sure that you are using the latest version of a modern browser and that Javascript and Webassembly (Wasm) are enabled.</noscript>",
					"<script src='./bootstrap.js'></script>"
				)
			),
			contents
		)
	}

	pub fn html_parts(&self) -> (HashMap<String, String>, String) {
		let contents = self
			.pages
			.iter()
			.map(|page| {
				(
					page.route.clone(),
					self.groups[page.root_id].html(&self.groups),
				)
			})
			.collect::<HashMap<String, String>>();
		(contents, self.styles.css())
	}
}

impl Group {
	fn html(&self, groups: &Vec<Group>) -> String {
		let link = match self.properties.cui.get(&CuiProperty("link".to_string())) {
			Some(value) => self.get_string(value.clone()),
			None => "".to_string(),
		};
		let attributes = [
			("style", &*self.properties.css.css()),
			("class", &*self.class_names.join(" ")),
			("href", &*link),
		]
		.iter()
		.filter(|(_, value)| !value.is_empty())
		.map(|(attribute, value)| format!(" {}='{}'", attribute, value))
		.collect::<Vec<String>>()
		.join("");

		let children = self
			.elements
			.iter()
			.filter(|&&element_id| groups[element_id].is_static())
			.map(|&child_id| groups[child_id].html(groups))
			.collect::<Vec<String>>()
			.join("");

		let contents = format!(
			"{}{}",
			match self.properties.cui.get(&CuiProperty("text".to_string())) {
				Some(value) => self.get_string(value.clone()),
				None => "".to_string(),
			},
			children
		);

		format!("<{0}{1}>{2}</{0}>", self.tag, attributes, contents)
	}
}
