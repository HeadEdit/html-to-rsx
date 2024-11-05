use html_parser::{Dom, Element, Node};

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

pub fn parse(html: &str) -> Result<String, Error> {
    let dom = Dom::parse(html).map_err(|e| Error::ParseError(format!("{e}")))?;
    let mut res = vec![];
    for c in dom.children {
        res.push(node_parser(c,0))
    }

    Ok(res.join(""))
}
fn node_parser(node: Node,layer:u32) -> String {
    let mut indents = String::new();
    for _ in 0..layer+1{
        indents.push('\t');
    }
    match node {
        html_parser::Node::Text(t) => format!("{indents}{t:?}"),
        html_parser::Node::Comment(_) => "".to_string(),
        html_parser::Node::Element(e) => element_parser(e,layer),
    }
}

fn element_parser(mut element: Element,layer:u32) -> String {
    let mut indents = String::new();
    let mut layer = layer+1;
    for _ in 0..layer{
        indents.push('\t');
    }

    let mut out = format!("{indents}{} {{ \n",element.name);
    indents.push('\t');
    if let Some(id) = element.id {
        let res = format!("{indents}id: \"{id}\", \n");
        out.push_str(&res);
    }

    for (atbr, val) in &element.attributes {
        if(atbr=="viewBox"){
            continue;
        }
        let atbr_name = atbr.replace("-", "_").replace("for", "r#for").replace("type", "r#type");
        let res = if let Some(v) = val {
            format!("{atbr_name}: \"{v}\",")
        } else {
            format!("{atbr_name},")
        };
        out.push_str(&format!("{indents}{}\n",&res));
    }

    if !element.classes.is_empty() {
        let res = format!("class: \"{}\",", element.classes.join(" "));
        out.push_str(&format!("{indents}{}\n",&res));
    }
    
    let res = element
        .children
        .into_iter()
        .map(|node|node_parser(node,layer))
        .collect::<Vec<_>>()
        .join(" ");
    out.push_str(&format!("{}",&res));
    indents.remove(0);
    out.push_str(&format!("{}}}\n",indents));
    out
}
