use kakomi_decorator::{DecoType, Kakomi};

fn main() {
    let content = "Hello world".to_string().decorate(DecoType::Normal);
    let totsuzen_no_shi = "突然の死!".to_string().decorate(DecoType::TotsuzenNoShi);

    println!("{}\n{}", content, totsuzen_no_shi);
}
