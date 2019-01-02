// #![feature(proc_macro_diagnostic)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Expr, Ident};

mod query {
    use std::fmt;
    use syn::parse::{Parse, ParseStream, Result};
    use syn::spanned::Spanned;
    use syn::{parse_macro_input, parenthesized, Expr, ExprParen, Ident, Token, Type, Stmt, Visibility};

    pub enum QueryPlaceholder {
        Pos(Ident),
        Pin(Expr),
    }

    impl fmt::Debug for QueryPlaceholder {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                QueryPlaceholder::Pos(pos) => write!(f, "Pos({})", pos),
                QueryPlaceholder::Pin(_) => write!(f, "Pin(...)"),
            }
        }
    }

    #[derive(Debug)]
    pub struct QueryStatement {
        pub relation: String,
        pub places: Vec<QueryPlaceholder>,
    }

    use syn::token;
    use syn::punctuated::Punctuated;
    pub struct Query {
        pub statements: Punctuated<QueryStatement, Token![,]>,
        pub body: Expr,
        pub paren: token::Paren,
    }


    impl fmt::Debug for Query {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Query {{ stmts: [")?;
            for stmt in self.statements.iter() {
                write!(f, "{:?}, ", stmt)?;
            }
            write!(f, "], body: ... }}")
        }
    }

    impl Parse for QueryStatement {
        fn parse(input: ParseStream) -> Result<Self> {
            let mut template = vec![];
            let mut places = vec![];
            while !input.is_empty() {
                let lookahead = input.lookahead1();
                if lookahead.peek(Ident) {
                    // WORD
                    let word = input.parse::<Ident>()?;
                    template.push(format!("{}", word));
                } else if lookahead.peek(Token![/]) {
                    // CAPTURES
                    input.parse::<Token![/]>()?;
                    let ident = input.parse::<Ident>()?;
                    input.parse::<Token![/]>()?;
                    template.push(String::from("_"));
                    places.push(QueryPlaceholder::Pos(ident));
                } else if lookahead.peek(syn::token::Paren) || lookahead.peek(syn::Lit) {
                    // PINS
                    let expr = input.parse::<Expr>()?;
                    template.push(String::from("_"));
                    places.push(QueryPlaceholder::Pin(expr));
                } else if lookahead.peek(Token![,]) {
                    break;
                } else {
                    return Err(lookahead.error());
                }
            }
            Ok(QueryStatement {
                relation: template.join(" "),
                places: places,
            })
        }
    }

    /*
            talk_macros::when!((/page/ blahblahblah, time is /t/) {
                println!("Page blahblahblah {:?}", page);
            });
    */
    impl Parse for Query {
        fn parse(input: ParseStream) -> Result<Self> {
            let content;
            Ok(Query {
                paren: parenthesized!(content in input),
                statements: content.parse_terminated(QueryStatement::parse)?,
                body: input.parse()?,
            })
        }
    }
}

use syn::spanned::Spanned;

#[proc_macro_hack]
pub fn when(item: TokenStream) -> TokenStream {
    println!("[when] {:?}", item);
    let query::Query {
        statements,
        body,
        paren,
    } = parse_macro_input!(item as query::Query);

    println!("Body expr\n==========\n{:?}", body);
    println!("");
    println!("Building query statements:");

    let assert_db = quote_spanned! {body.span()=>
        let _rtdb: ::imagine::RTVM = db;
    };

    // region query statement positions
    use std::collections::HashMap;
    let mut ident_found: HashMap<String, usize> = HashMap::new();
    let mut ident_pos: Vec<Ident> = vec![];

    for query::QueryStatement { relation, places } in statements.into_iter() {
        println!("Each {:?}", relation);
        for place in places {
            match place {
                query::QueryPlaceholder::Pos(ident) => {
                    let ident_name = format!("{}", ident);
                    if !ident_found.contains_key(&ident_name) {
                        ident_found.insert(ident_name, ident_pos.len());
                        println!("ident {} '{}'", ident_pos.len(), ident);
                        ident_pos.push(ident);
                    }
                },
                query::QueryPlaceholder::Pin(pin) => println!("pin {:?}", pin),
            }
        }
    }
    println!("Ident found: {:?}", ident_found);
    // endregion query statement positions

    let parameters = quote_spanned! {body.span()=>
        let page = ::imagine::value::text("page 1245");
    };

    println!("parameters {:?}", parameters);

    let expanded = quote! {{
        #assert_db
        let mut __result = vec!["0", "1", "2", "3", "4", "5"];
        #(let #ident_pos = __result.remove(0);)*
        #body
    }};

    TokenStream::from(expanded)
}

/// Add one to an expression.
#[proc_macro_hack]
pub fn add_one(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    TokenStream::from(quote! {
        1 + (#expr)
    })
}
