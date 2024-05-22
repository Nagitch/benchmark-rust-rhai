use rhai::{Engine, EvalAltResult, Scope};
use plotly::{common::{Mode, Title}, layout::Axis, Layout, Plot, Scatter};
use std::time::Instant;

fn main() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let script = "
        fn fibonacci(n) {
            if n <= 1 { return n; }
            else { return fibonacci(n-1) + fibonacci(n-2); }
        }
        fibonacci(n)
    ";
    let mut rust_times = Vec::new();
    let mut direct_times = Vec::new();
    let mut ast_times = Vec::new();
    let mut ns = Vec::new();

    for n in 1i64..=10i64 {
        ns.push(n);
        let mut scope = Scope::new();
        scope.push("n", n);

        // Rust (vanilla)
        let start = Instant::now();
        let _rust_result = fibonacci(n as i64);
        let rust_duration = start.elapsed().as_secs_f64();
        rust_times.push(rust_duration);

        // Rhai (direct)
        let start = Instant::now();
        let _direct_result: i64 = engine.eval_with_scope(&mut scope, script)?;
        let direct_duration = start.elapsed().as_secs_f64();
        direct_times.push(direct_duration);

        // Rhai (AST)
        let ast = engine.compile_with_scope(&mut scope, script)?;
        let start = Instant::now();
        let _ast_result: i64 = engine.eval_ast_with_scope(&mut scope, &ast)?;
        let ast_duration = start.elapsed().as_secs_f64();
        ast_times.push(ast_duration);
    }

    // プロットの作成
    let trace1 = Scatter::new(ns.clone(), rust_times)
        .name("Rust")
        .mode(Mode::LinesMarkers);
    let trace2 = Scatter::new(ns.clone(), direct_times)
        .name("Rhai Direct")
        .mode(Mode::LinesMarkers);
    let trace3 = Scatter::new(ns.clone(), ast_times)
        .name("Rhai AST")
        .mode(Mode::LinesMarkers);

    let layout = Layout::new()
        .title(Title::new("Rust vs Rhai and Rhai (AST) Fibonacci Benchmark"))
        .x_axis(Axis::new().title(Title::new("n: Fibonacci number index")))
        .y_axis(Axis::new().title(Title::new("Time (s)")));
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    plot.write_html("benchmark_results.html");

    Ok(())
}

fn fibonacci(n: i64) -> i64 {
    if n <= 1 { n } else { fibonacci(n - 1) + fibonacci(n - 2) }
}
