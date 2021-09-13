use data::{
	ast::Value,
	semantics::{Group, Semantics},
};

impl Value {
	fn as_string(&self) -> String {
		match self {
			Value::String(value) => value.clone(),
			Value::Number(_) => panic!("tried to get string value of number"),
			Value::Variable(variable) => panic!(
				"tried to get string value of unrendered variable {}",
				variable.0.to_string()
			),
		}
	}

	// pub fn as_integer(self) -> i32 {
	// 	match self {
	// 		Value::Number(value) => value,
	// 		Value::String(_) => panic!("tried to get numeric value of string"),
	// 		Value::Variable(_) => panic!("tried to get numeric value of unrendered variable"),
	// 	}
	// }
}

impl Group {
	pub fn get_string(&self, value: Value) -> String {
		match value {
			Value::String(value) => value.clone(),
			Value::Number(_) => panic!("tried to get string value of number"),
			Value::Variable(name) => self.variables[&name.0.to_string()].as_string(),
		}
	}
}

impl Semantics {
	pub fn render_value(&self, value: Value, ancestors: &Vec<usize>) -> Value {
		match value {
			Value::Variable(variable) => {
				for ancestor_id in ancestors.clone() {
					log::debug!(" Looking at ancestor: {}", ancestor_id);

					if let Some(value) = self.groups[ancestor_id]
						.variables
						.get(&variable.0.to_string())
					{
						return value.clone();
					}
				}
				panic!("unable to evaluate variable")
			}
			value => value.clone(),
		}
	}
}
