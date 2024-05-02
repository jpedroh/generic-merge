mod tweak_import_declarations;

use crate::ParsingHandlers;

pub fn get_default_java_parsing_handlers() -> ParsingHandlers {
    ParsingHandlers::new(vec![tweak_import_declarations::tweak_import_declarations])
}
