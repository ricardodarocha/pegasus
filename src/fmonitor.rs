use std::time::Instant;

pub fn monitore<F>(flag: &str, func: F)
where
    F: FnOnce(),
{
    let inicio = Instant::now();
    func();
    let duracao = inicio.elapsed();
    println!("🕛 [{:?}] {}", duracao, flag);
}