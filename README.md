# rust-projects 🦀

yeah i'm learning rust. no i'm not okay.

---

## what's in here

| project | status | what it does |
|--------|--------|--------------|
| [kv-store](./kv-store) | ✅ shipped | a CLI key-value store that actually saves to disk. get, set, delete, list. yes it persists. yes i'm proud. |
| markdown-to-html | 🚧 in progress | turns `.md` files into HTML because apparently i hate myself |
| task-queue | 🔨 soon | a to-do list that reads/writes JSON. touching grass is also on the list |
| tcp-chat | 🔨 soon | a chat server. single client only. my social life irl too |

---

## why rust tho

because i wanted to actually understand what ownership means instead of just vibing with garbage collectors forever. the borrow checker has humbled me more than any person ever has.

---

## what i've actually learned so far

- ownership & borrowing (it clicked eventually i promise)
- `Option` and `Result` handling without just `.unwrap()`-ing everything
- `HashMap`, iterators, pattern matching
- file I/O + JSON serialization with `serde`
- that `String` and `&str` are not the same thing and that's fine

---

## running anything

```bash
cd <project-folder>
cargo run
```

that's it. rust does the rest.

---

## stack

- rust (obviously)
- `serde` + `serde_json` for JSON stuff
- vibes

---

*still figuring out lifetimes. we don't talk about lifetimes yet.*
