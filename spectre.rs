use std::time::Instant;

fn measure_access_time(password: &str, guess: char, index: usize) -> f64 {
    let start_time = Instant::now();
    
    // Speculative execution
    if password.chars().nth(index) == Some(guess) {
        let mut _temp = 0;
        for _ in 0..1000 {
            _temp += 1;
        }
    }
    
    let elapsed_time = start_time.elapsed();
    elapsed_time.as_secs_f64()
}

fn crack_password(password: &str) -> String {
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    let mut cracked_password = vec!['*'; password.len()];
    
    for (i, _) in password.chars().enumerate() {
        let mut max_time = 0.0;
        let mut max_char = '*';
        
        for c in characters.chars() {
            let mut total_time = 0.0;
            for _ in 0..100 {
                let access_time = measure_access_time(password, c, i);
                total_time += access_time;
            }
            let avg_time = total_time / 100.0;
            
            if avg_time > max_time {
                max_time = avg_time;
                max_char = c;
            }
        }
        
        cracked_password[i] = max_char;
    }
    
    cracked_password.iter().collect()
}

fn main() {
    let password = "hTQU?K4asW.xl,T\\Gm?vj48`}";
    println!("Original Password: {}", password);
    
    let cracked_password = crack_password(password);
    println!("Cracked Password: {}", cracked_password);
}
