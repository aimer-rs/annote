:::eyebrow
---
eyebrow: Code Organization
---

# StatelessWidget

A reusable piece of UI that only turns its fields into widgets.
:::

## Overview

A `StatelessWidget` remembers nothing. It holds the values its parent gave it, and a `build` method that turns them into other widgets. Show the same fields and it draws the same thing, which makes it the simplest and most common kind of widget.

It is how you keep a large screen readable. Instead of one `build` that is two hundred lines long, you split the screen into small named pieces, each with a few fields.

The [previous page](/docs/code-organization/widget-helper) showed how far plain functions can take you. A `StatelessWidget` is the next step, when a piece deserves its own type. Everything here, `build`, the context and keys, carries over to the next page. If a piece has to remember something between builds, such as a counter, a hover flag or an animation controller, it needs a [StatefulWidget](/docs/code-organization/stateful-widget), which is what the next page covers.

## Writing one

Put `#[derive(StatelessWidget)]` on a struct, and implement `build`.

```rust
use aimer::*;

#[derive(StatelessWidget)]
struct Greeting {
    name: String,
}

impl StatelessWidget for Greeting {
    fn build(&self, _: &BuildContext) -> impl Widget {
        Text::new(format!("Hello, {}", self.name))
    }
}

// Use it like any other widget.
Greeting { name: "Alice".to_string() }
```

- **The fields are the inputs.** Whatever a parent passes in lives in the struct, and `build` reads it through `&self`.
- **`build` returns `impl Widget`.** Return the widget itself, with no boxing. When different branches return different widgets, return `AnyWidget` and call `.notype()` on each, as the [Core Concepts](/docs/core-concepts) page shows.
- **The derive does the rest.** It writes the `Widget` implementation, so there is no `to_element` to write. Prefer it over implementing `Widget` by hand.

A small `new` function is the usual way to build one when it has several fields.

```rust
impl Greeting {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

Greeting::new("Alice")
```

## Splitting a screen

The best use of a stateless widget is to name the parts of a screen. Each piece gets only the data it shows, and the screen reads as a short outline.

<svg viewBox="0 0 560 218" role="img" aria-label="A screen split into small widgets: ProfileScreen is made of Header, StatsRow and Actions, and each of those is made of smaller widgets." style="display:block;width:100%;max-width:560px;margin:20px auto;color:var(--ex-heading);font-family:inherit">
  <g stroke="currentColor" stroke-opacity="0.5" stroke-width="1.2" fill="none">
    <path d="M280 46V62M90 62H470M90 62V84M280 62V84M470 62V84"/>
    <path d="M90 116V132M45 132H125M45 132V148M125 132V148"/>
    <path d="M280 116V132M217 132H343M217 132V148M280 132V148M343 132V148"/>
    <path d="M470 116V132M435 132H505M435 132V148M505 132V148"/>
  </g>
  <g fill="#e0589e" stroke="#e0589e" stroke-width="1.5">
    <rect x="200" y="12" width="160" height="34" rx="7" fill-opacity="0.45"/>
    <rect x="40" y="84" width="100" height="32" rx="7" fill-opacity="0.28"/>
    <rect x="230" y="84" width="100" height="32" rx="7" fill-opacity="0.28"/>
    <rect x="420" y="84" width="100" height="32" rx="7" fill-opacity="0.28"/>
    <g fill-opacity="0.12">
      <rect x="10" y="148" width="70" height="30" rx="6"/>
      <rect x="90" y="148" width="70" height="30" rx="6"/>
      <rect x="187" y="148" width="60" height="30" rx="6"/>
      <rect x="250" y="148" width="60" height="30" rx="6"/>
      <rect x="313" y="148" width="60" height="30" rx="6"/>
      <rect x="400" y="148" width="70" height="30" rx="6"/>
      <rect x="470" y="148" width="70" height="30" rx="6"/>
    </g>
  </g>
  <g fill="currentColor" text-anchor="middle" font-weight="600">
    <g font-size="12.5">
      <text x="280" y="34">ProfileScreen</text>
      <text x="90" y="105">Header</text>
      <text x="280" y="105">StatsRow</text>
      <text x="470" y="105">Actions</text>
    </g>
    <g font-size="11.5" font-weight="500">
      <text x="45" y="168">Avatar</text>
      <text x="125" y="168">Name</text>
      <text x="217" y="168">Stat</text>
      <text x="280" y="168">Stat</text>
      <text x="343" y="168">Stat</text>
      <text x="435" y="168">Button</text>
      <text x="505" y="168">Button</text>
    </g>
  </g>
  <text x="280" y="206" text-anchor="middle" font-size="11.5" fill="currentColor" fill-opacity="0.8">every box is a small widget with its own few fields</text>
</svg>

```rust
#[derive(StatelessWidget)]
struct ProfileScreen {
    name: String,
    followers: u32,
}

impl StatelessWidget for ProfileScreen {
    fn build(&self, _: &BuildContext) -> impl Widget {
        Column::new().children([
            Header { name: self.name.clone() }.notype(),
            StatsRow { followers: self.followers }.notype(),
            Actions {}.notype(),
        ])
    }
}

#[derive(StatelessWidget)]
struct StatsRow {
    followers: u32,
}

impl StatelessWidget for StatsRow {
    fn build(&self, _: &BuildContext) -> impl Widget {
        Row::new().children([
            Stat { label: "Followers", value: self.followers }.notype(),
            Stat { label: "Posts", value: 12 }.notype(),
        ])
    }
}
```

Each part can be read, tested and reused on its own. `Stat` is written once and used wherever a number needs a label.

## Reading from the context

The second parameter of `build` is the `BuildContext`, which is how a widget reads things that were provided above it: the theme, the navigator, a shared store.

```rust
use aimer::style::*;

impl StatelessWidget for PriceTag {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = ThemeData::of(ctx);

        Container::new()
            .color(theme.surface_color)
            .child(Text::new(format!("{:.2}", self.price))
                .text_style(TextStyle::new().color(theme.on_surface_color)))
    }
}
```

This is why `build` may run again without a parent rebuilding it. A widget that asked for the theme with `ThemeData::of(ctx)` is built again when the theme changes. [Theme](/docs/user-interface/theme) covers it in full. `Navigator::<AppRoute>::of(ctx)` gives the navigator, as [Navigation](/docs/user-interface/navigation) shows, and `ctx.select` reads one value from a shared store and rebuilds only when that value changes.

## Keeping build pure

`build` can run many times: after a parent rebuilds, when something it read from the context changes, or when the window is resized. The widget is kept, and `build` is called on it again. So treat it as a description and nothing more.

- **Work only from `self` and `ctx`.** The same fields and the same context should give the same widgets.
- **Keep it cheap.** It can run on every frame that invalidates the widget, so no heavy work, no reading a file, no sorting a big list on each build.
- **Leave side effects out.** Do not start a timer, open a subscription or send a request from `build`. Those belong in a [stateful widget](/docs/code-organization/stateful-widget), or in an `AsyncBuilder`.

## Keys

A **key** is a stable identity, such as an id. It lets Aimer match a widget with its earlier version across rebuilds, even when its siblings are reordered or a new one is inserted. A named struct with a field `key: Option<Key>` has it forwarded to its element.

```rust
#[derive(StatelessWidget)]
struct Tile {
    key: Option<Key>,
    label: String,
}
```

A stateless widget has no state of its own to lose, so a key matters less here. It still helps when something below the widget does have state, such as a text field in a list that is reordered. Keys matter most on stateful widgets, and the next page shows them in full.

## Choosing the lightest way

A widget does not have to be a struct. The list below goes from the lightest form to the heaviest. Pick the first one that does the job.

| You have | Use |
| --- | --- |
| A helper pulled out of a long `build`, with no use anywhere else | A function that returns `impl Widget` |
| Branches that return different widgets | A function that returns `AnyWidget` |
| A reusable piece with several inputs, or one you want to name and test | A `StatelessWidget` |
| A piece of UI that needs to remember something | A [StatefulWidget](/docs/code-organization/stateful-widget), on the next page |

A function is enough for a quick piece, and [Widget Helper](/docs/code-organization/widget-helper) shows how to use them well.

```rust
use aimer::style::TextStyle;

fn section_title(text: &str) -> impl Widget {
    Text::new(text.to_string()).text_style(TextStyle::new().font_size(20))
}
```

When it grows its own fields, or you want to use it in several places, make it a `StatelessWidget`. Nothing else changes, because they are used the same way.

## Next

A stateless widget is rebuilt from scratch each time, so it cannot keep a count or a flag. The next page, [StatefulWidget](/docs/code-organization/stateful-widget), adds a state object that survives rebuilds.
