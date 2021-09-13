use {
	data::semantics::{properties::CuiProperty, Semantics},
	misc::id_gen::id_gen,
};

impl Semantics {
	pub fn render_element(&mut self, element_id: usize, ancestors: &mut Vec<usize>) {
		log::debug!("Rendering element {}", element_id);

		// TODO: verify that this precaution does anything
		// cascading can add more listeners, so this should ensure that the loop also iterates over those
		let mut last_idx = 0;
		while last_idx < self.groups[element_id].listeners.len() {
			let listener_id = self.groups[element_id].listeners[last_idx];
			log::debug!(
				" Attaching listener {} to element {}",
				listener_id,
				element_id,
			);
			self.cascade(listener_id, element_id, true);
			last_idx += 1;
		}

		for ancestor_id in ancestors.clone() {
			log::debug!(" Looking at ancestor: {}", ancestor_id);
			for class_id in self.groups[ancestor_id]
				.classes
				.get(
					&self.groups[element_id]
						.name
						.clone()
						.expect("element to have a name"),
				)
				.unwrap_or(&Vec::new())
				.clone()
			{
				log::debug!("  Attaching class {} to element {}", class_id, element_id);
				self.groups[class_id].members.push(element_id);
				self.groups[element_id].member_of.push(class_id);
			}
		}

		for source_id in self.groups[element_id].member_of.clone() {
			// TODO: not having this causes extra classes and would be good to be able to uncomment without breaking tests
			// if self.groups[element_id].is_static() && self.groups[source_id].properties.css.is_empty() {
			// 	continue;
			// }

			let selector = self.groups[source_id]
				.selector
				.get_or_insert_with(id_gen)
				.clone();
			log::debug!("  Generated selector {} for group {}", selector, source_id);
			self.cascade(
				source_id,
				element_id,
				self.groups[element_id].listener_scope != self.groups[source_id].listener_scope,
			);
			if !self.groups[source_id].properties.css.is_empty() {
				self.styles.insert(
					format!(".{}", selector),
					self.groups[source_id].properties.css.clone(),
				);
			}
			self.groups[element_id].class_names.push(selector);
		}

		self.groups[element_id].variables = self.groups[element_id]
			.variables
			.clone()
			.into_iter()
			.map(|(identifier, value)| (identifier, self.render_value(value, ancestors)))
			.collect();

		ancestors.push(element_id);
		for element_id in self.groups[element_id].elements.clone() {
			self.render_element(element_id, ancestors);
		}
		ancestors.pop();

		self.groups[element_id].tag = match self.groups[element_id]
			.properties
			.cui
			.get(&CuiProperty("link".to_string()))
		{
			Some(_) => "a",
			None => "div",
		};

		log::debug!(" Removing virtual groups from element {}", element_id);
		let listener_scope = self.groups[element_id].listener_scope;
		self.groups[element_id].elements = self.groups[element_id]
			.elements
			.clone()
			.into_iter()
			.filter(|&group_id| listener_scope == self.groups[group_id].listener_scope)
			.collect();
		let mut classes = self.groups[element_id].classes.clone();
		for (_, groups) in &mut classes {
			groups.retain(|&group_id| listener_scope == self.groups[group_id].listener_scope)
		}
		self.groups[element_id].classes = classes;
		self.groups[element_id].listeners = self.groups[element_id]
			.listeners
			.clone()
			.into_iter()
			.filter(|&group_id| listener_scope == self.groups[group_id].listener_scope)
			.collect();
	}
}
