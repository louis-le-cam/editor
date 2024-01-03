pub fn fuzzy_score(string1: &str, string2: &str) -> u32 {
    let mut score = 0;

    let string1_count = string1.chars().count();
    let string2_count = string2.chars().count();

    for l in 0..string1_count.min(string2_count) {
        for offset1 in 0..string1_count - l {
            for offset2 in 0..string2_count - l {
                let mut chars1 = string1.chars().skip(offset1).take(l + 1);
                let mut chars2 = string2.chars().skip(offset2).take(l + 1);

                loop {
                    let char1 = chars1.next();
                    let char2 = chars2.next();

                    if char1.is_none() && char2.is_none() {
                        score += l as u32 + 1;
                        break;
                    } else if char1 != char2 {
                        break;
                    }
                }
            }
        }
    }

    score
}
