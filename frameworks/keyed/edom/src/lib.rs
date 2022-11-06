use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
mod edom;
use edom::{ElementIterator, ElementNode};
use rand::prelude::*;

const ADJECTIVES: &'static [&'static str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy"];
const COLOURS: &'static [&'static str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black", "orange"];
const NOUNS: &'static [&'static str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard"];
fn next_line(n: &mut u32, thread_rng: &mut ThreadRng)->(u32, String) {
    let mut s = ADJECTIVES.choose(thread_rng).unwrap().to_string();
    s.push_str(" ");
    s.push_str(COLOURS.choose(thread_rng).unwrap());
    s.push_str(" ");
    s.push_str(NOUNS.choose(thread_rng).unwrap());
    let r=(*n, s);
    *n+=1;
    r
}

fn custom_button_clicked<EN>(container: &mut ElementIterator<EN>, text: &str, id: &str)->bool where EN: ElementNode {
    let divclass="col-sm-6 smallpad";
    let btnclass="btn btn-primary btn-block";
    let mut r=false;
    container.div(|d| {
        d.class(divclass);
        if d.button(text).id(id).class(btnclass).clicked() {
            r=true;
        }
    });
    return r;
}

fn add_jumbotron<EN>(container: &mut ElementIterator<EN>, v:&mut Vec<(u32, String)>,
        n: &mut u32, thread_rng: &mut ThreadRng) where EN: ElementNode {
    container.div(|jumbotron| {
        jumbotron.class("jumbotron");
        let btnclass="btn btn-primary btn-block";
        let divclass="col-sm-6 smallpad";
        jumbotron.div(|r|{
            r.class("row");
            r.div(|c1| {
                c1.class("col-md-6");
                c1.h1().text("edom-\"keyed\"")
            });
            r.div(|c2| {
                c2.class("col-md-6");
                c2.div(|c2r| {
                    c2r.class("row");
                    if custom_button_clicked(c2r, "Create 1,000 rows", "run") {
                        v.clear();
                        for _ in 0..1000 {
                            v.push(next_line(n, thread_rng));
                        }
                    }
                    if custom_button_clicked(c2r, "Create 10,000 rows", "runlots") {
                        v.clear();
                        for _ in 0..10000 {
                            v.push(next_line(n, thread_rng));
                        }
                    }
                    if custom_button_clicked(c2r, "Append 1,000 rows", "add") {
                        for _ in 0..10000 {
                            v.push(next_line(n, thread_rng));
                        }
                    }
                    if custom_button_clicked(c2r, "Update every 10th row", "update")  {
                        for i in (0..v.len()).step_by(10) {
                            v[i].1.push_str(" !!!")
                        }
                    }
                    if custom_button_clicked(c2r, "Clear", "clear")  {
                        v.clear();
                    };
                    if custom_button_clicked(c2r, "Swap rows", "swaprows")  {
                        let vlen=v.len();
                        if vlen > 2 {
                            v.swap(1, vlen-2)
                        }
                    }
                    if custom_button_clicked(c2r, "Create 30,000 rows", "run30000")  {
                        v.clear();
                        for _ in 0..30000 {
                            v.push(next_line(n, thread_rng));
                        }
                    }
                });
            });
        });
    });
}

fn add_table<EN>(mut container: &mut ElementIterator<EN>, v:&mut Vec<(u32, String)>, selected: &mut Option<u32>) where EN: ElementNode {
    web_sys::console::log_2(&"add_table, v.len()=".into(), &v.len().into());
    let mut table=container.element("table");
    table.class("table table-hover table-striped test-data");
    let mut tbody=table.element("tbody");
    tbody.id("tbody");
    let mut vremove=None;
    tbody.for_each(v.iter(), |elem| elem.0 as u64, "tr", |elem, row| {
        row.class(if Some(elem.0)==*selected {"danger"} else {""});
        row.element("td").class("col-md-1").text(elem.0.to_string().as_str());
        row.element("td").class("col-md-4")
            .element("a").class("lbl")
                .click(|| *selected=Some(elem.0)).text(elem.1.as_str());
        row.element("td").class("col-md-1")
            .element("a").class("remove")
                .element("span").class("remove glyphicon glyphicon-remove").attr("aria-hidden", "true")
                    .click(|| vremove=Some(elem.0));
        row.element("td").class("col-md-6");
    });
    if let Some(vr)=vremove {
        v.retain(|elem| elem.0 != vr)
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let mut v:Vec<(u32,String)>=Vec::new();
    let mut n=1;
    let mut selected : Option<u32>=None;
    let mut thread_rng = thread_rng();
    edom::EDOM::render("body", web_sys::Element::from(body), move |mut root| {
        root.div(|main|{
            main.id("main");
            main.div(|container| {
                container.class("container");
                add_jumbotron(container, &mut v, &mut n, &mut thread_rng);
                add_table(container, &mut v, &mut selected);
            });
        });
    });
    Ok(())
}
