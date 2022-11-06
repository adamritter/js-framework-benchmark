extern crate console_error_panic_hook;
extern crate smallstr;

use web_sys;
use std::{collections::{HashSet, HashMap}};
use std::panic;

fn add_offset(big_indexer: usize, delta: isize) -> Option<usize> {
    if delta < 0 {
        big_indexer.checked_sub(delta.wrapping_abs() as usize)
    } else {
        big_indexer.checked_add(delta as usize)
    }
}
trait TextNode {
    fn new(text: &str)->Self;
}
struct NoopTextNode  {
    text: String
}
impl TextNode for NoopTextNode {
    fn new(text: &str)->Self {
        Self { text: text.to_string() }
    }
}
struct NoopElementNode {
    tag: &'static str
}

trait Document {
    type TextNode : TextNode;
    type ElementNode : ElementNode;
    fn create_text_node(&self, text: &str)->Self::TextNode;
    fn new()->Self;
    fn create_element(&self, tag: &'static str)->Self::ElementNode;

}

struct NoopDocument {
}
impl Document for NoopDocument {
    type TextNode=NoopTextNode;
    type ElementNode=NoopElementNode;
    fn create_text_node(&self, text: &str)->NoopTextNode {
        NoopTextNode {  text: text.to_string() }
    }
    fn new()->Self {
        NoopDocument {}
    }
    fn create_element(&self, tag: &'static str)->Self::ElementNode {
        NoopElementNode {tag}
    }
}

impl Document for web_sys::Document {
    type TextNode=web_sys::Text;
    type ElementNode=web_sys::Element;
    fn new()->Self {
        let window = web_sys::window().unwrap();
        window.document().unwrap()
    }
    fn create_text_node(&self, text: &str)->web_sys::Text {
        self.create_text_node(text)
    }
    
    fn create_element(&self, tag: &'static str)->web_sys::Element {
        self.create_element(tag).unwrap()
    }
}

impl TextNode for web_sys::Text {
    fn new(text:&str)->Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        document.create_text_node(text)
    }
}

trait GenericNode  : Sized {
    type TextNode : TextNode;
    type ElementNode : ElementNode;
    fn into_text_node(self)->Self::TextNode;
    fn into_element_node(self)->Self::ElementNode;
}
impl GenericNode for web_sys::Node {
    type ElementNode = web_sys::Element;
    type TextNode = web_sys::Text;
    fn into_text_node(self)->Self::TextNode {
        self.dyn_into().unwrap()
    }
    fn into_element_node(self)->Self::ElementNode {
        self.dyn_into().unwrap()
    }
}

struct NoopNode {
}

impl GenericNode for NoopNode {
    type ElementNode = NoopElementNode;
    type TextNode = NoopTextNode;
    fn into_text_node(self)->Self::TextNode {
        NoopTextNode { text: "hello".to_string()}
    }
    fn into_element_node(self)->Self::ElementNode {
        NoopElementNode { tag: "hello"}
    }
}

pub trait ElementNode : Sized {
    type GenericNode : GenericNode<TextNode=Self::TextNode, ElementNode=Self>;
    type TextNode : TextNode;
    type Document : Document<TextNode=Self::TextNode, ElementNode=Self>;
    fn replace_text_child(&self, new: &Self::TextNode, old: &Self::TextNode);
    fn append_child(&self, child: &Self);
    fn append_child_before(&self, child: &Self, next_sibling: &Self);
    fn append_child_after(&self, child: &Self, prev_sibling: &Self);
    fn prepend_child(&self, child: &Self);
    fn append_text_child(&self, child: &Self::TextNode);
    fn set_attribute(&self, name: &str, value: &str);
    fn remove_child(&self, child: &Self);
    fn remove(&self);
    fn new(tag: &'static str)->Self;
    fn create_event_listener(&self, f : Rc<RefCell<dyn FnMut(u64, &'static str)>>, uid:u64, name:&'static str);
    fn deep_clone(&self)->Self;
    fn get_child_nodes(&self)->Vec<Self::GenericNode>;
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;
use std::rc::Rc;
use std::cell::RefCell;

impl ElementNode for web_sys::Element {
    type TextNode = web_sys::Text;
    type Document = web_sys::Document;
    type GenericNode = web_sys::Node;
    fn replace_text_child(&self, new: &Self::TextNode, old: &Self::TextNode) {
        self.replace_child(new, old);
    }
    fn append_child(&self, child: &Self) {
        web_sys::Node::append_child(self, child);
    }
    fn append_child_before(&self, child: &Self, next_sibling: &Self) {
        self.insert_before(child, Some(next_sibling));
    }
    fn append_child_after(&self, child: &Self, prev_sibling: &Self) {
        let next_sibling=prev_sibling.next_sibling();
        self.insert_before(child, next_sibling.as_ref());
    }
    fn prepend_child(&self, child: &Self) {
        self.insert_before(child, self.first_child().as_ref());
    }
    fn append_text_child(&self, child: &Self::TextNode) {
        self.insert_before(child, None);
    }
    fn set_attribute(&self, name: &str, value: &str) {
        web_sys::Element::set_attribute(self, name, value);
    }
    fn remove_child(&self, child: &Self) {
        web_sys::Element::remove_child(self, child);
    }
    fn remove(&self) {
        web_sys::Element::remove(&self);
    }
    fn new(tag: &'static str)->Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        document.create_element(tag).unwrap()
    }
    fn create_event_listener(&self, f : Rc<RefCell<dyn FnMut(u64, &'static str)>>, uid:u64, name:&'static str) {
        let closure = Closure::wrap(Box::new(move |e: Event| {
            web_sys::console::log_2(&uid.to_string().into(), &name.into());
            f.borrow_mut()(uid, name);
            e.prevent_default();
        })  as Box<dyn FnMut(_)>);
        self.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref());
        closure.forget();
    }
    fn deep_clone(&self)->Self {
        self.clone_node_with_deep(true).unwrap().dyn_into::<web_sys::Element>().unwrap()
    }
    fn get_child_nodes(&self)->Vec<Self::GenericNode> {
        let this_node : &web_sys::Node = self.as_ref();
        let node_list = this_node.child_nodes();
        let mut r : Vec<Self::GenericNode>=Vec::new();
        for i in 0..node_list.length() {
            r.push(node_list.get(i).unwrap());
        }
        r
    }

    
}
impl  ElementNode for NoopElementNode {
    type TextNode=NoopTextNode;
    type Document=NoopDocument;
    type GenericNode=NoopNode;
    fn new(tag: &'static str)->Self {
        Self { tag }
    } 
    fn create_event_listener(&self, f : Rc<RefCell<dyn FnMut(u64, &'static str)>>, uid:u64, name:&'static str) {
    }

    fn replace_text_child(&self, new: &NoopTextNode, old: &NoopTextNode) {
    }
    fn append_child(&self, child: &NoopElementNode) {

    }
    fn append_child_before(&self, child: &NoopElementNode, next_sibling: &NoopElementNode) {

    }
    fn append_child_after(&self, child: &NoopElementNode, prev_sibling: &NoopElementNode) {

    }
    fn prepend_child(&self, child: &NoopElementNode) {

    }
    fn append_text_child(&self, child: &NoopTextNode) {

    }
    fn set_attribute(&self, name: &str, value: &str) {

    }
    fn remove_child(&self, child: &NoopElementNode) {

    }
    fn remove(&self) {
    }
    fn deep_clone(&self)->Self {
        NoopElementNode { tag: self.tag }
    }
    fn get_child_nodes(&self)->Vec<Self::GenericNode> {
        Vec::new()
    }

}
static mut last_uid:u64=0;
impl NoopDocument {
    fn create_element(tag: &'static str)->NoopElementNode {
        NoopElementNode { tag }
    }
    fn create_text_node(text: String)->NoopTextNode {
        NoopTextNode {  text }
    }
    fn next_uid()->u64 {
        unsafe {
        let r=last_uid;
        last_uid+=1;
        return r;}
    }
    fn create_event_listener(dnode:&NoopElementNode, uid:u64, name:&'static str) {
    }
}

enum Node<EN> where EN:ElementNode {
    Text(String, EN::TextNode),
    Element(Element<EN>),
    ForEach(Vec<(u64, Element<EN>)>)
}

// TODO: speed up web framework by not creating JS DOM elements unless needed.
enum DNode<'a, EN> where EN:ElementNode {
    JSRef(EN),
    ParentRef(&'a Element<EN>, usize)
}

struct Element<EN> where EN:ElementNode {
    name: &'static str,
    attr: Vec<(&'static str,String)>,
    children: Vec<Node<EN>>,
    dnode: EN,
    events: Vec<&'static str>,
    uid: u64,
}

impl<EN> Element<EN>  where EN:ElementNode {
    fn new(name: &'static str, dnode: EN, uid: u64)->Self {
        Self {name, attr:vec![], children: vec![], dnode, events: Vec::new(), uid}
    }
    fn clone_using_dnode(&self, target_dnode: EN, edom: &mut EDOM<EN>)->Self {
        let mut r = Self {name: self.name, attr: self.attr.clone(), children: Vec::new(), dnode: target_dnode, events: self.events.clone(), uid: edom.next_uid()};
        // TODO: attach events
        for event_name in &self.events {
            r.dnode.create_event_listener(edom.fire_event.clone(), r.uid, event_name);
        }
        // TODO: attach children
        let mut next_child_idx=0;
        let new_dchildren=r.dnode.get_child_nodes();
        for new_dchild in new_dchildren.into_iter() {
            let child = &self.children[next_child_idx];
            match child {
                Node::Text(s, _)=>{
                    r.children.push(Node::Text(s.clone(), new_dchild.into_text_node()));
                },
                Node::Element(e)=>{
                    r.children.push(Node::Element(e.clone_using_dnode(new_dchild.into_element_node(), edom)));
                },
                _ => {
                    panic!("Only cloning element and text is supported so far");
                }
            }
            next_child_idx+=1;
        }
        r
    }
}



pub struct ElementIterator<'a, EN> where EN: ElementNode {
    edom: &'a mut EDOM<EN>,
    element: &'a mut Element<EN>,
    attrpos: usize,
    childpos: usize,
    eventpos: usize,
}

impl<'a, EN> ElementIterator<'a, EN> where EN:ElementNode {
    fn element_with<'z, F>(&'z mut self, name : &'static str, mut f:F) where F:FnMut(ElementIterator<'z,EN>) {
       f(self.element(name));
    }
    fn new(edom:&'a mut EDOM<EN>, element:&'a mut Element<EN>)->Self {
        Self {edom, element, attrpos: 0, childpos: 0, eventpos: 0}
    }
    pub fn element<'z>(&'z mut self, name : &'static str)->ElementIterator<'z,EN> {
        let mut edom=&mut self.edom;
        let element=&mut self.element;
        if edom.create {
            let mut elem= Element::new(name, edom.document.create_element(name), edom.next_uid());
            element.dnode.append_child(&elem.dnode);
            element.children.push(Node::Element(elem));
            if let Node::Element(elem2)= element.children.last_mut().unwrap() {
                let mut iterator=ElementIterator::new(edom, elem2);
                return iterator;
            } else {
                panic!("???")
            }
        } else {
            if let Node::Element(child)=&mut element.children[self.childpos] {
                self.childpos+=1;
                let mut iterator=ElementIterator::new(edom, child);
                return iterator;
            } else {
                panic!("Not Element")
            }
        }
    }

    pub fn attr<'z>(&'z mut self, name: &'static str, value: &str)->&'z mut ElementIterator<'a,EN> {
        if self.edom.create {
            self.element.dnode.set_attribute(name, value);
            self.element.attr.push((name, value.into()));
        } else { 
            let thisattr=&mut self.element.attr[self.attrpos];
            if thisattr.0 != name {
                panic!("name change")
            }
            if thisattr.1 != value {
                self.element.dnode.set_attribute(name, value)
            }
            self.attrpos+=1
        }
        self
    }
    
    pub fn text(&mut self, text:&str) {
        if self.edom.create {
            let tdnode=self.edom.document.create_text_node(text);
            self.element.dnode.append_text_child(&tdnode);
            let mut elem=Node::Text(text.into(), tdnode);
            self.element.children.push(elem);
        } else {
            let mut elem = &mut self.element.children[self.childpos];
            self.childpos+=1;
            if let Node::Text(text2, tdnode)=elem {
                if *text != **text2 {
                    *text2=text.into();
                    let mut newChild=self.edom.document.create_text_node(text);
                    self.element.dnode.replace_text_child(&newChild, &tdnode);
                    *tdnode=newChild;
                }
            } else {
                panic!("Not text");
            }
        }
    }
    fn event<'z,F>(&'z mut self, name:&'static str, mut f: F)->&'z mut Self where F:FnMut() {
        if self.edom.create {
            self.element.events.push(name);
            self.element.dnode.create_event_listener(self.edom.fire_event.clone(), self.element.uid, name);
        } else if let Some(ev) = self.edom.firing_event  {
            if self.element.uid == ev.0 {
                if *self.element.events[self.eventpos]==*ev.1  {
                    f();
                }
                self.eventpos+=1;
            }
        }
        self
    }
    pub fn button<'z>(&'z mut self, text: &str)->ElementIterator<'z,EN> {
        let mut elem=self.element("button");
        elem.text(text);
        elem
    }
    pub fn id(&mut self, id: &str)->&mut Self {
        self.attr("id", id)
    }
    pub fn class(&mut self, id: &str)->&mut Self {
        self.attr("class", id)
    }
    pub fn div<'z, FCB>(&'z mut self, mut fcb: FCB)->ElementIterator<'z,EN> where FCB:FnMut(&mut ElementIterator<EN>) {
        let mut r=self.element("div");
        fcb(&mut r);
        return r;
    }
    pub fn click<'z, F>(&'z mut self, mut f:F)->&'z mut Self where F:FnMut() {
        self.event("click", f)
    }
    pub fn clicked(&mut self)->bool {
        let mut r=false;
        self.event("click", || r=true);
        r
    }
    pub fn h1<'z>(&'z mut self)->ElementIterator<'z,EN> {
        self.element("h1")
    }
    
    fn for_each_consolidate<'z, FIdx, FCB, I, C>(&'z mut self, list : C, mut fidx: FIdx, tag: &'static str, mut fcb: FCB) where C:Iterator<Item=I>, FIdx:FnMut(&I)->u64, FCB:FnMut(&mut I, &mut ElementIterator<EN>) {
        let Node::ForEach(v) : &mut Node<EN>=&mut self.element.children[self.childpos] else {
            panic!("Bad node");
        };
        web_sys::console::log_2(&"consolidate, vlength:".into(), &v.len().to_string().into());

        // Remove old children from DOM.
        let mut newidxs:HashSet<u64>=HashSet::new();
        let mut list2=Vec::new();
        for l in list {
            list2.push(l);
        }

        for (_, e) in list2.iter().enumerate() {
            newidxs.insert(fidx(&e));
        }
        for (idx, elem) in v.iter().rev() {
            if !newidxs.contains(&idx) {
                elem.dnode.remove();
            }
        }
        // Remove old children from v.
        v.retain(|(idx, _)| newidxs.contains(idx));
        // position[idx]+relpos will contain the position of idx Element for all shown elements in v[ii] where ii>=i.
        let mut position:HashMap<u64, usize>=HashMap::new();
        for (i, (idx, _)) in &mut v.iter().enumerate() {
            position.insert(*idx, i);
        }
        let mut relpos: isize=0;
        let mut wrong_place: HashSet<u64>=HashSet::new();
        let mut create=Vec::new();
        let mut edom : &mut EDOM<EN>=&mut self.edom;

        // first_create is slower for some reason
        let first_create=true;

        web_sys::console::log_1(&"in for_each stuff".into());

        for (i, mut e) in list2.iter_mut().enumerate() {
            let idx=fidx(&e);
            if let Some(pos)=position.get(&idx).cloned() {
                let abspos=add_offset(pos, relpos).unwrap();
                if abspos!=i {
                    // Switch, set wrong_place indicators.
                    wrong_place.insert(idx);
                    wrong_place.insert(v[i].0);
                    v.swap(i, abspos);
                    // Swap positions of idx and v[i].0
                    position.insert(idx, *position.get(&v[i].0).unwrap());
                    position.insert(v[i].0, pos);
                }
                if wrong_place.contains(&idx) {
                    wrong_place.remove(&idx);
                    if i==0 {
                        self.element.dnode.prepend_child(&v[i].1.dnode);
                    } else {
                        self.element.dnode.append_child_after(&v[i].1.dnode, &v[i-1].1.dnode)
                    }
                }
                if first_create {
                    fcb(&mut e, &mut ElementIterator::new(edom, &mut v[i].1));
                } else {
                    create.push(false);
                }
            } else {
                // Insert new elem, increase relpos.
                let mut elem = Element::new(tag, edom.document.create_element(tag), edom.next_uid());

                if ! first_create {
                    create.push(true);

                    if i == v.len() {
                        self.element.dnode.append_child(&elem.dnode);
                    } else {
                        self.element.dnode.append_child_before(&elem.dnode, &v[i].1.dnode);
                    }
                }

                v.insert(i, (idx, elem));
                relpos+=1;
                if first_create {
                    if i>0 && edom.clone_for_each {
                        let new_dnode=v[0].1.dnode.deep_clone();
                        v[i]=(v[i].0, v[0].1.clone_using_dnode(new_dnode, edom));
                    } else {
                        edom.create=true;
                        fcb(&mut e, &mut ElementIterator::new(edom, &mut v[i].1));
                        edom.create=false;
                    }
                    if i+1 == v.len() {
                        self.element.dnode.append_child(&v[i].1.dnode);
                    } else {
                        self.element.dnode.append_child_before(&v[i].1.dnode, &v[i+1].1.dnode);
                    }
                }

            }
        }

        if !first_create {
            for (mut e, (ie, should_create)) in list2.iter_mut().zip(v.iter_mut().zip(create.iter())) {
                let elem=&mut ie.1;
                edom.create=*should_create;
                let mut it=ElementIterator::new(edom, elem);
                fcb(&mut e, &mut it);
                edom=it.edom;
            }
        }

        self.childpos+=1;
        edom.create=false;
    }
    
    pub fn for_each<'z, FIdx, FCB, I, C>(&'z mut self, list : C, mut fidx: FIdx, tag: &'static str, mut fcb: FCB) where C:Iterator<Item=I>, FIdx:FnMut(&I)->u64, FCB:FnMut(&mut I, &mut ElementIterator<EN>) {
        if self.edom.create {
            web_sys::console::log_1(&"for_each create".into());
            self.element.children.push(Node::ForEach(Vec::new()));
            let Node::ForEach(v)=self.element.children.last_mut().unwrap() else {
                panic!("Not foreach")
            };
            let mut list2=Vec::new();
            for l in list {
                list2.push(l);
            }
            let mut s:HashSet<u64>=HashSet::with_capacity(list2.len());

            for l in &list2 {
                let idx=fidx(&l);
                if !s.insert(idx) {
                    panic!("Idx must be unique.")
                }
                let elem=Element::new(tag, self.edom.document.create_element(tag), self.edom.next_uid());
                v.push((idx, elem))
            }
            let mut edom : &'z mut EDOM<EN>=&mut self.edom;

            for (mut l, ve) in std::iter::zip(list2, v.iter_mut()) {
                let elem=&mut ve.1;
                self.element.dnode.append_child(&elem.dnode);
                let mut it: ElementIterator<'z,EN>=ElementIterator {edom: edom, attrpos: 0, childpos: 0, eventpos: 0, element: elem};
                fcb(&mut l, &mut it);
                edom=it.edom;
            }
        } else {
            // Algorithm must be fast enough with 1000 new / clear all (0 common) and also with very few differences.
            self.for_each_consolidate(list, fidx, tag, fcb);
        }
    }
}

pub struct EDOM<EN> where EN:ElementNode {
    firing_event: Option<(u64, &'static str)>,
    last_uid: u64,
    create: bool,
    document: EN::Document,
    fire_event: Rc<RefCell<Box<dyn FnMut(u64, &'static str)>>>,
    clone_for_each: bool,  // Clone node for for_each instead of building up the DOM tree.
}

impl<EN> EDOM<EN> where EN:ElementNode {
    fn next_uid(&mut self)->u64 {
        let r=self.last_uid;
        self.last_uid+=1;
        return r;
    }
    
    fn render_once<F>(&mut self, root: &mut Element<EN>, mut f:F) where EN:ElementNode, F:FnMut(ElementIterator<EN>)->() {
        let ei=ElementIterator::new(self, root);
        f(ei);
        self.create=false;
    }

    pub fn render<F>(tag: &'static str, root: EN, mut f:F) where EN:ElementNode + 'static, F:FnMut(ElementIterator<EN>) + 'static {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        web_sys::console::time();
        let mut edom : EDOM<EN>=EDOM::new();
        let mut el=Element::new("body", root, edom.next_uid());
        edom.render_once(&mut el, &mut f);
        let fire_event=edom.fire_event.clone();
        let edomrc : Rc<RefCell<EDOM<EN>>>=Rc::new(RefCell::new(edom));
        let edomrc2=edomrc.clone();
        *fire_event.borrow_mut()=Box::new(move |a:u64, b:&'static str| {
            web_sys::console::time();
            web_sys::console::log_3(&"rc21".into(), &a.to_string().into(), &b.into());
            let mut edom=edomrc2.borrow_mut();
            edom.firing_event=Some((a, b));
            edom.render_once(&mut el, &mut f);
            edom.firing_event=None;
            edom.render_once(&mut el, &mut f);
            web_sys::console::time_end();
        });
        std::mem::forget(edomrc);
        std::mem::forget(fire_event);
        web_sys::console::time_end();
        web_sys::console::log_1(&"Done initial render".into());
    }

    fn new()->Self {
        let fire_event:Rc<RefCell<Box<dyn FnMut(u64, &'static str)>>>=Rc::new(RefCell::new(Box::new(|a:u64, b:&'static str| web_sys::console::log_3(&"rc".into(), &a.to_string().into(), &b.into()))));
        EDOM {fire_event, firing_event: None, last_uid: 0, create: true, document:EN::Document::new(), clone_for_each: true}
    }
}
/*

}
 */