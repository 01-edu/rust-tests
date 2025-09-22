use rot21::*;

#[test]
fn test_rot21_multiple_cases() {
    assert_eq!("ocdn dn v ozno", rot21("this is a test"));
    assert_eq!("mviyjh ndhkgz rjmyn", rot21("random simple words"));
    assert_eq!(
        "ojj  hpxc    nkvxzn      rjmfn",
        rot21("too  much    spaces      works")
    );
    assert_eq!("mv😋w", rot21("ra😋b"));
    assert_eq!("12Â nãj ábpv", rot21("12Â são água"));

    assert_eq!("VWXY", rot21("ABCD"));
    assert_eq!("GJJFDIB BJJY", rot21("LOOKING GOOD"));
    assert_eq!("WTZ", rot21("BYE"));
}
