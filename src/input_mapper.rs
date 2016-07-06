extern crate piston;

use piston::input::{Input};
use std::collections::HashMap;

struct InputMapper {
	mappings : mut HashMap<Input::Button, (GameAction, GameAction)>
}

impl InputMapper {
	fn new() -> InputMapper {
		return InputMapper {
			mappings : HashMap::new(),
		}
	}

	fn add_mapping(&mut self, key: Input::Button, on_press: GameAction, on_release: GameAction) {
		mappings.put(key, (on_press, on_release));
	}

	fn get_mapping(&mut self, input_type: Input)
}