use std::{collections::VecDeque, io::Read};

use crate::Aoc;

#[derive(Debug, PartialEq)]
struct File {
    id: usize,
    length: u8,
}

#[derive(Debug, PartialEq)]
struct Gap {
    length: u8,
}

#[derive(Debug, PartialEq)]
struct FileGap {
    id: usize,
    length: u32,
    gap_length: u32,
}

struct Data {
    files: Vec<File>,
    gaps: Vec<Gap>,
}

fn parse(buf: &mut dyn Read) -> Data {
    let bytes = buf.bytes();
    let mut files = Vec::new();
    let mut gaps = Vec::new();
    for (i, byte) in bytes.flatten().enumerate() {
        let length = match byte {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            b'\n' => break,
            _ => panic!("Byte not matched"),
        };

        if i % 2 == 0 {
            files.push(File { id: i / 2, length });
        } else {
            gaps.push(Gap { length })
        }
    }

    Data { files, gaps }
}

fn get_block_ids(files: Vec<File>, mut gaps: Vec<Gap>) -> impl Iterator<Item = usize> {
    let mut files = VecDeque::from(files);
    gaps.reverse();
    let final_length: usize = files.iter().map(|f| f.length as usize).sum();

    let mut file = files.pop_front().unwrap();
    let mut end_file = files.pop_back().unwrap();
    let mut gap = gaps.pop().unwrap();

    (0..final_length).map(move |_| {
        if file.length != 0 {
            file.length -= 1;
            file.id
        } else if gap.length != 0 {
            gap.length -= 1;
            if end_file.length == 0 {
                end_file = files.pop_back().unwrap();
            }
            end_file.length -= 1;
            end_file.id
        } else if files.len() != 0 {
            file = files.pop_front().unwrap();
            gap = gaps.pop().unwrap();
            file.length -= 1;
            file.id
        } else {
            end_file.length -= 1;
            end_file.id
        }
    })
}

fn part1(buf: &mut dyn Read) {
    let Data { files, gaps } = parse(buf);

    let result: usize = get_block_ids(files, gaps)
        .enumerate()
        .map(|(i, id)| i * id)
        .sum();

    println!("Part 1: {}", result);
}

fn get_block_ids_defrag(files: Vec<File>, gaps: Vec<Gap>) -> Vec<FileGap> {
    let mut file_gaps: Vec<_> = files
        .into_iter()
        .zip(gaps.into_iter().chain([Gap { length: 0 }].into_iter()))
        .map(|(file, gap)| FileGap {
            id: file.id,
            length: file.length as u32,
            gap_length: gap.length as u32,
        })
        .collect();

    let max_id = file_gaps.iter().map(|f| f.id).max().unwrap();

    // eprintln!("{:?}", &file_gaps);

    for id in (0..=max_id).rev() {
        let (file_index, moving_file) = file_gaps
            .iter()
            .enumerate()
            .find(|(_, f)| f.id == id)
            .unwrap();
        let place_after = file_gaps[0..file_index]
            .iter()
            .enumerate()
            .filter_map(|(i, fg)| (fg.gap_length >= moving_file.length).then_some(i))
            .next();
        if let Some(place_after) = place_after {
            let gap_length = file_gaps[place_after].gap_length;
            let new_gap_length = gap_length - moving_file.length;

            let mut file = file_gaps.remove(file_index);
            file_gaps[file_index - 1].gap_length += file.length + file.gap_length;
            file.gap_length = new_gap_length;
            file_gaps[place_after].gap_length = 0;
            file_gaps.insert(place_after + 1, file);
            // eprintln!("{:?}", &file_gaps);
        }
    }

    file_gaps
}

fn part2(buf: &mut dyn Read) {
    let Data { files, gaps } = parse(buf);

    let final_files_and_gaps = get_block_ids_defrag(files, gaps);

    let mut offset = 0;
    let result = final_files_and_gaps
        .into_iter()
        .map(|file_gap| {
            dbg!(&file_gap);
            let value = sum_between(offset, offset + file_gap.length as usize - 1) * file_gap.id;
            offset += (file_gap.length + file_gap.gap_length) as usize;
            value
        })
        .sum::<usize>();

    println!("Part 2: {}", result);
}

fn sum_to(n: usize) -> usize {
    (n) * (n + 1) / 2
}

fn sum_between(a: usize, b: usize) -> usize {
    if a == 0 {
        sum_to(b)
    } else {
        sum_to(b) - sum_to(a - 1)
    }
}

inventory::submit!(Aoc::new(
    2024,
    9,
    part1,
    part2,
    include_bytes!("./inputs/day09")
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_block_ids_a() {
        let files: Vec<_> = [1, 3, 5]
            .into_iter()
            .enumerate()
            .map(|(i, length)| File { id: i, length })
            .collect();
        let gaps: Vec<_> = [2, 4].into_iter().map(|length| Gap { length }).collect();

        let ids: Vec<_> = get_block_ids(files, gaps).collect();
        assert_eq!(ids, vec![0, 2, 2, 1, 1, 1, 2, 2, 2])
    }

    #[test]
    fn test_get_block_ids_b() {
        let files: Vec<_> = [2, 3, 1, 3, 2, 4, 4, 3, 4, 2]
            .into_iter()
            .enumerate()
            .map(|(i, length)| File { id: i, length })
            .collect();
        let gaps: Vec<_> = [3, 3, 3, 1, 1, 1, 1, 1, 0]
            .into_iter()
            .map(|length| Gap { length })
            .collect();

        let ids: Vec<_> = get_block_ids(files, gaps).collect();
        assert_eq!(
            ids,
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6
            ]
        );
        assert_eq!(
            ids.iter().enumerate().map(|(i, id)| i * id).sum::<usize>(),
            1928
        );
    }

    #[test]
    fn test_get_block_ids_b_defrag() {
        let files: Vec<_> = [2, 3, 1, 3, 2, 4, 4, 3, 4, 2]
            .into_iter()
            .enumerate()
            .map(|(i, length)| File { id: i, length })
            .collect();
        let gaps: Vec<_> = [3, 3, 3, 1, 1, 1, 1, 1, 0]
            .into_iter()
            .map(|length| Gap { length })
            .collect();

        let file_and_gaps: Vec<_> = get_block_ids_defrag(files, gaps);
        assert_eq!(
            file_and_gaps,
            vec![
                FileGap {
                    id: 0,
                    length: 2,
                    gap_length: 0
                },
                FileGap {
                    id: 9,
                    length: 2,
                    gap_length: 0
                },
                FileGap {
                    id: 2,
                    length: 1,
                    gap_length: 0
                },
                FileGap {
                    id: 1,
                    length: 3,
                    gap_length: 0
                },
                FileGap {
                    id: 7,
                    length: 3,
                    gap_length: 1
                },
                FileGap {
                    id: 4,
                    length: 2,
                    gap_length: 1
                },
                FileGap {
                    id: 3,
                    length: 3,
                    gap_length: 4
                },
                FileGap {
                    id: 5,
                    length: 4,
                    gap_length: 1
                },
                FileGap {
                    id: 6,
                    length: 4,
                    gap_length: 5
                },
                FileGap {
                    id: 8,
                    length: 4,
                    gap_length: 2
                },
            ]
        );

        let mut offset = 0;
        assert_eq!(
            file_and_gaps
                .into_iter()
                .map(|file_gap| {
                    dbg!(
                        offset,
                        offset + file_gap.length as usize - 1,
                        sum_between(offset, offset + file_gap.length as usize - 1)
                    );
                    let value =
                        sum_between(offset, offset + file_gap.length as usize - 1) * file_gap.id;
                    offset += (file_gap.length + file_gap.gap_length) as usize;
                    value
                })
                .sum::<usize>(),
            2858
        )
    }
}
