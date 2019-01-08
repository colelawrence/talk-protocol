use imagine::RTVM;
use imagine::value::{Value, Literal, text};


#[test]
fn rt_works() {
    let db = RTVM::default();
    let you = ::imagine::value::text("page 103");

    // talk_macros::rt!(When (/page/ blahblahblah; time is /t/));
    // talk_macros::test_query();
    talk_macros::when!((
        /page/ blahblahblah,
        /page/ points "up" at /target/, // "_ points _ at _", [Cap("page"), Pin("up"), Cap("target")]
        time is /t/,
        /left/ points "right" at (you),
        the temperature is greater than 15.9 "c",
    ) {
        println!("Page blahblahblah {:?}", page);
        println!("page: {:?}, target: {:?}, t: {:?}, left {:?}", page, target, t, left);
    });

    println!("{:?}", text(format!("hello {}", talk_macros::add_one!(3 + 4))));
}

