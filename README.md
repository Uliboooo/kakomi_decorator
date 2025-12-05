## Examples

```rust: main.rs
use kakomi_decorator::{DecoType, Kakomi};

fn main() {
    let content = "Hello world".to_string().decorate(DecoType::Normal);
    let totsuzen_no_shi = "突然の死!".to_string().decorate(DecoType::TotsuzenNoShi);

    println!("{}\n{}", content, totsuzen_no_shi);
}

```

<img width="111" height="102" alt="Screenshot 2025-12-06 at 1 41 06" src="https://github.com/user-attachments/assets/49adbf17-16b0-4af0-bec1-74cd8ad422b8" />

