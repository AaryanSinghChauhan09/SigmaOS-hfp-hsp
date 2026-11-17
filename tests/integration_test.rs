// Integration test for SigmaOS-hfp-hsp
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use hfp_hsp as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
