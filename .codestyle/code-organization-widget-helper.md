:::eyebrow
---
eyebrow: Code Organization
---

# Widget Helper

Keeping a long build readable by pulling parts of it into functions.

:::

## Overview

A `build` method describes a whole screen, and screens grow. Before long it is a tower of nested `Column`, `Container` and `Row`, where the part you want to change is somewhere in the middle and a missing bracket is hard to find.

You do not need a new kind of widget to fix that. A **widget helper** is an ordinary Rust function that returns a widget. You cut a piece out of `build`, give it a name, and call it. Nothing else about the widget changes.

<svg viewBox="0 0 560 232" role="img" aria-label="One long, deeply nested build on the left becomes a short build on the right that calls three small helpers, each of which holds one piece of the screen." style="display:block;width:100%;max-width:560px;margin:20px auto;color:var(--ex-heading);font-family:inherit">
  <text x="125" y="22" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">one long build</text>
  <rect x="10" y="32" width="230" height="176" rx="8" fill="#e0589e" fill-opacity="0.08" stroke="#e0589e" stroke-opacity="0.6" stroke-width="1.5"/>
  <g fill="#e0589e" fill-opacity="0.55">
    <rect x="24" y="46.0" width="120" height="5" rx="2.5"/>
    <rect x="34" y="56.4" width="150" height="5" rx="2.5"/>
    <rect x="44" y="66.8" width="140" height="5" rx="2.5"/>
    <rect x="54" y="77.2" width="120" height="5" rx="2.5"/>
    <rect x="64" y="87.6" width="100" height="5" rx="2.5"/>
    <rect x="54" y="98.0" width="110" height="5" rx="2.5"/>
    <rect x="64" y="108.4" width="90" height="5" rx="2.5"/>
    <rect x="54" y="118.8" width="120" height="5" rx="2.5"/>
    <rect x="44" y="129.2" width="130" height="5" rx="2.5"/>
    <rect x="34" y="139.6" width="110" height="5" rx="2.5"/>
    <rect x="44" y="150.0" width="100" height="5" rx="2.5"/>
    <rect x="54" y="160.4" width="90" height="5" rx="2.5"/>
    <rect x="44" y="170.8" width="110" height="5" rx="2.5"/>
    <rect x="34" y="181.2" width="120" height="5" rx="2.5"/>
    <rect x="24" y="191.6" width="80" height="5" rx="2.5"/>
  </g>
  <path d="M250 120H288M281 114L289 120L281 126" stroke="currentColor" stroke-opacity="0.7" stroke-width="1.4" fill="none"/>
  <text x="425" y="22" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">a short build, and helpers</text>
  <rect x="300" y="32" width="250" height="84" rx="8" fill="#e0589e" fill-opacity="0.3" stroke="#e0589e" stroke-width="1.5"/>
  <g fill="currentColor" font-size="11.5" font-family="ui-monospace, Menlo, monospace">
    <text x="316" y="54" font-family="inherit" font-weight="600" font-size="12">build</text>
    <text x="316" y="73">title_bar(..)</text>
    <text x="316" y="90">stats_row(..)</text>
    <text x="316" y="107">follow_button(..)</text>
  </g>
  <g stroke="currentColor" stroke-opacity="0.5" stroke-width="1.2" fill="none">
    <path d="M425 116V136M338 136H504M338 136V150M425 136V150M504 136V150"/>
  </g>
  <g fill="#e0589e" fill-opacity="0.16" stroke="#e0589e" stroke-opacity="0.8" stroke-width="1.2">
    <rect x="300" y="150" width="76" height="52" rx="6"/>
    <rect x="387" y="150" width="76" height="52" rx="6"/>
    <rect x="474" y="150" width="76" height="52" rx="6"/>
  </g>
  <g fill="#e0589e" fill-opacity="0.5">
    <rect x="310" y="176" width="48" height="5" rx="2.5"/><rect x="318" y="187" width="40" height="5" rx="2.5"/>
    <rect x="397" y="176" width="52" height="5" rx="2.5"/><rect x="405" y="187" width="38" height="5" rx="2.5"/>
    <rect x="484" y="176" width="46" height="5" rx="2.5"/><rect x="492" y="187" width="40" height="5" rx="2.5"/>
  </g>
  <g fill="currentColor" font-size="10.5" font-weight="600" text-anchor="middle">
    <text x="338" y="168">title_bar</text>
    <text x="425" y="168">stats_row</text>
    <text x="512" y="168">follow_button</text>
  </g>
  <text x="280" y="226" text-anchor="middle" font-size="11.5" fill="currentColor" fill-opacity="0.8">the same screen, read from the top as an outline</text>
</svg>

This is the lightest tool for organizing a build, and the right one to try first. When a helper needs more than a function can give, the next pages cover [StatelessWidget](/docs/code-organization/stateless-widget) and [StatefulWidget](/docs/code-organization/stateful-widget).

## Pulling a piece out

The examples on this page assume `use aimer::style::*;` and `use aimer::*;`.

Here is a screen with everything written inside `build`.

```rust
fn build(&self, _: &BuildContext) -> impl Widget {
    Column::new().children([
        Container::new()
            .padding(LayoutSpacing::all(16))
            .color(Color::Rgb(255, 158, 223))
            .child(Text::new(self.name.clone()).text_style(TextStyle::new().font_size(24)))
            .notype(),
        Row::new()
            .children([
                Column::new()
                    .children([Text::new("Followers").notype(), Text::new(self.followers.to_string()).notype()])
                    .notype(),
                Column::new()
                    .children([Text::new("Posts").notype(), Text::new(self.posts.to_string()).notype()])
                    .notype(),
            ])
            .notype(),
        Button::new()
            .on_press(|| println!("follow"))
            .child(Text::new("Follow"))
            .notype(),
    ])
}
```

Each part of it can be a function. A function that returns `impl Widget` needs no struct, no derive and no trait.

```rust
fn build(&self, _: &BuildContext) -> impl Widget {
    Column::new().children([
        title_bar(&self.name).notype(),
        stats_row(self.followers, self.posts).notype(),
        follow_button(|| println!("follow")).notype(),
    ])
}

fn title_bar(name: &str) -> impl Widget {
    Container::new()
        .padding(LayoutSpacing::all(16))
        .color(Color::Rgb(255, 158, 223))
        .child(Text::new(name.to_string()).text_style(TextStyle::new().font_size(24)))
}

fn stats_row(followers: u32, posts: u32) -> impl Widget {
    Row::new().children([
        stat("Followers", followers).notype(),
        stat("Posts", posts).notype(),
    ])
}

fn stat(label: &str, value: u32) -> impl Widget {
    Column::new().children([
        Text::new(label.to_string()).notype(),
        Text::new(value.to_string()).notype(),
    ])
}

fn follow_button(on_press: impl Fn() + 'static) -> impl Widget {
    Button::new().on_press(on_press).child(Text::new("Follow"))
}
```

`build` now reads as an outline of the screen, and each helper is short enough to see at once. `stat` was written twice in the long version, and is written once here.

### Taking what they need

A helper gets everything through its arguments. It has no `self` to reach into, which is a good thing: the signature tells you exactly what the piece shows.

- **Data goes in as values or references.** Copy what the widget keeps, such as `name.to_string()`, so the result does not borrow from the caller.
- **Behaviour goes in as a closure.** Take `impl Fn() + 'static` and pass it to `on_press`, as `follow_button` does, so the caller decides what a press does.
- **The context goes in as a parameter.** A helper that reads the theme or the navigator takes `ctx: &BuildContext` as its first argument.

```rust
fn price_tag(ctx: &BuildContext, price: f64) -> impl Widget {
    let theme = ThemeData::of(ctx);
    Text::new(format!("{price:.2}"))
        .text_style(TextStyle::new().color(theme.on_surface_color))
}
```

## Helpers on the state

A helper on a stateful widget can take the `StateView`, so it reads the state's fields and queues changes without a long list of arguments.

```rust
impl CounterState {
    fn controls(this: &StateView<Self>) -> impl Widget {
        Row::new().children([
            Button::new()
                .on_press(this.callback(|state| state.count -= 1))
                .child(Text::new("-"))
                .notype(),
            Button::new()
                .on_press(this.callback(|state| state.count += 1))
                .child(Text::new("+"))
                .notype(),
        ])
    }
}

// In build, where `this` is the `StateView`:
Column::new().children([
    Text::new(this.count.to_string()).notype(),
    Self::controls(this).notype(),
])
```

Each `callback` is its own closure, so a helper can make as many as it needs.

## One type, or many

A function that returns `impl Widget` returns one concrete type, and a function can only return one. That is enough for most helpers. When the branches of an `if` or a `match` produce different widgets, return `AnyWidget` and call `.notype()` on each branch.

```rust
fn status(loaded: bool) -> AnyWidget {
    if loaded {
        Text::new("Ready").notype()
    } else {
        Row::new().children([Text::new("Loading").notype()]).notype()
    }
}
```

A helper that returns a simple concrete type, such as `Text`, has a bonus: the caller can keep chaining.

```rust
fn heading(text: &str) -> Text {
    Text::new(text.to_string())
        .text_style(TextStyle::new().font_size(24).font_weight(FontWeight::Bold))
}

heading("Settings").wrapped()
```

## One widget per item

A helper is the natural row for a list. Map it over your data.

```rust
fn todo_row(todo: &Todo) -> impl Widget {
    Row::new().children([
        Text::new(todo.title.clone()).notype(),
        Text::new(if todo.done { "done" } else { "open" }).notype(),
    ])
}

Column::new().children(self.todos.iter().map(todo_row))
```

Every call returns the same type, so no `.notype()` is needed. For a very long list, use `.list(items).builder(...)` instead, which builds the rows as they come into view.

## Sharing a look

Helpers are not only for widgets. A value that repeats, such as a spacing, a text style or a colour made from the theme, deserves a name too.

```rust
const GAP: i32 = 16;

fn caption_style() -> TextStyle {
    TextStyle::new().font_size(13)
}

fn muted(ctx: &BuildContext) -> Color {
    ThemeData::of(ctx).on_background_color.with_alpha(0.72)
}
```

Changing the look of every caption is then one edit. A colour made from the theme, like `muted`, also follows the theme when it changes. [Theme](/docs/user-interface/theme) covers that.

## Where to put them

Put the helpers of a screen in the same module as the screen, below it, and keep them private. Only the screen itself needs to be public.

```rust
// src/screens/profile.rs
pub fn profile_screen(name: &str, followers: u32, posts: u32) -> impl Widget {
    Column::new().children([
        title_bar(name).notype(),
        stats_row(followers, posts).notype(),
    ])
}

fn title_bar(name: &str) -> impl Widget { /* ... */ }
fn stats_row(followers: u32, posts: u32) -> impl Widget { /* ... */ }
```

When a helper is used by several screens, move it to a module of shared pieces, such as `src/widgets.rs`, and make it `pub`.

## When a helper is not enough

A helper is part of the `build` that calls it. It has no identity of its own, and that is its limit. These are the signs that it should become a [StatelessWidget](/docs/code-organization/stateless-widget).

| | A helper function | A `StatelessWidget` |
| --- | --- | --- |
| Written as | A function | A struct, a derive and a `build` |
| Inputs | Arguments | Fields |
| Rebuilds | With the widget that calls it. Anything it reads from `ctx` rebuilds that whole widget. | By itself, when something it read changes |
| A key | No | Yes, with a `key` field |
| Reuse | Call the function | Use the struct, like any widget |

Move to a struct when:

- **It has many inputs.** A function with six arguments is a struct with six fields waiting to be written.
- **It reads the context and the parent is large.** As a widget, only the piece rebuilds when the theme changes, and not the whole screen.
- **It needs a key.** For example a row in a reorderable list.

If the piece has to remember something between builds, it needs more than a helper: a [StatefulWidget](/docs/code-organization/stateful-widget).
