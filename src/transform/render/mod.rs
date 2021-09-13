mod cascade;
mod element;
mod value;

use data::{
	ast::Value,
	semantics::{properties::PageProperty, Semantics},
};

impl Semantics {
	pub fn render(&mut self) {
		log::debug!("...Rendering...");
		for i in 0..self.pages.len() {
			let page_group_id = self.pages[i].root_id;
			log::debug!("Rendering page {}", page_group_id);
			// TODO: routes based on directory structure
			let ancestors = &mut Vec::new();
			let default = &Value::String("".into());
			let title = self.groups[page_group_id]
				.properties
				.page
				.get(&PageProperty::Title)
				.or(Some(default))
				.unwrap()
				.clone();
			self.pages[i].title = self.groups[page_group_id].get_string(title);
			self.render_element(page_group_id, ancestors);
		}
	}
}
