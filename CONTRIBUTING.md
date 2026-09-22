**Style guidelines**

* Listen to rustfmt mostly, with the following exceptions:
  * Sometimes, particularly when dealing with lots of rendering coordinates, it may be beneficial to slightly deviate from rustfmt and instead format things by hand. An example of this can be found in src/cursor.rs:

    ```rust
    // …
    if state.anim_phase == AnimPhase::NoFocus {
        #[rustfmt::skip]
        {
            let bg = hl.cursor_bg().to_rgbo(filled_alpha);
            snapshot.append_color(&bg, &Rect::new(          x,           y,   w, 1.0));
            snapshot.append_color(&bg, &Rect::new(          x,           y, 1.0,   h));
            snapshot.append_color(&bg, &Rect::new(          x, y + h - 1.0,   w, 1.0));
            snapshot.append_color(&bg, &Rect::new(x + w - 1.0,           y, 1.0,   h));
        };
        false
    } else {
    // …
    ```

    There's a lot of variables and arithematic happening here, so it's much easier to read if we prevent rustfmt from mangling things. Large tables of static data very often fall into this category.
  * `gtk::*Builder` usage patterns. We use these all over the place, and the majority of the time rustfmt is going to split the method chains here onto separate lines. As a result, single-line invocations of Builder types tend to look out of place and are less visually intuitive. So, feel free to have rustfmt skip these whenever it tries combining these method chains onto a single line. If it's the only Builder invocation in it's scope and it's really, seriously short though, feel free to use your best judgement.

* They're guidelines: use your best judgement and don't be afraid of making the wrong decision, if a piece of code seems like it'd be much more legible without rustfmt mangling it - feel free to throw a `[rustfmt::skip]` onto it. If a maintainer disagrees, they'll just let you know and the worst thing you'll have to do is change it ♥.

vim: tw=100 ts=2 sts=2 sw=2 expandtab
