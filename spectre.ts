function measureAccessTime(password: string, guess: string, index: number): number {
    const startTime = performance.now();
    
    // Speculative execution
    if (password[index] === guess) {
        let temp = 0;
        for (let i = 0; i < 1000; i++) {
            temp += 1;
        }
    }
    
    const endTime = performance.now();
    return endTime - startTime;
}

function crackPassword(password: string): string {
    const characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    const crackedPassword = new Array(password.length).fill('*');
    
    for (let i = 0; i < password.length; i++) {
        let maxTime = 0;
        let maxChar = '*';
        
        for (const char of characters) {
            let totalTime = 0;
            for (let j = 0; j < 100; j++) {
                const accessTime = measureAccessTime(password, char, i);
                totalTime += accessTime;
            }
            const avgTime = totalTime / 100;
            
            if (avgTime > maxTime) {
                maxTime = avgTime;
                maxChar = char;
            }
        }
        
        crackedPassword[i] = maxChar;
    }
    
    return crackedPassword.join('');
}

const password = "hTQU?K4asW.xl,T\\Gm?vj48`}";
console.log("Original Password:", password);

const crackedPassword = crackPassword(password);
console.log("Cracked Password:", crackedPassword);
