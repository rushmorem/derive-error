extern crate rspec;
#[macro_use]
extern crate derive_error;

use self::rspec::context::rdescribe;

#[test]
fn error_behaviour() {
    rdescribe("the implementation", |ctx| {
        ctx.it("should be able to derive errors", || {
            #[derive(Debug, Error)]
            pub enum _Error { };
        });
    });
}
