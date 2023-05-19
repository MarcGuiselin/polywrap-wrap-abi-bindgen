lazy_static! {
  static ref NAME: String = "enum_type/index.ts".to_string();
  static ref SOURCE: String = r#"export enum {{detect_keyword type}} {
  {{#each constants}}
  {{detect_keyword .}},
  {{/each}}
  _MAX_
}

export function sanitize{{type}}Value(value: i32): void {
  const valid = value >= 0 && value < {{detect_keyword type}}._MAX_;
  if (!valid) {
    throw new Error("Invalid value for enum '{{detect_keyword type}}': " + value.toString());
  }
}

export function get{{type}}Value(key: string): {{detect_keyword type}} {
  {{#each constants}}
  if (key == "{{detect_keyword .}}") {
    return {{detect_keyword type}}.{{detect_keyword .}};
  }
  {{/each}}

  throw new Error("Invalid key for enum '{{detect_keyword type}}': " + key);
}

export function get{{type}}Key(value: {{detect_keyword type}}): string {
  sanitize{{type}}Value(value);

  switch (value) {
    {{#each constants}}
    case {{detect_keyword type}}.{{detect_keyword .}}: return "{{detect_keyword .}}";
    {{/each}}
    default:
      throw new Error("Invalid value for enum '{{detect_keyword type}}': " + value.toString());
  }
}
"#.to_string();
}

use super::super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}