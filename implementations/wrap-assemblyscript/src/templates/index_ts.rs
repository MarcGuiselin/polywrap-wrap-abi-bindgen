lazy_static! {
  static ref NAME: String = "index.ts".to_string();
  static ref SOURCE: String = r#"{{#moduleType}}
{{#methods.length}}
import {
  {{#methods}}
  Args_{{#detectKeyword}}{{name}}{{/detectKeyword}}{{^last}},{{/last}}
  {{/methods}}
} from "./{{type}}";
{{/methods.length}}
{{/moduleType}}
{{#moduleType}}
{{#methods.length}}
export {
  {{#methods}}
  Args_{{#detectKeyword}}{{name}}{{/detectKeyword}}{{^last}},{{/last}}
  {{/methods}}
};
{{/methods.length}}
{{/moduleType}}
{{#moduleType}}
export { ModuleBase } from "./Module";
{{/moduleType}}
{{#objectTypes}}
export { {{#detectKeyword}}{{type}}{{/detectKeyword}} } from "./{{type}}";
{{/objectTypes}}
{{#enumTypes}}
export {
  {{#detectKeyword}}{{type}}{{/detectKeyword}},
  get{{type}}Key,
  get{{type}}Value,
  sanitize{{type}}Value
} from "./{{type}}";
{{/enumTypes}}
{{#importedModuleTypes}}
export { {{#detectKeyword}}{{type}}{{/detectKeyword}} } from "./imported/{{type}}";
{{/importedModuleTypes}}
{{#importedObjectTypes}}
export { {{#detectKeyword}}{{type}}{{/detectKeyword}} } from "./imported/{{type}}";
{{/importedObjectTypes}}
{{#importedEnvTypes}}
export { {{#detectKeyword}}{{type}}{{/detectKeyword}} } from "./imported/{{type}}";
{{/importedEnvTypes}}
{{#importedEnumTypes}}
export {
  {{#detectKeyword}}{{type}}{{/detectKeyword}},
  get{{type}}Key,
  get{{type}}Value,
  sanitize{{type}}Value
} from "./imported/{{type}}";
{{/importedEnumTypes}}
{{#interfaceTypes}}
export { {{#detectKeyword}}{{namespace}}{{/detectKeyword}} } from "./{{namespace}}";
{{/interfaceTypes}}
{{#envType}}
export { Env } from "./Env";
{{/envType}}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
