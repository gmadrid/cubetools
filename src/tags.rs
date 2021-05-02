// TODO: can we make these &str?
#[derive(Clone, Debug)]
pub struct Tag {
    name: String,
    attrs: Vec<(String, String)>,
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: Default::default(),
        }
    }

    pub fn attr(mut self, attr: &str, value: &str) -> Self {
        self.attrs.push((attr.to_string(), value.to_string()));
        self
    }

    pub fn open(&self) -> String {
        let attr_string = self
            .attrs
            .iter()
            .map(|(a, v)| format!("   {}=\"{}\"", a, v))
            .collect::<Vec<_>>()
            .join("\n");

        format!("<{}\n{}>\n", self.name, attr_string)
    }

    pub fn close(&self) -> String {
        format!("</{}>\n", self.name)
    }
}
