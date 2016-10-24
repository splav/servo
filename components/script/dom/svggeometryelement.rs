/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::attr::Attr;
use dom::bindings::codegen::Bindings::SVGGeometryElementBinding;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::Root;
use dom::bindings::str::DOMString;
use dom::document::Document;
use dom::element::AttributeMutation;
use dom::node::Node;
use dom::svggraphicselement::SVGGraphicsElement;
use dom::virtualmethods::VirtualMethods;
use string_cache::Atom;
use style::attr::AttrValue;

#[dom_struct]
pub struct SVGGeometryElement {
    svggraphicselement: SVGGraphicsElement
}

impl SVGGeometryElement {
    pub fn new_inherited(local_name: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> SVGGeometryElement {
        SVGGeometryElement {
            svggraphicselement:
                SVGGraphicsElement::new_inherited(local_name, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(local_name: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<SVGGeometryElement> {
        Node::reflect_node(box SVGGeometryElement::new_inherited(local_name, prefix, document),
                           document,
                           SVGGeometryElementBinding::Wrap)
    }
}

impl VirtualMethods for SVGGeometryElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<SVGGraphicsElement>() as &VirtualMethods)
    }

    fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
        self.super_type().unwrap().attribute_mutated(attr, mutation);
    }

    fn parse_plain_attribute(&self, name: &Atom, value: DOMString) -> AttrValue {
        match name {
            &atom!("cx") | &atom!("cy") | &atom!("r") => AttrValue::from_u32(value.into(), 0),
            //TODO(splav): implement full fill attribute spec
            &atom!("fill") => AttrValue::from_legacy_color(value.into()),
            _ => self.super_type().unwrap().parse_plain_attribute(name, value),
        }
    }
}
