use data::semantics::Semantics;

impl Semantics {
	fn create_element_from_group(&mut self, source_id: usize, parent_id: usize) {
		let element_id = self.groups.len();
		log::debug!(" Creating new element group {}", element_id);
		let source = &mut self.groups[source_id];
		let element = source.class_to_new_static_element(source_id);
		source.members.push(element_id);
		self.groups.push(element);
		self.groups[parent_id].elements.push(element_id);
	}

	pub fn cascade(&mut self, source_id: usize, target_id: usize, virtual_: bool) {
		log::debug!(
			"Cascading from group {} into group {}",
			source_id,
			target_id
		);
		if source_id == target_id {
			panic!("the build process should never try to cascade a group into itself")
		}

		if !virtual_ {
			for (property, value) in self.groups[source_id].properties.cui.clone() {
				log::debug!(" Cascading cui property {}:{:?}", property.0, value);
				self.groups[target_id]
					.properties
					.cui
					.entry(property)
					.or_insert(value.clone());
			}

			for (name, value) in self.groups[source_id].variables.clone() {
				log::debug!(" Cascading variable {}:{:?}", name, value);
				self.groups[target_id].variables.insert(name, value);
			}
		}

		for listener_id in self.groups[source_id].listeners.clone() {
			log::debug!(" Cascading scoped listener {}", listener_id);
			self.groups[target_id].listeners.push(listener_id);
			self.cascade(listener_id, target_id, true);
		}

		for (name, class_ids) in self.groups[source_id].classes.clone() {
			for class_id in class_ids {
				log::debug!(" Cascading scoped class {} with name {}", class_id, name);
				self.groups[target_id]
					.classes
					.entry(name.clone())
					.or_default()
					.push(class_id);
			}
		}

		if (self.groups[source_id].listener_scope == self.groups[target_id].listener_scope)
			&& self.groups[source_id].elements.len() > 0
			&& self.groups[target_id].elements.len() > 0
		{
			panic!("Source and target group specify different contents for the same element")
		}

		for source_id in self.groups[source_id].elements.clone() {
			log::debug!(
				" Cascading element with name {}",
				self.groups[source_id].name.clone().unwrap()
			);
			if self.groups[source_id].is_static() {
				self.create_element_from_group(source_id, target_id);
			} else {
				self.groups[target_id].elements.push(source_id);
			}
		}
	}
}
