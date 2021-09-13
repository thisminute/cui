extern crate log;
extern crate phf;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate simple_logger;
extern crate syn;
mod data;
mod misc;
mod transform;

use {
	data::{ast::Document, semantics::Semantics},
	log::LevelFilter,
	proc_macro::TokenStream,
	proc_macro2::TokenStream as TokenStream2,
	quote::{quote, ToTokens},
	simple_logger::SimpleLogger,
	std::{
		fs::{read_dir, read_to_string, write},
		path::Path,
	},
	syn::parse_macro_input,
};

fn pipeline(document: Document) -> (String, TokenStream2) {
	let mut semantics = document.analyze();
	semantics.render();
	(semantics.html().0, semantics.wasm(true))
}

#[proc_macro]
pub fn cui(input: TokenStream) -> TokenStream {
	SimpleLogger::new()
		.with_level(LevelFilter::Debug)
		.init()
		.unwrap();
	let mut input = input.into();

	// if it exists, import .cui files from the `cui` directory and attach them to the input
	let path = "./cui";
	if Path::new(path).exists() {
		for entry in read_dir(path).expect(&*format!("reading from {}", path)) {
			let entry = entry.expect("reading .cui file");
			let filename = entry.path().display().to_string();
			if filename.ends_with(".cui") {
				let contents: TokenStream2 = read_to_string(entry.path()).unwrap().parse().unwrap();
				contents.to_tokens(&mut input);
			}
		}
	}

	let input = input.into();
	let (html, runtime) = pipeline(parse_macro_input!(input as Document));
	let destination = &format!("target/html/index.html");
	write(destination, html).expect(&*format!("writing output html code to {}", destination));
	write("target/cui_macro_output.rs", runtime.to_string()).expect("writing output rust code");

	runtime.into()
}

#[proc_macro]
pub fn test_setup(input: TokenStream) -> TokenStream {
	if let Ok(_) = SimpleLogger::new().with_level(LevelFilter::Error).init() {}

	let document = parse_macro_input!(input as Document);
	let mut semantics = document.analyze();
	semantics.render();

	let (pages, styles) = semantics.html_parts();
	let content = pages.get("/").unwrap();

	let wasm = semantics.wasm(false);
	let wasm = quote! {
		let window = web_sys::window().expect("getting window");
		let document = &window.document().expect("getting `window.document`");
		let head = &document.head().expect("getting `window.document.head`");
		let body = &document.body().expect("getting `window.document.body`");
		{
			let style = document
					.create_element("style")
					.unwrap()
					.dyn_into::<HtmlElement>()
					.unwrap();
			style.set_inner_text(#styles);
			head.append_child(&style).unwrap();

			let root = document.create_element("div").unwrap();
			body.prepend_with_node_1(&root).unwrap();
			root.set_outer_html(#content);
		}

		{
			#wasm
		}

		let root = body
			.first_child()
			.expect("body should contain the root node")
			.dyn_into::<HtmlElement>()
			.expect("the root node should be an element");
	};

	log::debug!("***************************");
	log::debug!("{}", wasm);
	log::debug!("***************************");

	wasm.into()
}

#[proc_macro]
pub fn test_header(_input: TokenStream) -> TokenStream {
	Semantics::runtime().into()
}
