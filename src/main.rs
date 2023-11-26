use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

fn main() {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let codegen = CodeGen::new(&context, module, builder);
    codegen.generate();
}

struct CodeGen<'codegen> {
    context: &'codegen Context,
    module: Module<'codegen>,
    builder: Builder<'codegen>,
}

impl<'codegen> CodeGen<'codegen> {
    pub fn new(
        context: &'codegen Context,
        module: Module<'codegen>,
        builder: Builder<'codegen>,
    ) -> Self {
        Self {
            context,
            module,
            builder,
        }
    }

    pub fn generate(&self) {
        self.module.print_to_file("out.ll").unwrap();
    }
}
