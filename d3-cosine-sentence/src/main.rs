use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("Provide two sentenace for a cosine similarity, using their characters.");
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut a = String::new();
    let mut b = String::new();

    print!("Sentence A: ");
    stdout.flush()?;
    stdin.read_line(&mut a)?;
    print!("Sentence B: ");
    stdout.flush()?;
    stdin.read_line(&mut b)?;

    let mut a: Vec<f64> = a.chars().map(u32::from).map(f64::from).collect();
    let mut b: Vec<f64> = b.chars().map(u32::from).map(f64::from).collect();

    if a.len() != b.len() {
        let mut extension = vec![0f64; b.len().abs_diff(a.len())];
        if a.len() < b.len() { &mut a } else { &mut b }.append(&mut extension);
    }

    let a_magnitude = a.iter().copied().map(|x| x * x).sum::<f64>().sqrt();
    a.iter_mut().for_each(|x| *x /= a_magnitude);

    let b_magnitude = b.iter().copied().map(|x| x * x).sum::<f64>().sqrt();
    b.iter_mut().for_each(|x| *x /= b_magnitude);

    let cosine = a
        .iter()
        .copied()
        .zip(b.iter().copied())
        .map(|(a, b)| a * b)
        .sum::<f64>();

    println!("Cosine similarity: {cosine:.4}");
    Ok(())
}
