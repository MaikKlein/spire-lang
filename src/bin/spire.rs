extern crate spire;
use std::fs::File;
use std::io::Read;
use spire::Parser;
fn main() {
    let mut file = File::open("test/test.spire").expect("file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("read");
    let r = spire::entry().parse(input.as_str()).expect("fail");
    println!("{:#?}", r);
    spire::fields()
        .parse("Test1:Int,Test2:Int,Test3:Int")
        .expect("fields");

    spire::fields()
        .parse("Test1:Int,Test2:Int,Test3:Int,")
        .expect("fields with comma");

    spire::extension()
        .parse(
            r#"
            extension Test {
                Test1:Int,Test2:Int,Test3:Int
            }
        "#,
        )
        .expect("extension fields");

    // error :(
    spire::extension()
        .parse(
            r#"
            extension Test {
                Test1:Int,Test2:Int,Test3:Int,
            }
        "#,
        )
        .expect("extension fields with comma");
}
