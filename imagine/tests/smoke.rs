use imagine::*;

macro_rules! when_wrt {
    ( /$x:ident/, $e:tt ) => {{
        let mut $x = Vec::new();
    }};
    ( ($x:expr), $e:tt ) => {{
        println!("{:?}", $x);
    }};
}
macro_rules! when_idents {
    ($w:expr, ) => (());

    ($w:expr, /$x:ident/ $($rest:tt)* ) => {
        let $x = $w.pop().expect(stringify!($x));
        when_idents!($w, $($rest)*);
    };
    ($w:expr,  ($x:expr) $($rest:tt)* ) => {
        when_idents!($w, $($rest)*);
    };
    ($w:expr,  $tag:ident $($rest:tt)* ) => {
        when_idents!($w, $($rest)*);
    };
}

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

fn test_macro(rt: &RTVM) {
    let page1 = "page1";
    // (page1) points ("up") at /target/
    let mut vals: Vec<Value> = vec![
        text("banker value"),
        text("friend value"),
    ];
    when_idents!(vals, /friend/ loves /banker/);

    println!("friend {:?}", friend);
    println!("banker {:?}", banker);
    // when!(rt, [/friend/ blahblahblah] {
    //     println!("target: {:?}", target);
    // });
}

#[test]
fn it_works() {
    println!("Hello, world!");
    //region setup
    let rt = RTVM::default();
    // let rel2 = rt.lookup_relation("_ claims _ points _ at _.");
    // println!("Should be same first rel {:?} == {:?}", rel, rel2);
    // let si2 = rt.insert_statement(vec![], rel2, StatementValue(vec![Value::text("supporter2"), Value::text("page1"), Value::text("up"), Value::text("page2")]));

    let rel = rt.lookup_relation("_ claims _ blahblahblah.");
    let si1 = rt.insert_statement(
        vec![],
        rel,
        StatementValue::new(vec![text("page1"), text("page2")]),
    );
    let si3 = rt.insert_statement(
        vec![],
        rel,
        StatementValue::new(vec![text("page2"), text("page3")]),
    );
    let si4 = rt.insert_statement(
        vec![],
        rel,
        StatementValue::new(vec![text("page0"), text("page3")]),
    );
    let si5 = rt.insert_statement(
        vec![],
        rel,
        StatementValue::new(vec![text("page3"), text("page1")]),
    );
    //endregion setup

    test_macro(&rt);

    let qi1 = rt.insert_query(
        vec![],
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

                println!("blahblahblah: {} -> {} -> {}", page_a, page_b, page_c);
            }),
        },
    );
}
