:::eyebrow
---
eyebrow: Code Organization
---

# Single Widget File

One widget, one file, and a place for everything around it.

:::

## Overview

A small app can live in `main.rs`. A real one cannot, and the way to keep it readable is a rule you can say in one sentence: **a widget gets a file of its own**. Its struct, its state, the constants and helpers that only it uses, and its tests all sit in that one file, and nothing else does.

That gives you a few things for free. You find a widget by its name. A change to it touches one file. And when you delete it, nothing is left behind.

This page has the rule, the layout of folders that goes with it, and what belongs where.

## One widget, one file

Name the file after the widget, in `snake_case`: `TopicCard` is in `topic_card.rs`. Write the file in a steady order, so that every widget file reads the same way.

```rust title="src/components/topic_card.rs"
// 1. Imports.
use crate::models::Topic;
use aimer::share::ShareRef;
use aimer::style::{LayoutSpacing, TextStyle};
use aimer::*;

// 2. Constants that only this widget uses.
const PADDING: i32 = 12;

// 3. The widget, and the one way to make it.
/// A topic as a card: its title and how many replies it has.
#[derive(Clone, StatelessWidget)]
pub struct TopicCard {
    topic: ShareRef<Topic>,
}

impl TopicCard {
    pub fn new(topic: ShareRef<Topic>) -> Self {
        Self { topic }
    }
}

// 4. What it builds.
impl StatelessWidget for TopicCard {
    fn build(&self, _: &BuildContext) -> impl Widget {
        let title = self.topic.clone().project(|topic| topic.title.as_str());

        Container::new()
            .padding(LayoutSpacing::all(PADDING))
            .child(Column::new().children([
                Text::new(title).notype(),
                reply_count(self.topic.replies).notype(),
            ]))
    }
}

// 5. Helpers, private to the file.
fn reply_count(replies: u32) -> impl Widget {
    Text::new(format!("{replies} replies")).text_style(TextStyle::new().font_size(13))
}

// 6. Tests.
#[cfg(test)]
mod tests {
    // ...
}
```

- **Only the widget is `pub`.** The fields stay private and `new` is the way in, so the rest of the app cannot depend on how the widget is built inside.
- **Helpers stay in the file.** A function that only this widget calls is a [widget helper](/docs/code-organization/widget-helper), and it sits below the widget as a private `fn`.
- **Fields are the inputs.** Take the data the widget shows, as a [shared handle](/docs/code-organization/borrow-over-clone) when it is large, and not the whole store.
- **Tests sit beside the code** in a `tests` module at the bottom, so the behaviour and its tests are read together.

### A widget with state

A [stateful widget](/docs/code-organization/stateful-widget) is still one widget, so it is still one file. The widget, its state and the `State` impl stay together, because you will never change one without the other.

```rust title="src/components/like_button.rs"
#[derive(StatefulWidget)]
pub struct LikeButton {
    likes: u32,
}

struct LikeButtonState {
    likes: u32,
    liked: bool,
}

impl State<LikeButton> for LikeButtonState {
    // ...
}
```

### Types that belong to one widget

Whatever exists only for the widget goes in its file: the state, a small enum of its looks, a struct of its options.

```rust
/// The looks a badge can have. Nothing else in the app needs to name it.
#[derive(Clone, Copy)]
pub enum BadgeKind {
    Info,
    Warning,
}
```

A type that two widgets need is not private to either, so it moves up to `models/`. The one real exception is a route enum, which the `Router` derive turns into a widget. It belongs in `router.rs`, with the other routes, and not in a file of its own.

## Where things go

```text title="src"
src/
├─ main.rs              the entry point, the list of modules, the app
├─ router.rs            the routes, and which screen each one opens
├─ app_shell.rs         the frame that stays while screens change
├─ theme.rs             the theme
├─ store.rs             state shared by the whole app
├─ api.rs               a small facade over api/
├─ api/                 talking to the network
├─ models.rs            a small facade over models/
├─ models/              plain data types, one per topic
├─ components.rs        a small facade over components/
├─ components/          reusable widgets, one per file
└─ screen.rs            a small facade over screen/
   screen/              one screen per route
```

| Where | What lives there |
| --- | --- |
| `main.rs` | The `mod` lines and `AimerApp`. As little else as possible. |
| `router.rs` | The [route enum](/docs/user-interface/navigation) and the `build` that maps each route to a screen |
| `app_shell.rs` | The [shell](/docs/user-interface/navigation) with its `Outlet`: the header, the sidebar, the frame |
| `theme.rs` | The [theme](/docs/user-interface/theme) type and its light and dark values |
| `store.rs` | The shared state that is [provided at the root](/docs/core-concepts), such as the signed-in user |
| `api/` | Functions that fetch and send data, and return models. No widgets. |
| `models/` | Plain structs and enums for your data. No widgets, and no knowledge of the screen. |
| `components/` | Widgets that more than one screen can use, one per file |
| `screen/` | One widget per route, which puts components together |

### Who may use whom

Keep the arrows pointing down, and the app stays easy to change.

<svg viewBox="0 0 560 262" role="img" aria-label="Which folders may use which. The wiring files use the screens, the screens use the components, and the components use the models and the theme. The store and the api sit beside them and use the models. Models use nothing above them." style="display:block;width:100%;max-width:560px;margin:20px auto;color:var(--ex-heading);font-family:inherit">
  <defs>
    <marker id="sw-head" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
      <path d="M1 1L9 5L1 9" fill="none" stroke="currentColor" stroke-opacity="0.75" stroke-width="1.6"/>
    </marker>
  </defs>
  <text x="150" y="12" text-anchor="middle" font-size="11" fill="currentColor" fill-opacity="0.8">uses, from the top down</text>
  <rect x="20" y="20" width="260" height="46" rx="8" fill="#e0589e" fill-opacity="0.12" stroke="#e0589e" stroke-width="1.5"/>
  <text x="34" y="39" font-size="12.5" font-weight="600" fill="currentColor">main.rs · router.rs · app_shell.rs</text>
  <text x="34" y="55" font-size="11" fill="currentColor" fill-opacity="0.85">wires the app together</text>
  <rect x="20" y="86" width="260" height="46" rx="8" fill="#e0589e" fill-opacity="0.22" stroke="#e0589e" stroke-width="1.5"/>
  <text x="34" y="105" font-size="12.5" font-weight="600" fill="currentColor">screen/</text>
  <text x="34" y="121" font-size="11" fill="currentColor" fill-opacity="0.85">one file per route</text>
  <rect x="20" y="152" width="260" height="46" rx="8" fill="#e0589e" fill-opacity="0.22" stroke="#e0589e" stroke-width="1.5"/>
  <text x="34" y="171" font-size="12.5" font-weight="600" fill="currentColor">components/</text>
  <text x="34" y="187" font-size="11" fill="currentColor" fill-opacity="0.85">reusable widgets, one per file</text>
  <rect x="20" y="218" width="260" height="36" rx="8" fill="#e0589e" fill-opacity="0.38" stroke="#e0589e" stroke-width="1.5"/>
  <text x="34" y="236" font-size="12.5" font-weight="600" fill="currentColor">models/ · theme.rs</text>
  <text x="34" y="253" font-size="11" fill="currentColor" fill-opacity="0.85">plain data, and the look</text>
  <path d="M150 66V84" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#sw-head)"/>
  <path d="M150 132V150" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#sw-head)"/>
  <path d="M150 198V216" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#sw-head)"/>
  <rect x="330" y="86" width="210" height="46" rx="8" fill="#e0589e" fill-opacity="0.18" stroke="#e0589e" stroke-width="1.5"/>
  <text x="344" y="105" font-size="12.5" font-weight="600" fill="currentColor">store.rs</text>
  <text x="344" y="121" font-size="11" fill="currentColor" fill-opacity="0.85">state shared by the whole app</text>
  <rect x="330" y="152" width="210" height="46" rx="8" fill="#e0589e" fill-opacity="0.18" stroke="#e0589e" stroke-width="1.5"/>
  <text x="344" y="171" font-size="12.5" font-weight="600" fill="currentColor">api/</text>
  <text x="344" y="187" font-size="11" fill="currentColor" fill-opacity="0.85">talks to the network</text>
  <path d="M435 132V150" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#sw-head)"/>
  <path d="M435 198V236H284" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#sw-head)"/>
  <path d="M282 109H326" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#sw-head)"/>
  <text x="435" y="76" text-anchor="middle" font-size="11" fill="currentColor" fill-opacity="0.8">read by screens and components</text>
</svg>

- **Models use nothing above them.** A model is plain data, so it can be tested, saved and sent without any widget in sight.
- **Components use models and the theme.** They take what they show through their fields.
- **Screens use components.** A screen decides what is on the page, and a component decides how one piece looks.
- **The store and the api sit beside them.** The api fetches into models, the store keeps what the whole app shares, and screens and components read the store.

This is a direction to aim for, not a law. A component that opens a window for a screen will import that screen. What matters is that the **models never import a widget**, and that nothing cycles.

## The facade file

A folder gets a small file next to it, with the same name, that lists what is inside.

```rust title="src/components.rs"
pub mod avatar;
pub mod page_title;
pub mod side_bar;
pub mod topic_card;
```

Keep it to the `mod` lines. A caller writes `use crate::components::topic_card::TopicCard;`, and the facade is the one place where you can see every component at a glance.

## When a widget outgrows its file

A widget file that keeps growing is hiding more than one job. Split it by responsibility, make the file next to the folder the widget itself, and put the parts in the folder.

```text title="src/components"
components/
├─ side_bar.rs          the SideBar widget, and the mod lines below
└─ side_bar/
   ├─ nav_list.rs       the list of links
   ├─ category_section.rs
   └─ user_menu.rs      the account button
```

```rust title="src/components/side_bar.rs"
mod category_section;
mod nav_list;
mod user_menu;

use self::nav_list::NavigationRow;
use self::user_menu::UserMenuButton;

pub struct SideBar { /* ... */ }
```

- **Name the parts by what they do.** `nav_list.rs` and `user_menu.rs`, not `part1.rs`.
- **Keep the parts private** with a plain `mod`, so only the side bar can use them. Make one `pub` only when something else really needs it.
- **Re-export what callers need.** When a screen is split, the file next to the folder can pass the public ones up, so callers do not need to know the split exists.

```rust title="src/screen/login.rs"
mod email_code;
mod password;

pub use email_code::EmailCodeScreen;
pub use password::PasswordScreen;
```

As a rule of thumb, a file that has grown to several hundred lines is worth looking at. The split is rarely hard once you see two jobs in it.

## Screens and components

The two folders look alike, and it is easy to put a widget in the wrong one. Ask who uses it.

| | A component | A screen |
| --- | --- | --- |
| Used by | More than one screen | One route |
| Gets its data from | Its fields | The store, the api and the route |
| Knows about | The data it shows | The page: what to load, what to show |
| Lives in | `components/` | `screen/` |

- **Start in the screen.** A widget that only one screen uses stays with that screen, in its folder, such as `screen/settings/widgets.rs`. Move it to `components/` when a second screen wants it.
- **A screen is mostly composition.** It reads what it needs, and arranges components. If a screen builds the details of a card itself, that card is a component waiting to be written.
- **A component does not load data.** It shows what it is given. The screen that owns the page decides when to fetch, and passes the result down.

## Data

- **Models are plain.** Derive what you need (`Clone`, `Debug`, `PartialEq`, and `Serialize` and `Deserialize` for data from a server), and keep widgets out of them.
- **The api returns models.** A function such as `Api::get::<TopicList>("latest")` fetches and parses, and a screen puts the result into the store.
- **The store holds what many screens share.** The signed-in user, the loaded lists, the settings. Anything only one widget cares about stays in that widget's [state](/docs/code-organization/stateful-widget).
- **Pass handles, not copies.** A card that shows a topic holds a `ShareRef<Topic>`, so building it never copies the topic. [Borrow over Clone](/docs/code-organization/borrow-over-clone) covers this.

## A checklist

Before you add a file, ask where it belongs.

- Is it one widget? Give it a file named after it.
- Does only one screen use it? Put it in that screen's folder.
- Is it plain data? It is a model, and it imports no widget.
- Does it fetch or send data? It is in `api/`.
- Is it getting long? Split it into a folder, and keep the facade small.
