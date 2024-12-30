use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};


pub fn monitore<F>(flag: &str, total_size: usize, mut func: F)
where
    F: FnMut(&mut ProgressBar),
{

    let mut pb = ProgressBar::new(total_size as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40}] {percent}% {msg}")
        .unwrap()
        // .progress_chars("■■□"));
        .progress_chars("▉▒"));

    let inicio = Instant::now();
    func(&mut pb);
    let duracao = inicio.elapsed();
    pb.finish_with_message(format!("🕛 [{:?}] {}", duracao, flag));
    pb.finish_and_clear();
}