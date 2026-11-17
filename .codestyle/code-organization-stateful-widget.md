:::eyebrow
---
eyebrow: Code Organization
---

# StatefulWidget

A widget that remembers things between rebuilds.
:::

## Overview

The [previous page](/docs/code-organization/stateless-widget) showed a widget that only turns its fields into other widgets. It is thrown away and made again whenever something around it changes, which is fine for a label but not for a counter, a hover flag or a half-typed form. Anything that has to survive a rebuild lives in a separate **state** object, and a `StatefulWidget` is a widget that comes with one.

The two halves have different jobs.

- **The widget** is only configuration: the values a parent hands down, such as a title or a starting number. It is cheap and disposable.
- **The state** is everything that changes while the widget is on screen. The framework keeps it alive for as long as the widget stays in the tree.

Reach for a stateful widget when a piece of UI has something to remember: a count, a flag, a controller, an animation, a scroll position. If it only turns its fields into widgets, stay with a [StatelessWidget](/docs/code-organization/stateless-widget). It is simpler, and a stateful widget is only worth it for the memory.

## How it lives

<svg viewBox="0 0 560 226" role="img" aria-label="The life of a stateful widget: create builds the state from the widget, init_state does optional one-time setup, and build draws the UI. Calling set_state runs build again. When the parent rebuilds, adopt_config_from copies the new props into the live state, and build runs again." style="display:block;width:100%;max-width:560px;margin:20px auto;color:var(--ex-heading);font-family:inherit">
  <g fill="#e0589e" fill-opacity="0.22" stroke="#e0589e" stroke-width="1.5">
    <rect x="10" y="46" width="150" height="56" rx="8"/>
    <rect x="205" y="46" width="150" height="56" rx="8"/>
    <rect x="400" y="46" width="150" height="56" rx="8" fill-opacity="0.45"/>
    <rect x="205" y="140" width="150" height="56" rx="8" fill-opacity="0.1"/>
    <rect x="400" y="140" width="150" height="56" rx="8" fill-opacity="0.1"/>
  </g>
  <g stroke="currentColor" stroke-opacity="0.7" stroke-width="1.3" fill="none">
    <path d="M162 74H203M197 69L204 74L197 79"/>
    <path d="M357 74H398M392 69L399 74L392 79"/>
    <path d="M445 46C445 12 505 12 505 46M499 40L505 47L511 40"/>
    <path d="M357 168H398M392 163L399 168L392 173"/>
    <path d="M475 138V104M470 111L475 103L480 111"/>
  </g>
  <g fill="currentColor" font-size="12.5" font-weight="600" text-anchor="middle">
    <text x="85" y="70">create</text>
    <text x="280" y="70">init_state</text>
    <text x="475" y="70">build</text>
    <text x="280" y="164">parent rebuilds</text>
    <text x="475" y="164">adopt_config_from</text>
  </g>
  <g fill="currentColor" font-size="11" fill-opacity="0.85" text-anchor="middle">
    <text x="85" y="88">builds it from the widget</text>
    <text x="280" y="88">optional, runs once</text>
    <text x="475" y="88">describes the UI</text>
    <text x="280" y="182">emits the widget again</text>
    <text x="475" y="182">takes the new props</text>
  </g>
  <text x="436" y="28" text-anchor="end" font-size="11.5" font-weight="600" fill="currentColor">set_state</text>
  <text x="280" y="220" text-anchor="middle" font-size="11.5" fill="currentColor" fill-opacity="0.8">the state lives as long as the widget stays in the tree</text>
</svg>

## Writing one

A stateful widget is three small pieces: the widget, its state, and the `impl State` block that joins them. This counter takes a starting number from its parent.

```rust
use aimer::*;

// 1. The widget: configuration only. Its state is `CounterState`, the name plus `State`.
#[derive(StatefulWidget)]
struct Counter {
    start: i32,
}

// 2. The state: everything that changes.
struct CounterState {
    count: i32,
}

// 3. Create the state from the widget, and build the UI from the state.
impl State<Counter> for CounterState {
    fn create(widget: Counter) -> Self {
        Self { count: widget.start }
    }

    fn build(this: &StateView<Self>, _: &BuildContext) -> impl Widget {
        Button::new()
            .on_press(this.callback(|state| state.count += 1))
            .child(Text::new(format!("Pressed {} times", this.count)))
    }
}
```

- **`#[derive(StatefulWidget)]`** writes the `Widget` impl for you, and pairs the widget with the state named after it plus `State`: `Counter` with `CounterState`. There is no `StatefulWidget` trait to implement by hand.
- **`#[state(MyState)]`** points the widget at any other state type. Add it under the derive and it replaces the default, so the state can have any name.
- **`create`** takes the widget by value, so move its fields into the state instead of cloning them. The widget is gone once it returns.
- **`build`** describes the UI from the state. It gets a `StateView` called `this`, which reads like the state: `this.count` is the field. It cannot change the state directly. Changes are queued with `this.set_state` or `this.callback`.
- **The state can be private.** Only the widget names it, so it does not need to be `pub`.

### Naming the state yourself

The default name is only a convention. When the state should be called something else, give its name to `#[state(...)]`. Any type that implements `State<Counter>` will do.

```rust
#[derive(StatefulWidget)]
#[state(CounterModel)]   // instead of the default, `CounterState`
struct Counter {
    start: i32,
}

struct CounterModel {
    count: i32,
}

impl State<Counter> for CounterModel {
    // create and build, as above
}
```

The state holds no updater of its own, so there is no line to remember that wires one up.

## Changing the state

`build` cannot change the state directly. A change is described as a closure and queued with `set_state`, and Aimer applies it at the next rebuild.

```rust
this.set_state(|state| state.count += 1);
```

Calling it several times in one frame costs a single rebuild, so there is no need to batch changes yourself.

Never call `this.set_state` unconditionally in `build`. It queues a change, which marks the state dirty again, so the widget would rebuild on every frame. Put it behind something that runs later, such as a handler.

### A callback

For a press handler, or any closure that should change the state each time it runs, use `callback`. It is shorthand for a closure that calls `set_state`, and it returns that closure for you.

```rust
Button::new()
    .on_press(this.callback(|state| state.count += 1))
    .child(Text::new("Add"))
```

This is the same as writing `.on_press(|| this.set_state(|state| state.count += 1))` by hand, one line shorter.

The change may run many times, so it is an `Fn` and not an `FnOnce`. It can use what it captured by reference, so a value from the loop you are building from goes in as it is.

```rust
let id = item.id.clone();

Button::new()
    .on_press(this.callback(move |state| state.items.retain(|item| item.id != id)))
    .child(Text::new("Remove"))
```

### After an `await`

A handler made with `on_press_async` runs its future on the UI thread, so it can call `set_state` after an `await`. That is the way to load something and then put the result in the state.

```rust
Button::new()
    .on_press_async(move || async move {
        let text = fetch_greeting().await;
        this.set_state(move |state| state.greeting = text);
    })
    .child(Text::new("Load"))
```

The widget may be gone by the time the future finishes, for example when the user has left the page. Check `this.mounted()` before you touch the state from anything that can outlive the widget, such as a timer, a request or a task.

```rust
spawn(async move {
    let data = fetch().await;
    if this.mounted() {
        this.set_state(move |state| state.data = Some(data));
    }
});
```

## Setting up

`create` is enough for most states. Two optional hooks cover the rest.

- **`init_state(&mut self)`** runs once, right after `create` and before the first build. It changes the state directly, with nothing to queue. Use it for setup that needs the finished state, such as starting a controller.
- **`on_mount(&mut self, updater)`** runs right after `init_state` and hands the state its `StateUpdater`. Use it only to give the updater to something that outlives the call and has to reach back into the state later, such as a back or link handler, a subscription or a repaint callback. A state that only changes itself from a button press uses `this` in `build`, and needs neither this hook nor a field.

```rust
fn on_mount(&mut self, updater: StateUpdater<Self>) {
    self.back_guard = Some(back::register(NavigatorBack { updater }));
}
```

Whatever you register here can run after the widget is gone, so prefer `try_set_state` and `try_read` inside it.

## When the parent changes the props

This is the part that surprises people. When a parent rebuilds and emits the widget again with new values, `create` runs again and makes a fresh state. The framework does **not** replace the live state with it, because that would throw away the count. It keeps the live one, and offers the fresh one to `adopt_config_from`.

So if a prop can change, copy it across.

```rust
#[derive(StatefulWidget)]
struct Greeting {
    name: String,
}

struct GreetingState {
    name: String,        // a prop, copied from the widget
    taps: u32,           // real state, which must survive
}

impl State<Greeting> for GreetingState {
    fn create(widget: Greeting) -> Self {
        Self { name: widget.name, taps: 0 }
    }

    // `new` is the fresh state made by the latest parent rebuild.
    fn adopt_config_from(&mut self, new: Self) {
        self.name = new.name;
    }

    fn build(this: &StateView<Self>, _: &BuildContext) -> impl Widget {
        Text::new(format!("Hello, {} ({} taps)", this.name, this.taps))
    }
}
```

Leave `adopt_config_from` out and the state keeps the first name it ever saw, even after the parent passes a different one. The method is optional because a widget with props that never change does not need it.

Use it for configuration only. The fields that you want to keep, such as `taps`, stay as they are.

## Keys

Keys came up on the previous page. They matter more here, because a parent that reorders or inserts siblings can end up pairing a state with the wrong widget. A **key** is a stable identity that keeps each state attached to the widget it belongs to. A named struct with a field `key: Option<Key>` has it forwarded for you.

```rust
#[derive(StatefulWidget)]
struct TodoRow {
    key: Option<Key>,
    title: String,
}

// A key made from a tag and the item's id.
TodoRow { key: Some(Key::from(("todo", item.id))), title: item.title.clone() }
```

- **Keys must be stable and unique.** Use the same key for the same item every time, and never two widgets with one key. A duplicate stops the app with a message that names the widget.
- **Prefer integers.** `Key::from(7)` and `Key::from(("todo", 7))` do not allocate. A key made from a `String` copies the text on every build.
- **`Key::unique()` never matches anything.** Use it when you want to throw the state away and start over.

## Keeping it small

- **Put in the state only what changes.** If a value can be worked out from other fields, work it out in `build`.
- **Keep `build` free of side effects.** It can run again on any frame that invalidates the widget.
- **Share, do not pass down.** When many screens need the same data, keep it in one store and provide it with `Provider`, as the [Core Concepts](/docs/core-concepts) page shows, and leave this state for what only one widget cares about.
- **Split it.** A state that grows long is usually two widgets. Move the part with its own memory into a smaller stateful widget.
