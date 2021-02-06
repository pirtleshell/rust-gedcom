extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(HasEvents)]
pub fn has_events_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_has_events(&ast)
}

fn impl_has_events(ast: &syn::DeriveInput) -> TokenStream {
    // TODO: ensure the struct we're implementing on has `events` property
    // let data = match &ast.data {
    //     syn::Data::Struct(s) => {
    //         println!("{:?}", s);
    //         s
    //     }
    //     _ => panic!("derive(HasEvents) only makes sense on a struct."),
    // };


    // TODO: can we support adding something like this to the generated code?
    // maybe a `can_add_event(&self, event) -> boolean`?!?
    // let event_type = &event.event;
    // for e in &self.events {
    //     if &e.event == event_type {
    //         panic!("Family already has a {:?} event", e.event);
    //     }
    // }

    let name = &ast.ident;
    let gen = quote! {
        impl HasEvents for #name {
            fn add_event(&mut self, event: Event) -> () {
                self.events.push(event);
            }
            fn events(&self) -> Vec<Event> {
                self.events.clone()
            }
            fn dates(&self) -> Vec<String> {
                let mut dates: Vec<String> = Vec::new();
                for event in &self.events {
                    if let Some(d) = &event.date {
                        dates.push(d.clone());
                    }
                }
                dates
            }
            fn places(&self) -> Vec<String> {
                let mut places: Vec<String> = Vec::new();
                for event in &self.events {
                    if let Some(p) = &event.place {
                        places.push(p.clone());
                    }
                }
                places
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
