extern crate rspec;
#[macro_use]
extern crate derive_error;

#[test]
fn error_behaviour() {
    rspec::run(&rspec::describe("the implementation", (), |ctx| {
        ctx.it("should be able to derive from unit structs", |_| {
            #[derive(Debug, Error)]
            pub enum _Error {
                Msg
            }
        });
    }));
}
