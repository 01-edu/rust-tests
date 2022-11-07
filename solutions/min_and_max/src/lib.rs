pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let mut tmp_min = nb_1;
    let mut tmp_max = nb_1;

    if nb_2 < tmp_min {
        tmp_min = nb_2;
    }
    if nb_2 > tmp_max {
        tmp_max = nb_2;
    }
    if nb_3 < tmp_min {
        tmp_min = nb_3;
    }
    if nb_3 > tmp_max {
        tmp_max = nb_3;
    }

    (tmp_min, tmp_max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(min_and_max(-1, 2, 4), (-1, 4));
        assert_eq!(min_and_max(3, 2, 1), (1, 3));
        assert_eq!(min_and_max(-23, -43, 0), (-43, 0));
        assert_eq!(min_and_max(15, 15, 15), (15, 15));
    }
}
