:::eyebrow
---
eyebrow: Code Organization
---

# Borrow over Clone

Sharing data between widgets without copying it.

:::

## Overview

A `build` method runs again and again, and the data it uses is often large: an article, a list of messages, a long piece of text. The easy way to hand that data to a widget is `.clone()`, and that is also the expensive way. Every clone of a `String` or a `Vec` allocates and copies all of it, on every build.

Most of the time a widget only needs to **read** the data. So the order of preference is:

- **Borrow it.** A `&str` or a `&T` costs nothing, and is the right choice for a helper function that only shows the data.
- **Share it.** When a widget has to **keep** the data, for a callback or a retained child, share one copy with a handle that is cheap to clone.
- **Clone it** only when you really need your own copy that you will change.

Aimer has three tools for sharing, all in `aimer::share`.

| Tool | What it is |
| --- | --- |
| `Shared<T>` | One value with many owners. Cloning it adds an owner and copies nothing. |
| `ShareRef<T>` | A read-only handle to a value, or to one part of a value |
| `RcProjectExt` | Adds the same handles to the `Rc<T>` you already have |

<svg viewBox="0 0 560 206" role="img" aria-label="Cloning a String makes a second copy of the data. Cloning a Shared handle makes a second handle to the same data and only changes a count." style="display:block;width:100%;max-width:560px;margin:20px auto;color:var(--ex-heading);font-family:inherit">
  <defs>
    <marker id="bc-head" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
      <path d="M1 1L9 5L1 9" fill="none" stroke="currentColor" stroke-opacity="0.75" stroke-width="1.6"/>
    </marker>
  </defs>
  <text x="140" y="22" text-anchor="middle" font-size="12.5" font-weight="600" fill="currentColor">value.clone()</text>
  <rect x="20" y="44" width="44" height="28" rx="6" fill="#e0589e" fill-opacity="0.18" stroke="#e0589e" stroke-width="1.5"/>
  <text x="42.0" y="62.5" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">a</text>
  <rect x="20" y="128" width="44" height="28" rx="6" fill="#e0589e" fill-opacity="0.18" stroke="#e0589e" stroke-width="1.5"/>
  <text x="42.0" y="146.5" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">b</text>
  <rect x="150" y="38" width="110" height="40" rx="6" fill="#e0589e" fill-opacity="0.3" stroke="#e0589e" stroke-width="1.5"/>
  <text x="205.0" y="62.5" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">String</text>
  <rect x="150" y="122" width="110" height="40" rx="6" fill="#e0589e" fill-opacity="0.14" stroke="#e0589e" stroke-width="1.5"/>
  <text x="205.0" y="146.5" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">String (copy)</text>
  <path d="M66 58L146 58" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#bc-head)"/>
  <path d="M66 142L146 142" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#bc-head)"/>
  <text x="140" y="190" text-anchor="middle" font-size="11.5" fill="currentColor" fill-opacity="0.85">every clone copies the whole value</text>
  <path d="M280 20V190" stroke="currentColor" stroke-opacity="0.2" stroke-width="1" stroke-dasharray="3 4"/>
  <text x="420" y="22" text-anchor="middle" font-size="12.5" font-weight="600" fill="currentColor">shared.clone()</text>
  <rect x="310" y="44" width="44" height="28" rx="6" fill="#e0589e" fill-opacity="0.18" stroke="#e0589e" stroke-width="1.5"/>
  <text x="332.0" y="62.5" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">a</text>
  <rect x="310" y="128" width="44" height="28" rx="6" fill="#e0589e" fill-opacity="0.18" stroke="#e0589e" stroke-width="1.5"/>
  <text x="332.0" y="146.5" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">b</text>
  <rect x="430" y="74" width="120" height="56" rx="6" fill="#e0589e" fill-opacity="0.35" stroke="#e0589e" stroke-width="1.5"/>
  <text x="490.0" y="106.5" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">String</text>
  <text x="490" y="118" text-anchor="middle" font-size="11" fill="currentColor" fill-opacity="0.9">count: 2</text>
  <path d="M356 58L426 88" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#bc-head)"/>
  <path d="M356 142L426 118" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#bc-head)"/>
  <text x="420" y="190" text-anchor="middle" font-size="11.5" fill="currentColor" fill-opacity="0.85">a clone is one more handle, and a count</text>
</svg>

## Why a clone costs

A `String` field is cheap to read and expensive to hand out. Look at what happens when a long text is passed to `Text`.

```rust
// Copies the whole body on every build, and then once more inside Text.
Text::new(self.body.clone()).wrapped()
```

`Text::new` accepts a `&'static str` for free, which is why a literal costs nothing. A `String` it has to turn into shared text, which is another copy. If the body lives in your state as a shared handle, none of that is needed.

```rust
use aimer::share::ShareRef;

struct ArticleState {
    body: ShareRef<str>,    // made once, when the article loads
}

// A clone of the handle is a count, and Text keeps the same text.
Text::new(self.body.clone()).wrapped()
```

`Text::new` takes a `ShareRef<str>` as it is, and a `ShareRef<String>` too.

## `Shared<T>`

`Shared<T>` is a reference-counted pointer, like `Rc`. Make one with `Shared::new`, and every `clone()` is one more owner of the same value.

```rust
use aimer::share::Shared;

let title = Shared::new(String::from("pineapple"));
let again = title.clone();                 // the text is not copied

assert_eq!(&*again, "pineapple");          // it reads like the value itself
assert_eq!(Shared::strong_count(&title), 2);
assert!(Shared::ptr_eq(&title, &again));   // both point at the same place
```

The value is **read-only** while it is shared, and it is dropped when the last owner goes.

### Text and lists

A `Shared` can hold a type whose size is not fixed, so a string or a list is shared as `str` or `[T]`, with no wrapper around it.

```rust
let name: Shared<str> = Shared::from("Aimer");
let names: Shared<[String]> = Shared::from(["Ann".to_string(), "Bo".to_string()]);
```

### Changing a shared value

A shared value cannot be changed in place, because others are reading it. There are three honest ways to deal with that.

```rust
let mut list = Shared::new(vec![1, 2, 3]);

// Alone? Then you may change it in place.
if let Some(items) = Shared::get_mut(&mut list) {
    items.push(4);
}

// Shared with others? Copy on write: this handle gets its own copy first.
let other = list.clone();
Shared::make_mut(&mut list).push(5);
assert_eq!(*other, vec![1, 2, 3, 4]);       // the other owner is untouched
```

`make_mut` copies only when somebody else holds the value, so a value that is not shared is changed in place for free. For state, the usual move is simply to replace the handle with `set_state`.

### Handles that do not own

`Shared::downgrade` makes a `Weak<T>`. It watches the value without keeping it alive, and `upgrade()` gives you an owner again, or `None` when the value is gone. Use it for a pointer back to a parent, because a cycle of owners is never freed.

```rust
let value = Shared::new(String::from("pineapple"));
let weak = Shared::downgrade(&value);

assert!(weak.upgrade().is_some());
drop(value);
assert!(weak.upgrade().is_none());
```

## `ShareRef<T>`

A `ShareRef<T>` is a read-only handle that a function can accept without knowing where the value came from. The value can be a whole `Shared`, a field inside one, or something with a static lifetime, and the handle looks the same. Cloning it clones the ownership, never the value.

```rust
fn show_title(title: ShareRef<str>) -> impl Widget {
    Text::new(title)
}
```

### Handing out one field

The useful part is **projection**. You give a function that picks a field, and you get a handle to only that field. It keeps the whole value alive, so the handle stays valid even after the original owner is gone.

<svg viewBox="0 0 560 216" role="img" aria-label="A Shared owner keeps a whole Article alive. A ShareRef handle points at only its title, and keeps the whole Article alive for as long as the handle exists." style="display:block;width:100%;max-width:560px;margin:20px auto;color:var(--ex-heading);font-family:inherit">
  <defs>
    <marker id="bc-head" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
      <path d="M1 1L9 5L1 9" fill="none" stroke="currentColor" stroke-opacity="0.75" stroke-width="1.6"/>
    </marker>
  </defs>
  <rect x="10" y="88" width="128" height="36" rx="6" fill="#e0589e" fill-opacity="0.3" stroke="#e0589e" stroke-width="1.5"/>
  <text x="74.0" y="110.5" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">Shared&lt;Article&gt;</text>
  <rect x="200" y="14" width="180" height="170" rx="8" fill="#e0589e" fill-opacity="0.07" stroke="#e0589e" stroke-opacity="0.7" stroke-width="1.5"/>
  <text x="212" y="32" font-size="11.5" font-weight="600" fill="currentColor">Article</text>
  <rect x="212" y="42" width="156" height="32" rx="5" fill="#e0589e" fill-opacity="0.5" stroke="#e0589e" stroke-width="1.5"/>
  <rect x="212" y="84" width="156" height="32" rx="5" fill="#e0589e" fill-opacity="0.12" stroke="#e0589e" stroke-opacity="0.5" stroke-width="1.2"/>
  <rect x="212" y="126" width="156" height="32" rx="5" fill="#e0589e" fill-opacity="0.12" stroke="#e0589e" stroke-opacity="0.5" stroke-width="1.2"/>
  <g fill="currentColor" font-size="12" font-weight="600" text-anchor="middle">
    <text x="290" y="63">title</text>
    <text x="290" y="105" fill-opacity="0.7">body</text>
    <text x="290" y="147" fill-opacity="0.7">likes</text>
  </g>
  <path d="M140 106L196 106" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#bc-head)"/>
  <rect x="430" y="44" width="120" height="36" rx="6" fill="#e0589e" fill-opacity="0.3" stroke="#e0589e" stroke-width="1.5"/>
  <text x="490.0" y="66.5" text-anchor="middle" font-size="12" font-weight="600" fill="currentColor">ShareRef&lt;str&gt;</text>
  <path d="M428 62L372 62" stroke="currentColor" stroke-opacity="0.65" stroke-width="1.3" fill="none" marker-end="url(#bc-head)"/>
  <path d="M490 82V176H384" stroke="currentColor" stroke-opacity="0.45" stroke-width="1.2" stroke-dasharray="4 4" fill="none" marker-end="url(#bc-head)"/>
  <g fill="currentColor" font-size="11.5">
    <text x="440" y="100" font-weight="600">shows only title</text>
    <text x="440" y="116" fill-opacity="0.85">and keeps the whole</text>
    <text x="440" y="132" fill-opacity="0.85">Article alive</text>
  </g>
  <text x="280" y="208" text-anchor="middle" font-size="11.5" fill="currentColor" fill-opacity="0.8">no field was copied</text>
</svg>

```rust
struct Article {
    title: String,
    body: String,
    likes: u32,
}

let article = Shared::new(Article {
    title: "Borrow over Clone".to_string(),
    body: "A long text...".to_string(),
    likes: 0,
});

let title = article.project(|a: &Article| a.title.as_str());
let body = article.project(|a: &Article| a.body.as_str());
drop(article);

assert_eq!(&*title, "Borrow over Clone");   // the Article is still alive
```

A widget that shows the title is given a `ShareRef<str>` for the title, so it cannot see the body, and it does not copy the title to get it. You can narrow a handle again with `.project(...)`, for example from a `ShareRef<String>` to a `ShareRef<str>`, and no text is copied.

### Where handles come from

| From | Use |
| --- | --- |
| A `Shared` | `shared.project(..)`, `shared.as_share_ref()`, or `shared.into_share_ref()` to move it in |
| An `Rc` | `ShareRef::from_rc(&rc)`, or the methods below |
| A `String` | `ShareRef::from(string)` gives a `ShareRef<str>`, made once |
| A string slice | `ShareRef::from_str_slice("text")` copies it once |
| A list | `ShareRef::from([1, 2, 3])` gives a `ShareRef<[i32]>` |
| A constant | `ShareRef::from_static(&VALUE)` stores the reference, with no allocation |

## The Rc extensions

You may already have an `Rc<T>`. Import `RcProjectExt` and it gains the same handles. The handle keeps an `Rc` clone, so it outlives the original, and the field you select is not cloned.

```rust
use aimer::share::RcProjectExt;
use std::rc::Rc;

let state = Rc::new(Article {
    title: "Borrow over Clone".to_string(),
    body: "A long text...".to_string(),
    likes: 0,
});

let title = state.project(|a| a.title.as_str());   // ShareRef<str>
let whole = state.as_share_ref();                  // ShareRef<Article>, clones the Rc
let moved = state.into_share_ref();                // ShareRef<Article>, moves the Rc in
```

| Method | What it does |
| --- | --- |
| `project(select)` | A handle to one part of the value |
| `as_share_ref()` | A handle to the whole value. The `Rc` is cloned, so the original stays usable. |
| `into_share_ref()` | The same, but the `Rc` is moved in, so no count is added |

The values you read from the theme or from a provider come as a `Snapshot`, and it has `project` too. A widget can be given one colour of the theme without being given the theme.

```rust
let theme = ThemeData::of(ctx);
let primary: ShareRef<Color> = theme.project(|t: &ThemeData| &t.primary_color);
```

## In callbacks

A closure given to a button must own everything it uses, so it cannot borrow from `self`. This is where code is tempted to clone the data. Move a handle in instead.

```rust
let items = self.items.clone();      // Shared<[String]>: a count, not a copy

Button::new()
    .on_press(move || println!("{} items", items.len()))
    .child(Text::new("Count"))
```

## A guide

| You hold | Do this |
| --- | --- |
| A value that a helper only shows | Take `&T`, `&str` or `&[T]` |
| A piece of text kept in state | `ShareRef<str>`, and give `Text::new` a clone of it |
| A list shown by several widgets | `Shared<[T]>`, or a `ShareRef<[T]>` to hand it out |
| One field of a bigger value | `.project(..)`, to give out only that field |
| A constant | A `&'static str`, or `ShareRef::from_static` |
| A value inside an `Rc` already | `RcProjectExt` |
| A value to change | `Shared::make_mut`, or replace the handle in `set_state` |

Two limits to remember. `Shared` and `ShareRef` stay on the UI thread, anyway if you want to use it on another thread, let's talk with compiler :)).
