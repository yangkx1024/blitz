//! In the auto table layout algorithm, a cell's (or its column's) length `width` is a
//! minimum: the column still grows to the min-content width of its cells, and does
//! not grow past the larger of the two to fit their max-content width.
//!
//! The engine made such a column a fixed track, so content wider than the width
//! overflowed into the next column, as in GitHub's notification mail
//! (`<th style="width: 24px">Status</th>`).

use blitz_test_harness::Harness;

/// A box of `width` pixels standing in for a word, so the test does not depend on fonts.
fn word(width: u32) -> String {
    format!("<span style='display:inline-block;width:{width}px;height:10px'></span>")
}

#[test]
fn a_cell_width_grows_to_fit_its_content() {
    let harness = Harness::from_html(&format!(
        "<table style='width:400px;border-spacing:0'><tr>\
         <td id=a style='width:24px;box-sizing:border-box;padding:0 0 0 16px'>{}</td>\
         <td id=b style='width:100%'>{}</td>\
         </tr></table>",
        word(50),
        word(10),
    ));

    let (a, b) = (harness.layout_rect("#a"), harness.layout_rect("#b"));
    assert_eq!(a.width, 66.0);
    assert_eq!(b.x, a.x + 66.0);
}

#[test]
fn a_cell_width_is_kept_when_its_content_is_narrower() {
    let harness = Harness::from_html(&format!(
        "<table style='width:400px;border-spacing:0'><tr>\
         <td id=a style='width:24px;padding:0'>{}</td>\
         <td id=b style='padding:0'>{}</td>\
         </tr></table>",
        word(10),
        word(10),
    ));

    assert_eq!(harness.layout_rect("#a").width, 24.0);
    assert_eq!(harness.layout_rect("#b").width, 376.0);
}

#[test]
fn a_cell_width_does_not_grow_to_fit_content_that_wraps() {
    let harness = Harness::from_html(&format!(
        "<table style='width:400px;border-spacing:0'><tr>\
         <td id=a width=100 style='padding:0'>{} {}</td>\
         <td id=b style='padding:0'>{}</td>\
         </tr></table>",
        word(60),
        word(60),
        word(10),
    ));

    assert_eq!(harness.layout_rect("#a").width, 100.0);
    assert_eq!(harness.layout_rect("#b").width, 300.0);
}

#[test]
fn a_column_width_is_a_minimum_for_cells_in_later_rows() {
    let harness = Harness::from_html(&format!(
        "<table style='width:400px;border-spacing:0'>\
         <tr><td style='width:24px;padding:0'></td><td style='padding:0'></td></tr>\
         <tr><td id=a style='padding:0'>{}</td><td id=b style='padding:0'>{}</td></tr>\
         </table>",
        word(50),
        word(10),
    ));

    let (a, b) = (harness.layout_rect("#a"), harness.layout_rect("#b"));
    assert_eq!(a.width, 50.0);
    assert_eq!(b.x, a.x + 50.0);
}
