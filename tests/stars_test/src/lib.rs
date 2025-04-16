use stars::*;

#[test]
fn test_stars() {
    assert_eq!(stars(0), "*");
    assert_eq!(stars(1), "**");
    assert_eq!(stars(2), "****");
    assert_eq!(stars(3), "********");
    assert_eq!(stars(4), "****************");
    assert_eq!(stars(5), "********************************");
    assert_eq!(
        stars(10),
        "****************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************"
    );
}
