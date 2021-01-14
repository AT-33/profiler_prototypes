use quote::{quote, TokenStreamExt};
use proc_macro2::{TokenStream, TokenTree, Group, Delimiter, Literal};

/** Extracts name of a function from its TokenStream. */
fn get_function_name(fn_stream: TokenStream, attr_stream: TokenStream) -> TokenTree {
    let tree = fn_stream.into_iter().nth(1)
        .expect("Can't get function name");
    if let Some(TokenTree::Ident(attr)) = attr_stream.into_iter().nth(0) {
        TokenTree::Literal(Literal::string(attr.to_string().as_str()))
    }
    else if let TokenTree::Ident(id) = tree {
        TokenTree::Literal(Literal::string(id.to_string().as_str()))
    } else {
        panic!("Function name was not an identifier: {}", tree)
    }
}

/**
Function like this:
```
    fn foo() -> T { /* code here */ }
```
Is transformed to:
```
    fn foo() -> T {
        test_profiler::profile_event("foo", true); // begin event

        let _return_value = { /* code here */ };

        test_profiler::profile_event("foo", false); // end event
        _return_value
    }
```
*/
#[proc_macro_attribute]
pub fn profile_func(attr: proc_macro::TokenStream, item: proc_macro::TokenStream)
    -> proc_macro::TokenStream {
    let attr: TokenStream = attr.into();
    let mut func_begin: TokenStream = item.clone()
        .into_iter()
        .take(item.clone().into_iter().count() - 1)
        .collect::<proc_macro::TokenStream>()
        .into();
    let func_body = item.into_iter().last().unwrap();
    let func_body: proc_macro::TokenStream = func_body.into();
    let func_body: TokenStream = func_body.into();
    let func_body: TokenTree = func_body.into_iter().next().unwrap();
    let mut func = TokenStream::new();
    let fn_name = get_function_name(func_begin.clone(), attr);
    func.extend(quote! {
        test_profiler::profile_event( #fn_name , true);
        let _return_value =
    });
    func.append(func_body);
    func.extend(quote! {
        ;
        test_profiler::profile_event( #fn_name , false);
        _return_value
    });
    let func_body = TokenTree::Group(Group::new(Delimiter::Brace, func));
    func_begin.append(func_body);
    func_begin.into()
}
