
// region when macro
#[proc_macro_attribute]
pub fn when(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("[m] attr: {:?}", attr);
    println!("[m] item: {:?}", item);

    item
}

#[proc_macro_attribute]
pub fn when2(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("[m] attr: {:?}", attr);
    println!("[m] item: {:?}", item);

    item
}
// endregion
// region when examples
#[when(/b/ blahblahblah)]
fn a(b: Value) {
    println!("A! {:?}", b);
}

#[when(/b/ blahblahblah, time is /t/)]
fn a2(b: Value, t: Value) {
    println!("When b: {:?}, t: {:?}", b, t);
}
// endregion
// region ignore
#[proc_macro_attribute]
pub fn hello(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn m(attr: TokenStream, item: TokenStream) -> TokenStream {
    let result = quote! {
        fn hello() {
            println!("quote");
        }
    };

    result.into()
}

struct When {
    names: Vec<Ident>,
    length: usize,
    query: QueryStatement,
    then: Expr,
    otherwise: Option<Expr>, 
}

// impl Parse for When {
//     fn parse(input: ParseStream) -> Result<Self> {

//     }
// }


macro_rules! when {
    ($db:expr, [$($stmts:tt)*] $body:block) => {{
        when_idents!($($stmts)*);
        let rt: &RTVM = &$db;
        rt.insert_query(
            Vec::new(),
            Query {
                positions: 3,
                statements: vec![
                    QueryStatement {
                        relation: rt.lookup_relation("_ claims _ blahblahblah."),
                        places: vec![QueryPlaceholder::Pos(0), QueryPlaceholder::Pos(1)],
                    },
                    QueryStatement {
                        relation: rt.lookup_relation("_ claims _ blahblahblah."),
                        places: vec![QueryPlaceholder::Pos(1), QueryPlaceholder::Pos(2)],
                    },
                ],
                resolve: Box::new(|deps: Vec<Si>, vals: Vec<Value>| {
                    let page_a = vals.get(0).expect("value");
                    let page_b = vals.get(1).expect("value");
                    let page_c = vals.get(2).expect("value");

                    $body;
                }),
            }
        );
        println!(stringify!($($stmts)*));
    }}
}

#[proc_macro]
pub fn rt(item: TokenStream) -> TokenStream {
    
    let result = quote! {
        let a = 1;
    };

    result.into()
}
// endregion ignore
