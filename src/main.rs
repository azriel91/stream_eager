use fn_graph::FnGraphBuilder;
use futures::stream::StreamExt;
use resman::{IntoFnRes, Resources};

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_time()
        .build()
        .unwrap();

    rt.block_on(async {
        let full_start = tokio::time::Instant::now();
        let _v = futures::stream::iter([0, 1, 2, 3, 4, 5].into_iter())
            .map(|n| {
                let now = tokio::time::Instant::now();
                (now, n, n * 2)
            })
            .map(|(start, n, n2)| async move {
                let elapsed = start.elapsed();

                // elapsed should all be small!
                println!("{n}: {elapsed:?}");

                n2
            })
            .buffered(100)
            .fold(Vec::new(), |mut v, n2| async move {
                v.push(n2);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                v
            })
            .await;
        let full_elapsed = full_start.elapsed();
        println!("fold: {full_elapsed:?}");

        println!();

        let full_start = tokio::time::Instant::now();
        let _v = futures::stream::iter([0, 1, 2, 3, 4, 5].into_iter())
            .map(|n| {
                let now = tokio::time::Instant::now();
                (now, n, n * 2)
            })
            .map(|(start, n, n2)| async move {
                let elapsed = start.elapsed();

                // elapsed should all be small!
                println!("{n}: {elapsed:?}");

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                n2
            })
            .buffer_unordered(7)
            .collect::<Vec<_>>()
            .await;
        let full_elapsed = full_start.elapsed();
        println!("collect: {full_elapsed:?}");

        println!();

        let full_start = tokio::time::Instant::now();
        let mut v = vec![0; 10];
        futures::stream::iter([0, 1, 2, 3, 4, 5].into_iter())
            .map(|n| {
                let now = tokio::time::Instant::now();
                (now, n, n * 2)
            })
            .map(|(start, n, n2)| async move {
                let elapsed = start.elapsed();

                // elapsed should all be small!
                println!("{n}: {elapsed:?}");

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                n2
            })
            .zip(futures::stream::iter(&mut v))
            .for_each_concurrent(None, |(n2, slot)| async move {
                *slot = n2.await;
            })
            .await;
        let full_elapsed = full_start.elapsed();
        println!("for_each_concurrent: {v:?}");
        println!("for_each_concurrent: {full_elapsed:?}");

        println!();

        // NOTE:
        //
        // `bruh![moment]`:
        //
        // > btw interestingly it depends on the order in which the subtasks are ready due to the implementation
        // > details of `FuturesOrdered`, you're kinda breaking it by sleeping in fold:
        //
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=21944c56fe9e89b57272d85d576dddd7

        println!("Warning: sleeping less than previous thread means a later task\n\
                  still waits for the previous tasks to finish");
        println!("---");
        println!("sleep less than previous thread, buffered fold:");
        let full_start = tokio::time::Instant::now();
        let _v = futures::stream::iter([0, 1, 2, 3, 4, 5].into_iter())
            .map(|n| {
                let now = tokio::time::Instant::now();
                (now, n, n * 2)
            })
            .map(|(start, n, n2)| async move {
                let elapsed = start.elapsed();

                // elapsed should all be small!
                println!("{n}: {elapsed:?}");

                tokio::time::sleep(tokio::time::Duration::from_millis(20 - n)).await;
                n2
            })
            .buffered(100)
            .fold(Vec::new(), |mut v, n2| async move {
                v.push(n2);
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                v
            })
            .await;

        let full_elapsed = full_start.elapsed();
        println!("buffered fold: {full_elapsed:?}");
        println!();

        println!("sleep less than previous thread, buffer_unordered fold:");
        let full_start = tokio::time::Instant::now();
        let _v = futures::stream::iter([0, 1, 2, 3, 4, 5].into_iter())
            .map(|n| {
                let now = tokio::time::Instant::now();
                (now, n, n * 2)
            })
            .map(|(start, n, n2)| async move {
                let elapsed = start.elapsed();

                // elapsed should all be small!
                println!("{n}: {elapsed:?}");

                tokio::time::sleep(tokio::time::Duration::from_millis(20 - n)).await;
                n2
            })
            .buffer_unordered(100)
            .fold(Vec::new(), |mut v, n2| async move {
                v.push(n2);
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                v
            })
            .await;

        let full_elapsed = full_start.elapsed();
        println!("buffer_unordered fold: {full_elapsed:?}");
        println!();

        // fn_graph

        let mut resources = Resources::new();
        resources.insert(0u32);
        resources.insert(0u64);
        let resources = &resources;

        let fn_graph = {
            let mut fn_graph_builder = FnGraphBuilder::new();
            let _node_ids = fn_graph_builder.add_fns([
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
                do_nothing.into_fn_res(),
            ]);
            fn_graph_builder.build()
        };

        let full_start = tokio::time::Instant::now();
        let _events = fn_graph
            .stream()
            .map(|f| async move {
                let event = f.call(resources);
                println!("called {}", event.fn_name);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                event
            })
            .buffer_unordered(15)
            .collect::<Vec<_>>()
            .await;
        let full_elapsed = full_start.elapsed();
        println!("fn_graph collect: {full_elapsed:?}");
    });
}

fn do_nothing(n: &u32) -> Event {
    let n_before = *n;
    let n_after = *n;

    Event {
        fn_name: "do_nothing",
        n_before,
        n_after,
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Event {
    fn_name: &'static str,
    n_before: u32,
    n_after: u32,
}
