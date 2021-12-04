use rustyline::Editor;
use v8;

fn eval(scope: &mut v8::HandleScope, src: &str) -> Option<String> {
    let code = v8::String::new(scope, src)?;
    let script = v8::Script::compile(scope, code, None)?;
    let result = script.run(scope)?.to_string(scope)?;

    Some(result.to_rust_string_lossy(scope))
}

fn log_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _m_return: v8::ReturnValue,
) {
    let text = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    println!("{}", text);
}

fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
    let scope = &mut v8::HandleScope::new(isolate);

    let global = v8::ObjectTemplate::new(scope);
    global.set(
        v8::String::new(scope, "log").unwrap().into(),
        v8::FunctionTemplate::new(scope, log_callback).into(),
    );

    let ctx = v8::Context::new_from_template(scope, global);
    let ctx_scope = &mut v8::ContextScope::new(scope, ctx);

    if std::env::args().len() > 1 {
        let file = std::env::args().nth(1).unwrap();
        let content = std::fs::read_to_string(file).unwrap();
        eval(ctx_scope, content.as_str());

        return;
    }

    let mut rl = Editor::<()>::new();
    println!("Press <Ctrl+D> to exit.");
    loop {
        let readline = rl.readline("v8 >> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                match eval(ctx_scope, line.as_str()) {
                    Some(r) => println!("<- {}", r),
                    None => continue,
                }
            }
            Err(_) => break,
        }
    }
}
