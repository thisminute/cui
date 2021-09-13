mod css;
mod cui;
pub use self::cui::{CuiProperty, PageProperty};

use {self::css::CSS_PROPERTIES, data::ast::Value, std::collections::HashMap};

pub type CssProperty = String;

pub type PageProperties = HashMap<PageProperty, Value>;
pub type CssProperties = HashMap<CssProperty, Value>;
pub type CuiProperties = HashMap<CuiProperty, Value>;

pub type CssRules = HashMap<String, CssProperties>;

#[derive(Default, Clone)]
pub struct Properties {
	pub page: PageProperties,
	pub css: CssProperties,
	pub cui: CuiProperties,
}

pub fn is_css_property(name: &str) -> bool {
	CSS_PROPERTIES.contains(&name)
}

// use {quote::quote, TokenStream};
// 	match &**name {
// 		"blur" => quote! { set_onblur },
// 		"focus" => quote! { set_onfocus },
// 		"click" => quote! { set_onclick },
// 		"mouseover" => quote! { set_onmouseover },
// 		"mouseenter" => quote! { set_onmouseenter },
// 		"mouseleave" => quote! { set_onmouseleave },
// 		"mouseout" => quote! { set_onmouseout },
// 		_ => panic! {"unrecognized html event {}", name },
// 	}
