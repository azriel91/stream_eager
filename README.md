# Stream Eager

Test how to get concurrent / eager evaluation of stream functions.

```bash
cargo run
```

Example output:

```yaml
0: 50.678µs
1: 101.466887ms
2: 201.923654ms
3: 303.430559ms
4: 403.8804ms
5: 505.318849ms
fold: 605.834373ms

0: 10.408µs
1: 27.337µs
2: 34.8µs
3: 39.731µs
4: 46.453µs
5: 52.991µs
collect: 101.656898ms

called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
called do_nothing
fn_graph collect: 102.146443ms
```
