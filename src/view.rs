
use std::collections::VecDeque;
use std::ops::Range;

use ropey::Rope;

type LineNo = usize;
struct Line(LineNo, Vec<Range<usize>>);
struct Dims {
    width: usize, height: usize,
}

struct View {
    lines: VecDeque<Line>,
    dims: Dims,
}

impl View {

    pub fn from_rope(r: &Rope, dims: Dims) -> Self {

        let mut dq: VecDeque<Line>  = VecDeque::with_capacity(dims.height);
        for (ref i, ref line) in r.lines().enumerate() {
            let line_no: LineNo = i + 1;
            let ranges = get_ranges(line.len_chars(), dims.width);
            let l: Line = Line(line_no, ranges);
            dq.push_front(l);
        }
        View {
            lines: dq,
            dims: dims,
        }
    }
}

fn get_ranges(line_len: usize, width: usize) -> Vec<Range<usize>> {
    let mut ranges = Vec::with_capacity(line_occupies(line_len, width));
    if line_len <= width {
        return vec![(0..line_len)];
    } else {
        let mut n = line_len / width;
        if (line_len % width) != 0 {
            n += 1;
            for i in (0..n) {
                if i == (n - 1) {
                    let start = i * width; 
                    let end = (i * width) + (line_len % width);
                    let r = (start..end);
                    ranges.push(r);
                } else {
                    let start = i * width; 
                    let end = (i * width) + width;
                    let r = (start..end);
                    ranges.push(r);
                }
            }
            return ranges;
        } else {
            for i in (0..n) {
                let start = i * width; 
                let end = (i * width) + width;
                let r = (start..end);
                ranges.push(r);
            }
            return ranges;
        }
        return ranges;
    }
    return ranges;
}

/// Compute how many lines of the editor we require to display a line of text.
/// If greater than 1, that means we'll need to slice our line into N segments
/// for wrapping. We always return at least 1.
fn line_occupies(line_len: usize, editor_width: usize) -> usize {
    if line_len <= editor_width {
        1
    } else {
        let mut n = line_len / editor_width;
        if (line_len % editor_width) != 0 {
            n += 1;
        }
        n
    }
}


#[test]
fn test_view_from_rope() {

}
