#[cfg(test)]
mod tests {
    use inv_pyramid::*;

    #[test]
    fn it_works() {
        assert_eq!(inv_pyramid(String::from("#"), 0), [] as [&str; _]);
        assert_eq!(inv_pyramid(String::from("#"), 1), [" #"]);
        assert_eq!(inv_pyramid(String::from("a"), 2), [" a", "  aa", " a"]);
        assert_eq!(
            inv_pyramid(String::from(">"), 5),
            vec![
                " >",
                "  >>",
                "   >>>",
                "    >>>>",
                "     >>>>>",
                "    >>>>",
                "   >>>",
                "  >>",
                " >",
            ]
        );
        assert_eq!(
            inv_pyramid(String::from("&"), 8),
            [
                " &",
                "  &&",
                "   &&&",
                "    &&&&",
                "     &&&&&",
                "      &&&&&&",
                "       &&&&&&&",
                "        &&&&&&&&",
                "       &&&&&&&",
                "      &&&&&&",
                "     &&&&&",
                "    &&&&",
                "   &&&",
                "  &&",
                " &",
            ]
        );
    }
}
