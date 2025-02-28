extern crate proc_macro;

use proc_macro::*;

use quote::quote;

use syn::{Expr, ExprReturn, FnArg, ReturnType, Stmt};

#[proc_macro_attribute]
/// Convenience procedural macro that allows you to bind an async function to the [`InteractionHandler`]
pub fn slash_command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // There is _probably_ a more efficient way to do what I want to do, but hey I am here
    // to learn so why not join me on my quest to create this procedural macro...lol

    let mut defer = false;

    // Parse the stream of tokens to something more usable.
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    // Let's see if the programmer wants to respond with a deferring acknowlegdement first.
    // If so, the end-result needs to be built differently.
    for at in &input.attrs {
        for seg in at.path.segments.clone() {
            if seg.ident == "defer" {
                defer = true;
            }
        }
    }

    // Ok here comes the fun part

    // Get the function name
    let fname = &input.sig.ident;
    // Get the visibility (public fn, private fn, etc)
    let vis = &input.vis;

    // Get the parameters and return types
    let params = &input.sig.inputs;
    let ret_sig = &input.sig.output;

    // Must be filled later, but define its type for now.
    let ret: syn::Type;

    // Get the function body
    let body = &input.block;

    // Check for a proper return type and fill ret if found.
    match ret_sig {
        ReturnType::Default => {
            panic!("Expected an `InteractionResponse` return type");
        }
        ReturnType::Type(_a, b) => {
            ret = *b.clone();
        }
    }

    // Using quasi-quoting to generate a new function. This is what will be the end function returned to the compiler.
    if !defer {
        let subst_fn = quote! {
            #vis fn #fname<'context> (#params) -> ::std::pin::Pin<::std::boxed::Box<dyn 'context + Send + ::std::future::Future<Output = #ret>>>{
                Box::pin(async move {
                    #body
                })
            }
        };
        subst_fn.into()
    }
    // Deferring is requested, this will require a bit more manipulation.
    else {
        // Find the return statement and split the entire tokenstream there.
        let mut ind: Option<usize> = None;
        let mut expr: Option<ExprReturn> = None;
        for n in 0..body.stmts.len() {
            let s = &body.stmts[n];
            match s {
                Stmt::Expr(e) => match e {
                    Expr::Return(a) => {
                        expr = Some(a.clone());
                        ind = Some(n);
                        break;
                    }
                    _ => (),
                },
                Stmt::Semi(e, _) => match e {
                    Expr::Return(a) => {
                        expr = Some(a.clone());
                        ind = Some(n);
                        break;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        let (nbody, _reta) = body.stmts.split_at(ind.unwrap_or_else(|| {
            panic!(
                "Could not find return statement in slash-command. Explicit returns are required."
            );
        }));

        // Create a nice block out of it.
        let vbody = syn::Block {
            brace_token: body.brace_token,
            stmts: nbody.to_vec(),
        };

        // Find the name of the Context parameter
        let mut ctxname: Option<syn::Ident> = None;
        for p in params {
            match p {
                FnArg::Typed(t) => match &*t.pat {
                    syn::Pat::Ident(a) => {
                        ctxname = Some(a.ident.clone());
                        break;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        // Unwrap, unwrap, unwrap, unwrap.
        let expra = expr
            .unwrap_or_else(|| panic!("Expected return"))
            .expr
            .unwrap_or_else(|| panic!("Expected some return value"));

        // Now that we have all the information we need, we can finally start building our new function!
        // The difference here being that the non-deffered function doesn't have to spawn a new thread that
        // does the actual work. Here we need it to reply with a deffered channel message.
        let subst_fn = quote! {
            #vis fn #fname<'context> (#params) -> ::std::pin::Pin<::std::boxed::Box<dyn 'context + Send + ::std::future::Future<Output = #ret>>>{
                Box::pin(async move {

                    ::actix::Arbiter::spawn(async move {
                        #vbody
                        #ctxname.edit_original(&WebhookMessage::from(#expra)).await;
                    });

                    return InteractionResponseBuilder::default().respond_type(InteractionResponseType::DEFFERED_CHANNEL_MESSAGE_WITH_SOURCE).finish();

                })
            }
        };
        subst_fn.into()
    }
}

#[proc_macro_attribute]
/// Send out a deffered channel message response before doing work.
pub fn defer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
