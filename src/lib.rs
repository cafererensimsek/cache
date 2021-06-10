extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn cache(_args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let function_name = &item.sig.ident;

    let mut returned_value = None;


    &item.block.stmts.iter().map(|s| {
        match s {
            syn::Stmt::Expr(e) => {
                match e {
                    syn::Expr::Call(c) => {
                        match &*c.func {
                            syn::Expr::Path(p) => {
                                println!("LOOOOOKKKK: {:?}", &p.path.segments[0].ident);
                                if  p.path.segments[0].ident.to_string() == "Ok".to_string() {
                                    returned_value = Some(c.args);
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {},
                }
            },
            _ => {}
        }
    });

    if returned_value.is_none() {
        return quote! {compile_error!("Cannot compile!")}.into()
    }

    let return_type = match &item.sig.output {
        syn::ReturnType::Default => return quote! {compile_error!("Cannot cache without a return type!")}.into(),
        syn::ReturnType::Type(_, t) => {
            let x = match &**t {
                syn::Type::Path(p) => {
                    Some(p.path.segments[0].ident.clone())
                },
                _ => return quote! {compile_error!("Cannot compile!")}.into(),
            };
            x
        },
    };
    let mut parameter_vec = vec![];

    for input in item.sig.inputs.iter() {
        match input {
            // Not sure if I should throw an error or just skip
            syn::FnArg::Receiver(_) => return quote! {compile_error!("Cannot compile!")}.into(),
            syn::FnArg::Typed(i) => match &*i.pat {
                syn::Pat::Ident(id) => parameter_vec.push(id.ident.clone()),
                _ => return quote! {compile_error!("Cannot compile!")}.into(),
            },
        }
    }

    println!("{:#?}", item);

    /* let folded_parameter_vec = parameter_vec.iter().fold(vec![], |mut acc, new| {
        acc.push(new);
        acc
    }); */


    // TODO Bu hatayı çözemedim ama yukarıdaki comment biraz editlenince düzelecek sanırım
    let cache_check = quote! {
        let cache = read_cache(#function_name, #parameter_vec);

        if !cache.is_none() {
            if !cache.clone().unwrap().#function_name.is_none() {
                println!("returning from cache");
                let result = apply_cache_data

                return Ok(read_cache_as::<#return_type>(#function_name, #parameter_vec).unwrap());
            }
        }
    };

    // TODO
    let cache_write = quote! {
        let date = Local::today().naive_local() + Duration::days(7);

        write_cache(
            #function_name,
            #parameter_vec,
            wrap_shared(date),
            wrap_shared(#returned_value),
        );
    };


    // TODO Hata düzelince bunu kontrol edip, doğru birleştirdiğinden emin olmak lazım
    // Önce kontrol edecek eğer cache'de değer varsa onu dönecek
    // Yoksa normal fonksiyonu çalıştıracak
    // Sonra da Ok()'un içinde ne varsa alıp cache'e yazacak
    // TODO Burada direk #item demek #cache_write'a gelmeden dönmesine neden olur
    let final_stream = quote! {
        #cache_check 
        #item  
        #cache_write
    };

    println!("{:#?}", final_stream);

    TokenStream::from(final_stream)

    // TokenStream::new()
}
