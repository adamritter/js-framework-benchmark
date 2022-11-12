#![allow(warnings)]
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
pub mod edom;
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
pub fn next_line(n: &mut u32, thread_rng: &mut ThreadRng)->(u32, String) {
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
                        for _ in 0..1000 {
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
                .click(|| *selected=Some(elem.0))
                .text(elem.1.as_str());
        row.element("td").class("col-md-1")
             .element("a").class("remove")
                .element("span").class("remove glyphicon glyphicon-remove").attr("aria-hidden", "true")
                    .click(|| vremove=Some(elem.0))
                    ;
        row.element("td").class("col-md-6");
    });
    if let Some(vr)=vremove {
        v.retain(|elem| elem.0 != vr)
    }
}

pub fn js_framework_benchmark<EN>(mut root: &mut ElementIterator<EN>, v:&mut Vec<(u32, String)>,  n: &mut u32, thread_rng: &mut ThreadRng,
        selected: &mut Option<u32>) where EN: ElementNode {
    root.div(|main|{
        main.id("main");
        main.div(|container| {
            container.class("container");
            add_jumbotron(container, v, n, thread_rng);
            add_table(container, v, selected);
        });
    });
}


// #[wasm_bindgen(start)] // 600ms
pub fn bench_create_100k_vec() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_create_100k in vector".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let mut v=Vec::new();
    for _ in 0..1000000 {
        v.push(document.create_element(&"div"));
    }
    web_sys::console::time_end();
    Ok(())
} 


// #[wasm_bindgen(start)] // 60ms
pub fn bench_create_100k_simple() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_create_100k in simple".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    for _ in 0..1000000 {
        document.create_element(&"div");
    }
    web_sys::console::time_end();
    Ok(())
} 


// #[wasm_bindgen(start)] // 600ms
pub fn bench_create_100k_remove() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_create_100k remove".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    for _ in 0..1000000 {
        let e=document.create_element(&"div").unwrap();
        e.remove();
    }
    web_sys::console::time_end();
    Ok(())
} 


// #[wasm_bindgen(start)] // 800ms / 100k
pub fn bench_create_10k_top_down() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_create_10k_top_down".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut last=document.create_element(&"div").unwrap();
    for _ in 0..10000 {
        let new=document.create_element(&"div").unwrap();
        last.append_child(&new);
        last=new;
    }
    web_sys::console::time_end();
    Ok(())
} 

// #[wasm_bindgen(start)] // 800ms
pub fn bench_create_10k_bottom_up() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_create_10k_bottom_up".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut last=document.create_element(&"div").unwrap();
    for _ in 0..10000 {
        let new=document.create_element(&"div").unwrap();
        last.append_child(&new);
        last=new;
    }
    web_sys::console::time_end();
    Ok(())
} 


// #[wasm_bindgen(start)] // 10ms
pub fn bench_add10k() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_add10k".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    for _ in 0..10000 {
        let new=document.create_element(&"div").unwrap();
        main.append_child(&new);
    }
    web_sys::console::time_end();
    Ok(())
} 


// #[wasm_bindgen(start)] // 180ms (10x20)
pub fn bench_add20x10k() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_add20x10k".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    for _ in 0..10000 {
        let new=document.create_element(&"div").unwrap();
        for _ in 0..20 {
            new.append_child(&document.create_element(&"div").unwrap());
        }
        main.append_child(&new);
    }
    web_sys::console::time_end();
    Ok(())
} 


// #[wasm_bindgen(start)] // 190ms (10x20)
pub fn bench_add20x10k_after() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_add20x10k_after".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    for _ in 0..10000 {
        let new=document.create_element(&"div").unwrap();
        main.append_child(&new);
        for _ in 0..20 {
            new.append_child(&document.create_element(&"div").unwrap());
        }
    }
    web_sys::console::time_end();
    Ok(())
} 


// #[wasm_bindgen(start)] // 38ms (10x20)
pub fn bench_clone20x10k_after() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_clone20x10k_after".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    let new=document.create_element(&"div").unwrap();
    for _ in 0..20 {
        new.append_child(&document.create_element(&"div").unwrap());
    }
    for _ in 0..10000 {
        main.append_child(&new.deep_clone());
    }
    web_sys::console::time_end();
    Ok(())
} 


// #[wasm_bindgen(start)] // 72ms (10x20)
pub fn bench_clone20x10k_after_get_10_random_children() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_clone20x10k_after_get_10_random_children".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    // document.body().unwrap().append_child(&main);
    let new=document.create_element(&"div").unwrap();
    for _ in 0..20 {
        new.append_child(&document.create_element(&"div").unwrap());
    }
    let mut v=Vec::new();
    for _ in 0..10000 {
        let clone=new.deep_clone();
        for i in 0..10 {
            v.push(clone.get_child_node(i*2));
        }
        main.append_child(&clone);
    }
    web_sys::console::time_end();
    Ok(())
} 


// #[wasm_bindgen(start)] // 60ms (10x20)
pub fn bench_clone20x10k_after_get_first_10_children() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_clone20x10k_after_get_first_10_children".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    // document.body().unwrap().append_child(&main);
    let new=document.create_element(&"div").unwrap();
    for _ in 0..20 {
        new.append_child(&document.create_element(&"div").unwrap());
    }
    for _ in 0..10000 {
        let clone=new.deep_clone();
        let mut child=clone.first_child().unwrap();
        for i in 0..9 {
            child=child.next_sibling().unwrap();
        }
        main.append_child(&clone);
    }
    web_sys::console::time_end();
    Ok(())
}


// #[wasm_bindgen(start)] // 73ms (10x20)
pub fn bench_clone20x10k_after_get_children_next_sibling() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_clone20x10k_after_get_children_next_sibling".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    // document.body().unwrap().append_child(&main);
    let new=document.create_element(&"div").unwrap();
    for _ in 0..20 {
        new.append_child(&document.create_element(&"div").unwrap());
    }
    for _ in 0..10000 {
        let clone=new.deep_clone();
        let mut child=clone.first_child().unwrap();
        for i in 0..19 {
            child=child.next_sibling().unwrap();
        }
        main.append_child(&clone);
    }
    web_sys::console::time_end();
    Ok(())
}

// #[wasm_bindgen(start)] // 75ms (10x20)
pub fn bench_clone20x10k_after_get_children_child_nodes() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_clone20x10k_after_get_children_child_nodes".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    // document.body().unwrap().append_child(&main);
    let new=document.create_element(&"div").unwrap();
    for _ in 0..20 {
        new.append_child(&document.create_element(&"div").unwrap());
    }
    let mut v=Vec::new();
    for _ in 0..10000 {
        let clone=new.deep_clone();
        let mut children=clone.get_child_nodes();
        for child in children {
            v.push(child);
        }
        main.append_child(&clone);
    }
    web_sys::console::time_end();
    Ok(())
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// #[wasm_bindgen(start)] // 650ms
pub fn bench_20k_listeners() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_20k_listeners".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    // document.body().unwrap().append_child(&main);
    let new=document.create_element(&"div").unwrap();
    for _ in 0..20000 {
        let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(move |e: web_sys::Event| {
            web_sys::console::log_2(&13.to_string().into(), &"click".into());
            e.prevent_default();
        })  as Box<dyn FnMut(_)>);
        new.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
    web_sys::console::time_end();
    Ok(())
}


// #[wasm_bindgen(start)] // 16ms
pub fn bench_20k_listeners_same_closure() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_20k_listeners_same_closure".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    // document.body().unwrap().append_child(&main);
    let new=document.create_element(&"div").unwrap();
    let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(move |e: web_sys::Event| {
        web_sys::console::log_2(&13.to_string().into(), &"click".into());
        e.prevent_default();
    })  as Box<dyn FnMut(_)>);
    for _ in 0..20000 {
        new.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
    }
    closure.forget();
    web_sys::console::time_end();
    Ok(())
}


// #[wasm_bindgen(start)] // 43ms
pub fn bench_20k_listeners_same_closure_bind1() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_20k_listeners_same_closure_bind1".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    // document.body().unwrap().append_child(&main);
    let new=document.create_element(&"div").unwrap();
    let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(move |uid: u64, e: web_sys::Event| {
        web_sys::console::log_2(&13.to_string().into(), &"click".into());
        e.prevent_default();
    })  as  Box<dyn FnMut(_,_)->()>);
    let f:&js_sys::Function=closure.as_ref().unchecked_ref();
    for _ in 0..20000 {
        new.add_event_listener_with_callback("click", &f.bind1(&JsValue::undefined(), &"59".into()));
    }
    closure.forget();
    web_sys::console::time_end();
    Ok(())
}

// #[wasm_bindgen(start)] // 16ms
pub fn bench_20k_listeners_data_uid() -> Result<(), JsValue> {
    web_sys::console::time();
    web_sys::console::log_1(&"bench_20k_listeners_same_closure".into());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut main=document.create_element(&"div").unwrap();
    // document.body().unwrap().append_child(&main);
    let new=document.create_element(&"div").unwrap();
    let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(move |e: web_sys::Event| {
        web_sys::console::log_2(&13.to_string().into(), &"click".into());
        e.prevent_default();
    })  as Box<dyn FnMut(_)>);
    for i in 0..20000 {
        new.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        new.set_attribute("data-uid", i.to_string().as_str());
    }
    closure.forget();
    web_sys::console::time_end();
    Ok(())
}
use js_sys::Function;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();


    let closure = Closure::wrap(Box::new(move |a: u64, b: u64|->u64 {
        web_sys::console::log_2(&a.to_string().into(), &b.to_string().into());
        web_sys::console::log_1(&(a+b).to_string().into());
        a+b
    })  as Box<dyn FnMut(_,_)->(_)>);
    let add:&js_sys::Function=closure.as_ref().unchecked_ref();
    let b49=add.bind1(&JsValue::undefined(), &"49".into());
    let b59=add.bind1(&JsValue::undefined(), &"59".into());
    b49.call2(&JsValue::undefined(), &"13".into(), &"64".into());
    web_sys::console::log_1(&":(".into());
   let three = b49.call2(&JsValue::undefined(), &"1".into(), &"2".into())?;
   web_sys::console::log_2(&"1 + 2 = ".into(), &three.into()); // 1 + 2 = JsValue(3)
  
   b59.call2(&JsValue::undefined(), &"133".into(), &"64".into());


    // for _ in 0..3 {
        let mut v:Vec<(u32,String)>=Vec::new();
        let mut n=1;
        let mut selected : Option<u32>=None;
        let mut thread_rng = thread_rng();
    
        // for _ in 0..10000 {
            // v.push(next_line(&mut n, &mut thread_rng));
        // }
        let body = document.body().unwrap();
        let body=web_sys::Element::from(body);
        let nobody=edom::NoopElementNode {tag:"body"};

        edom::EDOM::render("body", body, move |mut root| {
            web_sys::console::time();
            js_framework_benchmark(&mut root, &mut v, &mut n, &mut thread_rng, &mut selected);
            web_sys::console::time_end();
        });
    // }
    Ok(())
}
