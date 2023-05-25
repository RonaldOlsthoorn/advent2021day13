use std::{io::{BufReader, BufRead}, fs::File};
use std::fmt::Debug;
use ndarray::{Array2, Axis, Slice, Zip};


struct Point {
    x: u16,
    y: u16
}

struct InstructionSheet {
    instructions: Array2<bool>
}

impl Debug for InstructionSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for row in self.instructions.rows() {
            let line = row.iter().fold("".to_string(),
            |mut res, e| {if *e {res.push('#');} else {res.push('.');} return res;});
            writeln!(f, "{}", line)?;
        }
        return Ok(());
    }
}

impl InstructionSheet {

    fn fold_y(&self, at: usize) -> Self {

        let mut upper_half = self.instructions
            .slice_axis(Axis(0), Slice::from(0..at)).to_owned();

        let lower_half = &self.instructions.slice_axis(
            Axis(0), Slice::new(at as isize + 1, None, -1));        

        Zip::from(&mut upper_half.slice_axis_mut(Axis(0), Slice::from((upper_half.nrows() - lower_half.nrows())..at)))
            .and(lower_half)
            .for_each(|upper, &lower| *upper |= lower);

        return Self{instructions: upper_half};
    }

    fn fold_x(&self, at: usize) -> Self {

        let mut left_half = self.instructions
            .slice_axis(Axis(1), Slice::from(0..at)).to_owned();

        let right_half = &self.instructions.slice_axis(
            Axis(1), Slice::new(at as isize + 1, None, -1));

        Zip::from(&mut left_half.slice_axis_mut(Axis(1), Slice::from((left_half.ncols() - right_half.ncols())..at)))
            .and(right_half)
            .for_each(|left, &right| *left |= right);

        return Self{instructions: left_half};
    }
}

fn main() {

    let mut lines = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap());

    let mut points: Vec<Point> = vec![];

    let mut max_x = 0;
    let mut max_y = 0;

    let mut line = lines.next().unwrap();

    while line != "" {
        let mut splits = line.split(',');

        let x = splits.next().unwrap().parse::<u16>().unwrap();
        let y = splits.next().unwrap().parse::<u16>().unwrap();
        
        max_x = std::cmp::max(max_x, x);
        max_y = std::cmp::max(max_y, y);

        points.push(Point{x, y});
        line = lines.next().unwrap();
    }

    let mut field: Array2<bool> = Array2::<bool>::default((max_y as usize + 1, max_x as usize + 1));
    points.iter().for_each(|p| field[[p.y as usize, p.x as usize]] = true);

    let original_instruction_sheet = InstructionSheet{instructions: field};
    let mut fold = original_instruction_sheet;

    for instruction in lines {

        println!("fold {:?}", fold);

        println!("number of dots {:?}", fold.instructions.iter().fold(0, |count, &e| count + (e as usize)));

        if instruction.contains("fold along x=") {
            println!("dim: {:?} fold along x={}", fold.instructions.dim(), instruction.split_at(13).1.parse::<usize>().unwrap() - 1);
            fold = fold.fold_x(instruction.split_at(13).1.parse::<usize>().unwrap());
        }
        else {
            println!("dim: {:?} fold along y={}", fold.instructions.dim(), instruction.split_at(13).1.parse::<usize>().unwrap() - 1);
            fold = fold.fold_y(instruction.split_at(13).1.parse::<usize>().unwrap());
        }
    }
    println!("fold {:?}", fold);

    println!("number of dots {:?}", fold.instructions.iter().fold(0, |count, &e| count + (e as usize)));
}
