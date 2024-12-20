use std::io::Read;

fn main() {
    let input = get_input().unwrap();
    let crossword = Crossword::parse_input(&input);
    println!("Total: {}", &crossword.get_crossed_xmases());
}

struct Crossword {
    board: Vec<Vec<char>>,
}

impl Crossword {
    fn parse_input(input: &str) -> Self {
        let mut rows = Vec::new();
        for line in input.lines() {
            let mut column = Vec::new();
            for c in line.chars() {
                column.push(c);
            }
            rows.push(column);
        }
        Crossword { board: rows }
    }

    fn get_crossed_xmases(&self) -> u64 {
        let mut total: u64 = 0;
        for (row_index, row) in self.board.iter().enumerate() {
            for (column_index, column) in row.iter().enumerate() {
                if *column == 'A' {
                    println!("Found A at ({}, {})", row_index, column_index);
                    total += self.check_a(row_index, column_index);
                }
            }
        }

        total
    }

    fn check_a(&self, x: usize, y: usize) -> u64 {
        let mut total: u64 = 0;
        // North is empty between M's
        if self.check_point(x, y, Direction::NorthWest, 'M')
            && self.check_point(x, y, Direction::NorthEast, 'M')
            && self.check_point(x, y, Direction::SouthEast, 'S')
            && self.check_point(x, y, Direction::SouthWest, 'S')
        {
            println!("\tFound North centered cross!");
            total += 1;
        }

        // East is empty between M's
        if self.check_point(x, y, Direction::NorthEast, 'M')
            && self.check_point(x, y, Direction::SouthEast, 'M')
            && self.check_point(x, y, Direction::NorthWest, 'S')
            && self.check_point(x, y, Direction::SouthWest, 'S')
        {
            println!("\tFound East centered cross!");
            total += 1;
        }

        // South is empty between M's
        if self.check_point(x, y, Direction::SouthEast, 'M')
            && self.check_point(x, y, Direction::SouthWest, 'M')
            && self.check_point(x, y, Direction::NorthWest, 'S')
            && self.check_point(x, y, Direction::NorthEast, 'S')
        {
            println!("\tFound South centered cross!");
            total += 1;
        }

        // West is empty between M's
        if self.check_point(x, y, Direction::SouthWest, 'M')
            && self.check_point(x, y, Direction::NorthWest, 'M')
            && self.check_point(x, y, Direction::NorthEast, 'S')
            && self.check_point(x, y, Direction::SouthEast, 'S')
        {
            println!("\tFound West centered cross!");
            total += 1;
        }

        total
    }

    fn move_point(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::North => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Direction::South => {
                if x == self.board.len() - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::East => {
                if y == self.board.first().unwrap().len() - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::West => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::NorthWest => {
                if let Some((new_x, new_y)) = self.move_point(x, y, Direction::North) {
                    self.move_point(new_x, new_y, Direction::West)
                } else {
                    None
                }
            }
            Direction::NorthEast => {
                if let Some((new_x, new_y)) = self.move_point(x, y, Direction::North) {
                    self.move_point(new_x, new_y, Direction::East)
                } else {
                    None
                }
            }
            Direction::SouthWest => {
                if let Some((new_x, new_y)) = self.move_point(x, y, Direction::South) {
                    self.move_point(new_x, new_y, Direction::West)
                } else {
                    None
                }
            }
            Direction::SouthEast => {
                if let Some((new_x, new_y)) = self.move_point(x, y, Direction::South) {
                    self.move_point(new_x, new_y, Direction::East)
                } else {
                    None
                }
            }
        }
    }

    fn check_point(&self, x: usize, y: usize, direction: Direction, letter: char) -> bool {
        if let Some((new_x, new_y)) = self.move_point(x, y, direction) {
            if let Some(board_letter) = self.get_letter(new_x, new_y) {
                return board_letter == letter;
            }
        }

        false
    }

    fn get_letter(&self, x: usize, y: usize) -> Option<char> {
        if let Some(columns) = self.board.get(x) {
            if let Some(letter) = columns.get(y) {
                return Some(*letter);
            }
        }

        None
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

fn get_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input)?;

    if input.is_empty() {
        input = DEFAULT_INPUT.to_string();
    }

    Ok(input)
}

const DEFAULT_INPUT: &str = "SAMMSMSXMMXXXXSSSMXMAXAMSSSSMSAXMXSSXXXSMMSMMXMASXSSSSSMSMMXMAXMSMMMASMSXSAMMXMMMSMXXSMMXSMXSXXXXMMMMXMASMMMSAMXXSSMMSMAMXMSSMXSSMMSAMSAMXMM
MASAAAMAMMAMSMMAAXMSSXMSAMAAXXMAMAMXMSMMMXSASXSMSMMAAAXMAMSAMXSMAASMXXASMSASMMSAMASXXMAMAMMAMXMMMMAASAMXAXASXSXSASAAAAXAXXMAAASXSXAMAXAMSASX
SAMXSMSMMMAMMAMSMMXAMAXMAMMMMSSSMXSAAAAAMAMXMAAAMAMMMMMXSAMXASAXSXMMAMXMASAMAXSASASXXSAMSSMASAMAASMMSASMMMSMAAXXMSXMXSMMXXMASMMAMMMSMMMXSMAM
MASAXAAASXMXXAMMAMXAMMXSAMASMXAAAASMMSSMMASAMSMMSSMAXXMXMAXXMAMMMSSXMSMSMMAMMMSAMXXMXMXMXAXAXASMXXMMSXMAAMAMMMMSXSMSAMAMMAXAXAXXMAXAXAAMXXMA
SXMASMSMSAASMSMSAMSXAXASASAMAMSMMMMAAMXXSASAMMMXXMMXMAXXSSMMSMXSAAXSXAAAXSXMXAMSMAMAAAMASXMSSMMMSMSAMASMXSASAAAMAMAMASAASXMSSSSXSXSSSXSAXMXS
SXMAMAXAMMMMAAMMAMMSSMAMXMAXMMAAASMMMSAXMASXMMXSXSAAMSAMXAAAMXMMMXSMXMMMMXAXMASXMSASMSAMAAAAMMXAAAMASAMMASXSMMMMAMMMXMMMXAAMAAMAXXAMAMMXSAMX
MAXSXXMAMXAMSMMSSMMAMMXMSSMMXSSMMMAXAMXMSXMAMSAMAMSXSAXSSSMMSAMASMMXMSASXMASMXSXMAMXAXXXMMMMMAMMXSXMMXSMXMASXMASMSSMSASXSXMMMMMMMXMMAMMAMMMM
SSMMAMSAMXMAXMMAXAMSSXMSAAASXMMASMSMMMMMXAMAMMAMAMMMMMXXAAMAXASASAXMASAMXMASXMMASMXMAMXMMSSSMXSMAXMAMXMMXSXMMSMSAAXASXMASXAAXAAAMXXSXSMMMAMA
XMAMAMSASXXSXAMXMXMASXMMMSMMASXXMAAAAAAXMXMAMSSMXSXAASXMSMMSSMMMSMXAMMSMAMASAMSAMXAMXMAMSAAAMAAMAMSAMAMMMSAMAMXMMMMXMAMXMSXMMSSSSMXMMMAXSASM
XSMMAXSAMXAXSMMSXMASXXXAAMASAMAXMMMSXSSSMMSAXAMXAMMSMXMAAXSXAASAMXMSMAAMXMASAMXXSSXMAXXSMMXMMSSMSMSASXSAAXAMASAMSXXMSMSAMXMXMAMXXMASASAMMASA
XMXXXXMSMMXMAMAMAMXXMMSMSMAMXSMSAMXXAAAXAAXMASXMXSAASMSSMSMSSMMMSMAAMSSSSMMMXSAMXMASASMXMSASMXAXMASXMASMMXAMASXSAAMXAMMXAMXSAXXMASAXXMASXMMM
XXASMXMAXAMASMMSAMAMMAXMAMAMXAXAAXAMMMMMMAXMXXASASXSSXAAXAAAMAAAMMSSXXXXXAXAAXAMXMXMAMMAXMAXASAMMASAAXMASXSMMMMMMSMSASXSXMASXMASXMASXSAMXMSX
SMMSAAMMMMAMXAMXMMSSMMMMXXMSMMAXAMXXXAXXMXMSMSAMAXMXMAMMMMMSSSMMSAAXXMXSXXMMMXSMAMSSSSSMMMMMMAMXMAXAMXXMAMXAAXMAAAASAMAMXMASAAXMAAMAAMASAAAX
XAXSXMMXAASMMMMMMAXXAXSSMSAAAXAXMXSXSXXSAMXAAMAMXMXAMXASXSAXMAMXMMMSXSAAASXXXAAMXSAAXMAXSASAXAXSMSSMMXMMMASMMXSAXMMMXMAMXMASXMAMSMMMSMMSMSSS
MSXSAMXSXMXXAAAAMMSMSMAAAMXSMSMSMAMAMASAXAMSSSXMAMMASMMAAMMSSSMXSSMSAMMXMAAAMSMMXMMSMSAMXASMSXSAAXAMMXSAMMAAAMMAXSAMXSXMASAXMASXMASXMAXXAAAX
AMASAMXMSMXSMMXXXAAAAMMMXMAMXMAAAAMAMMMMXSAMXMASAMAMXAAMXMAAMAXMAAAMXMXSSMMMMXAMMMAAAXXXMXMAXMXMMMMXMASMXXXMXXMAMSAMMSASAMXAMAAASXMASAMXMMXM
XMAMMMAAAMMSMSSSMSSXMXXXXMXSAMSMSMSXSASMXMASXSXMASXXSXMASXMSSXMASMMMXSXAAXXAXXAMAMAMAMMMMMMMMMAXAAXAMXXAASMSMMMMMXAMASMMASMXMASMMMMMMMAXSSSS
XSAMXSMXMSAMXAAAMAXAXSAMXXAMAMMAAAXAXASAASMMMMASASXAAXXMXMAMAXXMXMXSAXMMMMSXMMAMXMASAXMAAAAMXSAXMXSMSXMXMXAAASAMSSMMMSXMAMXAMMMXMAMMASMMMAAS
AMASXAAMSXMXMMSMMASXMMAMXMMSAMSSMSMXMXMXMMXAAMMMASMSMMMSAMAMSMSAMXAMMSSSSMXAMMSMMXMSASASMSMSAMASXMAAAMSMMMSMSMASAAAXXXXMASXXMAMMSSXSASAAMMMM
ASAMXMXMAASXSXMXMAXAASMMAXMXAXAMXXAXSAMXSXSSMSAMAMXMAAAMXMAXAAAASMASXAAAAMMXMAXXSAMMAMMAAAAMASAAASMMMMAAXAMXAMMMMMMMSXMAXSAMSASAAXAMASMMMSXS
MMXXSXMXMMMMXAXXMASXMMMSSSSSSSMASMSMSAMAXXXAMXMMAMMSSMSSMSSSMSMAXAAMMMMSMMSXMASASASXAXXMMMMSMMMSMMSSSSSSMSSSSSXASXSAXXAMXMAMSASXMMSMMMXMXAAA
MASMMASMXMAMSAMMMXXMAAASAAAAAXXAXXMASAMSSMMMAAMSXSXAAAMAMAAAAXXSSMXSASXMXXAXSASMSAMMMSMXMXXXAAAXXXAXXAAXAMXAMXMXSAMASMSXMMAMSXMAXMXXMMASXMMM
MAMASAMAASAMMAMXAMMSSMMMMMMMMMMSSMMMMXMMAAAXMAXMAAMSSMSAMMMMXMXAMMMSASASMMMAMASXMMMAMAMASAMSMMMSMMMMMMMMASMMMMSXMXMAMXXAMSAXXAXXMMMSAMXSAAMX
MSXXMMXSMMASXMMMXXXAAAMXSXMASXMXAXXSXMSXSSMSSMMSMMXMAASXSAASXMMSAAAMAMXMASXMSAMXMAXAXASXSAXAMSXAXAAXAAXMAXAXAAMXSAMASXSAMSMMSSMXMAASXMASXMMS
MMASXSAXXMAMXMASMXSMSSMAMASXXAXSXMMAMAAAMAMMAAAAXMXSMMMMMXXSAXAMMMSMXMMSMMMXMAXXXXSXSASASMMMMMSSSSSMMXSMAMSMMMSASASASAMMMSAAAMXAMMXXAMAXAMAS
MXAMAMASMMASMSASAMSAAXMAMMMMSMMSAAAMAMMMMAMSSMMXMSAMXMXXAMMSAMSSMXAMMSAAAAXASMMMSMAXMAMAMXSXXAAXXMAMSMSMSAMMMMMAMAMXSXXXASMMSASXSSMSSMSSMMAS
MMMSXMMMASXSAMAMXAMMMSSMMMAMMMASXMMXXXAXSMMXAAAASMMMAMXMMSAMAXAAMSMXAMSSSMMXMAAXAASAMXMMMMMMMMSSMXXMAAXAMAMAASMSMSMMXMMMXSAMMAXXAXAAXXXAAMXM
AAXSASXSAMMMMMAMMMAAAXMMASXSXMAMAXXSSMMMXAXSSMMXSAMSASASAMXSXMXSMXMASMMMAMXSSSMMSSMMMASAAXAAAXXAAASMMSMXMMMSXSAMXAASXAASAMXXMMMMXMMMSMSSMMMS
SSSXAMASXAAAXMSMXXSMSSXSAMAXMMMSSMAXMASASXMMMSSSMMMSASAMMSMMMXXMXMMMXAMSSMMXAAMAXAXXSASXSSMSSSMMMMXAAAXXXXAXAMMMMSSMSXMMAXMAMAAAAXAAAAAMASAA
MMMMSMXMMSSSMAXXSMXAAAMMMSSMMSMAMMXMAXMASAAAAASAAMXXMMMMXAAAXXXXAMXASAMAMAMMXMMSSMMMMASAXAXXMXAAMXSSMMXSAMXSMMXAAMAMXMSXSMASXSSXMSSMMMMSASMM
XAAMAAXXMAXAMXMAXAMMMSMMAAAMXAMASXSMMSMAMXSMMXXMMMSASXXSSSSMSMSSSMMMSXMXSAMMASXMMAMXAMMMMSMSAXSMSXXAASASMMASXMMMXSASXXAAXMMMAAAXXAMXXAAMAXAX
SSSXSSXMMMSXMSMMSSMMAXAMMSSMSMSMSAMSAAMAXAXMSMSSMXSAMSAAMAMAMAMXXMAXMXMASAXMAAAXSAMMXSASMMAMMMMSMXSSMMAMAXASAXXSAAXXXMMSMAAMXMSMMASXSMSSSSMM
MAMXMMXMXXXAMXAAAAAMXSXMMXMAAXMXMMXMSSXSSMMASAAXMAMAMMMMMSMAMMXMAMSAMXMMSAMMSMMMSASAXMMMAMSMAASASAMXMMSMMMASXMXMASMMASAMXSMSAMXAMAMMSAMXMAXM
MAMASAMMASXMMXMMSSMMXMAXXAMXMSMXMMAXMMAAXXMAMMMXMAMAMXMXSAMAXXSMMAMAMAXAMAMAAAXXSAMXMXASAMXSSMSAMMMSAAXAXAXMAMSSSMAXAMASXMXMASMMMMSMMXMAXMMM
SASXSAMSASAMXXXAXAMMSMMSSMSAAAMAASMSAMSMMXMAXASASMSMSAMSAMSSSMXAMAXAMMSSSMMMSSMAMXMSMSMSSSMAMAMXMAXXMMSSMSASXMAMAMSMMSAMXMASXMMXAMAMXMXSSMSS
SASXXAMSSSMMXAMMXMMMAAMAAAXMMMSSXSAMXMAXAMXXSASAMAAASAMXMXAAAAMXMAMMMAAAAAXAAAMXMAXAAAAMMMMAMXMXMXMMXMAAAXAMMMMSAMAAMMAMSMMXAAAXSXXSASXAAAAX
MAMMMSMMAXXAASXSAXASXSMSSMXMSXXMAXMMSSMXSAAXMMMXMSMMMAMXXMAXMMASMMAMXMMXXXMMSSMMXMSMSMXMASMMXMMAMASXMMSSMMMMAAXSAMMSMSMMSAMXSMMMMAASASMMMMMS
XSMSAMAMMMMMMMAXAXMSMAAAMXASAMMSSMMSXAAAXMMMSMAMXXMMSAMXMXSSSXXSAMSAMXMSXSAAAXAXAXAAAAMSASMSAAMXSAMMXAAAXAXMMSXXSMAXAMXAXAMAXAXAMMMMXXAXSAMX
MMAMMSMMXAXMXMXMXSXXMMMMXSMSASAAAAXSMMMMXXSAXMAXAXXAAXSASAAAXSMMAMAAXXAAASMMMSASMSMSMSMMASASXSMAMMSAMMSSMSXSAMXAXMXMASMXSMMXSMMMXMMMMMSMXASA
XXAMXXMASXSMAMASAAAMSMXXASASAMMSSMMMXSAXXMMSSSSMMMMMSXSASMMMMMXXAMSMMMMMXMXSXMAAMAXXMXAMXMMMSMMMSXAMMAMXAAXMXMMXMAXMAXMAMXMXXMASAXAAAAAASAMX
XSXSXMAMAAMMASASMMSMAAXMASAMXMMXAXXAAXXSSMAMMAXAAAAXAAMAMXSXMXMXSMMAXAMXMSAMXSMMSXMAXXAMXSXMMXAMXMMXSAMMSMSAMXSMAXXMXSXMMAMXXMAXMMSSSSXMAAMX
MMMMAMSAMXSAXSASXXMMSSMMASXMASXSASMMMSMMAMMSMAMMSMSSMSMAMXAAMMSAXASMMXXAXMASAXAXXAXASMSMAMAAAMSSMSXASMMAAAXXAMMAMXXAAMAMSAMXSMXMSAMXMAXMASMA
XAASMMMSXAMMMMXMASMXMMAXASAAXAAMAMMAAAXSXMXAMMSMAMMMAAXMMMXAMAMMMMMMXMSASXXMASAMSMMMXAAMMMMMMSXAASMMSMMSSMMXAAMAMSSMMSAMASXXMASAMMMAMAMMAMXM
MXMXMAMXMSAXAMXXAMAMXMMSASXMAMMMSMXMXSXMMMSMSAAXAMMMSMMSAMSMMSSMASAMAXAXMMMSAXMAAXAMMSMMSASAXAMMMMAXXAAAAAMASMMAXXAAASASAXMMSMMMMMSSSSSXAMAM
SSSXSASXAMMSMSMMSSMSXSAMXSMMXXMAXMMXSMXXSAMXMXSXMMSAMAASMXAXXAAXMXAXSSMXXXMMSSXXMMMSAXAAMAXASMAXMMAMSMMSSMSMMMXXMXSMMSXMMSXAAMAXSAMMAMXMMXAM
XAAASAXMXMAAAAAXMAXAAMSMAMAMXSMMMXAMXXAXMASXXAXASASAMMMMXXSSMSSMSMAMMAMASXSAAMMSMSXMXSMMMSSMMXMMSSMAXAAXMAAAASMXSAAXXXASAMMSSSXSMASMXMAMXSAS
MMMMMMMSAMSMSXSMSXMMXMXMXSSMMMMMAXMMMMSMSAMXSMMAMAMXMMSSSMAXXAXAXMSMXAMXXAMMXSAAXAAMAXAXXAAAXASMAAMSMMMSMSSSMSAAMMMXSMMMASAAMMMMXMMMASXXAAAX
AXAXAAAXAMXAMAAAXXSXMMXSAMAAMAMMXSAAXAAMXAXAXXMSMSMAXXAAAMMMXMMMMMMSSXMMMXMAMAXMSSXMAMMAMXSMMMAMSSMXXASXMAXXAMMMMXMXSAXSAMMXSAAXASASASAMSXSM
SMMXSSSSSMMAMMMSMMSASXAMASXMAMSAAXMMMSMMSMMMSAMXAASMMMMSMMXSMXAAAAAXXMASXMMASMSXXMAMSXXAMXMXXXSMMMASAMXXMXMMXMSXXXAASXMMXXSASXXXAMAMXSMAXAAA
MSMMXAMXAXMASXAAAXSAMMSSMMXSAAMMSMMXAXXMAAAXXMAMXMMMXSAAAXMXMMSSSMSSMSMMAMSASXMXSSMMAMSXMAXMAMXAXMAMAMSMMSMSAAXXSAMXSMSXMXMASAXMSMSAMXXAMSMS
SAMXMAMMXXXMMMSSXMMAMAAAXAXMAXXAAASMMSSSSSMXMSSXMXXAAMSSSMMAMXAMMAMXAAASMMMXSAAAMXMXAXSASMSMASMMMMMSSMAAAAASMMMAMXMAMASAMXMMMXAAAAXXMAMXXAXX
SXMMXMMMSMSXXAXMXSMMMMMAMXSXMMMMSMMAAAMMMAMXAAMMSMMMXMAMXASAMMAMMMXMSMXMAAXASMMXXAMSSXXAMXAXXSAXXXXAXSMSMMMMAXMASAAAMXMAMXMSAMMSMSMSMMSMSMSM
XSXMMSAMAAMAMSXSASASXSMSMMXAMSXAXASMMSSXSAMSMMSAAMAMXMSXSXMASXMMMSAMXMSMSSMMSXSMSXXAMXMSMSXSMSMMMSMMXXXAAXXSXMXMAXSASMSMMXXMASXAXMAMXAAXXXMA
AAMMMSASMMMMMAAMAXAMAXMAXSSMMAMASMMMAMMAMXMXAXMMSXMASAMASMSXXAAXAXMXAXMAAMAXMASMASMSSXMXAMSAMXAMAAASAMXSMMXMASXSMXXAMXAAXMXSAMXMMASMMMSXSASM
SMASASMMASAXMMSMSMXMMMMMMMAXSXXAMAAMAXXASMMMSMMAMAMAMMSAMAMMSMMMSSSSMSMMMSSMMAMMAMXAMAMMAMMMSSMMMXMMASAAMXAMXMAMXSMMMASXMSMMMSMSAAAXASAMSAMX
AXSMAMSAXXAMMMAAAMSMXMAXAMMMMSMSSSMSXSSMSASAMAMSMSMXMAMXMXMAMXAAXXAAAMASAMXMMSSMASMMSAMSMSXAAAXMMSMSAMMSMSMSMMAMXXAAAMMMAAXMXSAAMSMMMMAXMMMX
SXMMXMAMSSMSXSMSMSAMSXSMXSAAAAXMAXASXMAXSMMASXMXAXASMASMXSMMXMXXSMSMMXXSASXSAMAMASAXSAXAAXMMSSMSMAAMXSXMASAAMSMSSMSMXMAMSMMSAMXMXAASMSXMASMM
AAXXMXAXAAXSXMAMXXAMSAMAXXMMMMMMAMSMMSMMXXMAMMASAMAMXMMMAAAASMMXSAMXSXAMXMAXXSSMASXMSMXMSMXAXXAAMMXMXMMMMMXMXAMAMAAMMSSMAMMMASMXSMXMAAXSMMXA
SXMAASMMMMMMAMSMSXSAMAMXASXMSSSMSXMAAAAMAMMMSMAMAMAXMSAMXMMMAASAMXMASMSMSMMMXMAMASAAXXXSXMMSSMSMSAAMAMXSASXSSMMAMMMXXAMSMSASMMXAMXAMMMMMAMMS
XMAXMMAAAMSSMMMAMXMAMXMXMSAAAAXAXAMXXXMMAXAAAMXSXMSXXMASAXAXXXMASXMXSAAAAAASAMAMASXMMMMSAMXMMAXMMMXXASAMXSAMAXSASXSXMAXAXSMMAAMXXMXXXMAMAMAX
XAXMASXMMXMASXMAMXSSMAMAXSMMMSMMMXMASMSSSSSSMXAAMMXMAMAMXSSMSMMXMXAAMXMSMSMSASXSXXAAMMAXMMSSXAXAMAMSMMASAMAMAXSXMAASMXMAXMXMMSMSSSMSMSMSXSAM
XSMSASXSXASAMXSAMMAAMASMXSXMSAMXAXSXSAAAAAMXAMSSMXMASMMXASXASMSSXSAMXAXXXAASMMMAAXSMMMMMAAXMMMSXMAXAMSSMXSXMXXMMMMMMXMAXMASXXXAXAAAAAMMAXMAS
MMASASAXSMMMSASASAMMSAXMASMMSAMXSXMMMMMMMMMMSMAAMXMAXAAMSMMMMAAAMMMMSSSXSMXMXAMXMMXXSASAMXMXAXAASXXMXSAMXSMMMXMAXAAMXMAXMXMSAMXMSMMMSSMMMSAM
AMAMAMMMSMAMMASAMMXXMXMMMSMXMAMAMASAMSMSAMXAXMMMMAMXMMMXAASMMMMSXSAXAMMMSXAMMMSXAXXAMASMSSXMAMSMMXXSXMASASMMSAMSSSSSXSASAMXMMMMXASXXAAAAAMAS
SMAMAAXSXMAXMXMAMXMAMAXXXMASAAMXSAMXMAXMXXMXSXXXSASMSXXXSMMXAAAAXMXMAMSAMXXMASXMASMMMAMAAAMXSXMXSXMMASXMASAAMAMXAAXXAMXSMAAXAAXSASXSASMMXSAM
XSXMSSXMAMMXMXXAMAXAMXSMASASXSXXMMSMSMSMXMSMSMXMAXSAAMMMMAMSAMMSSSSMSMMASXMSMSAMAXASMXSMMSMAXASAAMMSXMASXMMMSSMSMMMMSMAMMSMSMSMMAMAMAMMXXMAS
MXMMMMMMMMSAMSSMSXSAXAAMXMXMAMMMSAAAAAAXMAXAXAASMAMAMAMAMXXAMXXMXAAAMMXMMMXAXSAMXSSMMAAMMXMXSSMXMSAAAMAMAAAXAMAMAMXMASASMAMXAXXMAMXMAMXMASAM
AMXAAMAAXASASAAXXAAAMSMMXMXMXMAXMSSSMXMSMSXAXSMSMAXMAMSASMXAASXMMMMMMAASASXMMSAMXMMXXSXAMASMMXAMXMAMSMAXSMMMMMAMAMMMAXMXMAAMMMMMSSMMASAMXXSS
SASMXMSXXASXMXMMMMMXMAASAMASMMMSMXMMXMAAAAMMMMAXMMMMAMSAMMMXMAAXXMAAMSMXASXMAXAMAAMSMMAMSASAAMXAMXMAXMMMXAXAASASMSXMSSMMSMSAMASXAAXSXMASAAAS
MMMAMXASMAMMXAMMSAMXMXXXASAXAAXAAAXAAMXMSMSXAXAXSMASXMMAMXASMSMMMSSMMAAMSMMMSMMSSSMSMAMMMASMMSMSSSMAMASAMXSSXSASAMXXAAMAMXMMSASMSMMMMXAMMMMM
SAMAMMASMAMXSASAMASMMSSSMMMSSMSSSMSSXSXXXASMSMSMASASAMSMMXSXMASAXMMMSMSMAAMAXAAXAMAMMMXXXXMXXXAAAXMASMMMSAMXXMMMAMXMSSMSXAMAMMSAAAMASMMMXMXS
MXXMXMAMXMAASXMAXSAMXAAMAXMAMXAXXAXXAMMMMMXAXAAMAMMSMMAMXXMAMXSMMMAMXXMASXMASMMMXMAMMSSMSSMXSMMMSMSMSAAXMMSSXMSSSMXMAMXMMXMASAMXSAMASAMXXSAM
SMASAMASXMMMXAMAMMMXMMSMMMMSSMXSMMMMSMAXXXASMSMSMSMSASMSMMSAMMSAMSXSSSSMXAMAMXXAMXSSXAAXAAAAMAXXMAAASMMMAAXXAMXAMAMMAMAXAMSASAMXMXMASXMMAMAS
AMASASASXSASXSMXSMMSSXXAXAXXXXMAMMAAXSXSXMAXAAXAAAASXMXMAMSASAMAMASXXAAASMMSSMMXSAASMSSMSSMMSAMXMXMMMAMSMMSMSMMAMSXSASXSSXMASMMAXXXAMASAMSAM
MMAXXMXXMSAXAXMASAAXASAMSMSMXSAMSXMMXMSMMMAMMMSMMMMMXMASXMSAMXSMMMMMASMMMAAXAXAAMMSMMXXAXAAAXASXSSSXSMMAAXSMMASMSMAMAXXAMXSAMASMXXMAMSXSXMAM
XMXSSMXMAMAMSMMAMMMSMMSXAAAAASXXMMSMXMAMAMMSMMMMSXAXAXMASXMAMAMXXMAMXMAXSXMSSMSSSXAAMXMSMMMSSSMAMAAMXMSSSMMAXAMXAMXMAMMSMMMASAAAASMSMXAMXSAM
XSAXAAAASMMMXAMSMMXXMAXXMSMMMXSXSAAAASASXXAAAXAAMXXSASMAXMMAMMXAXSAMASXMXXAMXAXAMMSMMAMAAXAAXAMAMMMMXXMAMXMAMSSSMSAMXMAAAAMMMMSMMSAAAMSMXSXS
SMMXXMMMXAAASXMXASAMMXXMAAXXMAMMMSMMAXXMAMSSMMMSSMMMMXMXSAXXXSMSXSASASASAMSSMXMAMMMAXAXSSMMSSMSSSXSMXXMAMAMSSMAAMSAMSMSSSMSXXXXAXMMMMMMAXMMS
XASMSSSXSSMXXMASAMAMSMMSXMMAMAMSAMXSMSSMSMAAXAAAAXSASAXXXMAMSAMXASXMASAMXSMAMXSAXASMXSXMXAAMAAAAMAAMSXSXMMXMAMSMMSAMXAAAAMAMXSXMXSAMAAXMMMAM
SAAAMAMXMAMXMMXMMSAMAAMAASXMMSMMXSAAXAAMAAXSMMMSSMSASMSMAXMXASXMMMAMMMXMXMXAMXAMMMSAAXXMSMMSMMMSMSAMXMMAMXSXSMMXASAMMMMSMMASMSMSASASMSMMAXXS
MMMMMAMMMSMSAMXAASASASAMXMAAAXAMXMASMSSMMXMAXXAAAAMXMASAAMXSAMXAXMAMAAAXAASMMSASAXMMMMAMXMAAXXAXMXXSAMMASASAAAXMXSSMXXXAMSAMAMAAMSXMXMASAXMA
AASASXSAAXAXMSSMMSAMAMXAAMXMAXAAXXAAAAAAXMSSSSSXSXMASXMMXSAXXXXAMXMSASMSMMSAASAMMMXMASMMMMSSSMMMSXMSASXSMASXMMMMXMMMXXSMXMXSAMSMAXAMXAMSASXM
SXXASASMSMMMXAAAXMXMAMXSASAMSSMMMMASMSSMMAXAAAAXXASXSASAMXMMSMMXSAMXXAMXMAMMMMXMASMMAXMAMXAAXXSAMXXMAMXAMXMMSAAXAAAAMMAXSAASAMXMSMSSSSXSMMAX
MAMMMMXXAAXXMAMMMMXMASAMMAXXXAMAMMXMXMAXSMMMMMMMSAMXSAMXSMAAXMAAMAMMMMMAMASASXXXXSAASMSSSMMSMMMSSMXMAMXMXMAXSASMSSMXAXMSMMMSXMXAAXXAAMXSASXM
MASXASASMSSSMSSSXMASMAXAAAXXSASMSMAXMXMMSAMXAXXASMMXMSMAMMMMSMSSSSMSAASXSASASMMSMMXMAAXAAMAAMAXAMAAMASAXMAXXXAMAXAMMXMXXXXAXMSMSMSMMMMASMMXX
SAMXXMASAAAXAAXXASASAMXMMXXASAMAAMXMMMSASAMMMSMMXAMXAAMAMAAMAAXAMXAMSXSASMMMMMAAAXSMMMMSMMXSXSXMASMSASXSASMMMMMXSMMSAAXAMMMSAAXAAXXXAMXMAMXX
MMSSMMMMMMMMMMSSXMASMXXMASMAMAMSMMAXXXMASXMAASASMSMMSMSAMSMSMMMXSMMMMXMAMXAASMSSSMXMASXMAMAXXMAXXAAMXSMMXXAAAAXXMAAXAXXMXAAMXSSMSMSMSSSMMMXM
MMAAMXSMXAXMAAXMAMAMXMXMAMMXMAMXAMSXSAMAMAMMXSAMXXXXAMMMXMAAXXSMMASAXMMSMSMMMAAAXAMSXSXSAMASAMXMAMXMMXAMMSSSSSSMSSMSSXMASMXSAXMXXAAAXAAMXSAA
MMSSMASASMMXMMMMSXAXAMAMXXXASMSMXMAASXMASXMSAMAMXMMSMSXMAMMMSAMASAMXMXXAMASXMMMSMSAMXMAXXMASAMAMXXMSSMSAAXAAMAAAAAAAAMAMAAAMMMSAMMMSMSXMASXS
AAAAMAMXMASXSXMAXMMSSSXMAMSAMMAMMMMMMSXMAXSMMMXMASAMXMAMXSXXXAXAMASMSSMMMAXXSSXXAAMSAMMSMMASAXAMMMAAAAMMMMMMMXMMMMMMSASXMAMSXAMAXXXXXXXMXSAM
MMSSMSSXSAAXMAMXXAXAAXAXAMMAMXAXAMXMMAMAMXXAASAMMMXSASAMAAXMSMMXSXMAMAASMMSMMSAMXMASASAAAMMSASXXMMSSMMMASXMASXSSXXSXMXXMMAAXMXSAMSMSMSXSXMAM
MXMAMAAAMMSXSAMMSMMXSSSMSMSAASMSMSASXMSMAXSXMXAXAXAXXSMSSSSMAAAMXXMAMMAMAMAMAMXMAXXSXMMSSMAMMMXAAAAAASXMMAMAMMAMXMAMXXSASMSMSAMMMSAAXAAXMSMM
MAMMSMMMSMMXSASMASMAMAMAAAMAMAAAASAMXXAXSXMMSSSMSMASASAAMMAXMSXSXMSASXSSSMXMASAXMAMMMXXXXMXSXAXMMMSSMMAASXMASMSMSMSAXAMAMXXAMXSAAMMMSMMMAAAA
SASAXXXAAAMXMAMMAAMMSAMSMMMSMMMMXMAMMSXMMAAMMAMAAASAAMMMMSAMXAASAMSASXMAXAXSASMSASAAMMMAMXMMMMSSXAAAMSXMMASMMMAAXAXSMMMSSMMSMMSMXSXMAAAXSMSM
SAMASMMSSSMSMSMMXSAAMXMXXSAXMXSXSSMMASMMSSMMMAMSXSMMMMXAXMXMMMSMAMMAMXAMMSMMASAMAMMASXMSMSMAMMAAMMXSMMASMXMXXSMXMXMXMXAAAAAAAXXMASMSSSMXXAAX
XAMXMXAAAXAAAAMSMMMXXMASAMXXSASXMASMXSAMAMMAXAMMMMAAMMMSSMSSSMMXAMMAMMXSAXAMAMAMMMSAMAMAAASASMMSMSMXAXAAXMMMXMAMSAMMSMMSSMMSAMSAMXAXMXMAMMMM
XAMXXMSMMMSMSMSMAAXSMSXMXMXAMMSASAMXASMMASXMSSSMASXMMAAXAAMXXAAMMMSASAAAAMSMSSXMXAMASAMMSMSXSAMXAXAMSMSMXAAAMXMASASAAXMAXMXXXAXMSMSMAAMASAMA
ASXSAAAXAAXMAXAMMMSAAMMSASAMXAMMMASMMSASAMXAAMAMMSASMMMSMMMSSMMSMASAMMSMXMXAMAXMMXSASXSXMAMXSXMMSMSMXAAXSXXSMAMMMMMXSSMMSSSMXSASAMXASXSASASM
MXAMMMMSSSSSMSAMXMMMAMASASMXMSAAMXSAXSXMMSMMSSMMMSAMAAAXAAAAAAAAAXMAMAMXXXMAMSMSAAMASMAMMASMSAMXXAMXSSXMMSMMXAXAAASXMAMXXAMAAMSSMSSXMMMMSAMX
XMAMASXMMAAAASXMAXXXAMMMXMAXAMSXSASAMMAMMXAXAAMAMXXXMMMXSMMXSMSXSMSSMMSSMSSSMAAXMXSXXAAASXSASMMAMAMXMASMAAXASMSSSSMXSXMMMMMMMSASXAMAMASXMMMS
XSAMXXAAMXMMMMASMSSXSSXXASMMXXAXMXXSASXMAMSMSSMXMAAASXSAAASXXAXXXMXXAXXAAAAASMSMSASMMSAMXXMMMMSASAMSMMXMSMSMMXAAXMXAMASXMAAAMXASMMMSMASMXAAM
XSXXXXSMMXXSSXXMXAAAMAMSASXSXMMMSMXXXXASMSXAXAXAMMAMAAMXSMXAMAMAXMSMMMSMMMSMMMAAMASAAAAXMXMXMASAMMSMAMXMAMSAMMMSMMMASXMMSMSXSMAMAXAMMAMAMMSS
MMMSSMXAMMSMMXSXMSMXMAMMMMXSAAXAAAAMAXXMSAMXMASMXAXXMXMMMMMSMAMMMMAAAAAAAXMASMMXMAMMMSSMMASAMMMMMMAMMMASAXXAMXAAAXSMMMMAAAXASMMSXMXSMMMSXAAX
XAAAAAXAMXAAMAMAMXXAMXMXAXAMMMMSSSSMSSMMSXXMAAMMASXMXAXAAXAXXXAAAAXSMSXSMSAMXXSMMXSXAXMXSAMAMAAMSSSSMSXSMSMSMSSMSMMAMAXSMSSMXAASAMASASAXMMMS
SMMSSMSSMSSSMAMSMMMXMXSXMMXXAXXAMMAXXAAXMASMMSSXXMMAXSSSSMMSSSSSSSMXAXAMAMXXAAMAMSXMASXXMMSSSMSMXAXAASMMASAXAMMSAAAAMAXMXMAASMMXAMASXMMXAAAM
MAXMMAXAAAAMMASAAAMAMAMMSAAXSXMAXSSMSSMMSMAXAXMMXMASAMAAXASAAMAMAMMMMMXSAMMMMMSAMXAMMMXMXXAMXAAMMSMMMMAMAMXMAMXMMMMMSSSMASXMMSASXMASAXSMSMSS
SXMAMMSMMMXSSMSXXMMAMXSAMXSAAMSMMXAAAXMXAMMMMSASMMAMAMXMSMMSMMAMAMAAXSMSAMSXAASASMSMASAMXMMSMSMMAMMMSSXMXXMSSMMSXSAMAMAXASASAXXAAMXMMMSAMXMX
AMSXMAXXSMMMAASAMSSMSAMASAMMMMAMMSMMMAXMMXXAXXXSAMAMAMXAXAAXMSASXSSSXMASAMMMSMSXMAXSXSASAAMSAMXMAXAAXMMMSAMAAAXAASXMASMMMSAMSSSXMASXXAMMMSMM
MAMXMMSMXMAXMMMASAAXMASXMASMXSXXXAMAMXSXAMSSXSAXXMSSSXSMSMXSASXSXXAAMMMMMXXAMAMXMAMXXMXSMSMMAMXMASMMSXAAXAMSSMMMMMASASXSMMXMMXMASAMXMXSAAMAM
XSMXXAAXSMSSXXXXMMXXMASASMMMXXAMSAMXMAMMSMMAAMMMMXXAMAMMAAXMMMAMASMXSSSSSMMSSMMAMAXMXSAXAXMMSMXMAAAAAMMSXMMXXMMMMSAMASASXMMXXSXMMSAMXASMXMAM
AAAXASMMSAMMMSMMXXAMSMSAMAASXMMMSMMSMMXAAAMMMMSAAMMAMAMMMSXMXMAMAMXXAAAXXXAAAAMSSMSAAMAMAMAAXMAMSSMMSXAXASMMMSXAAMASAMXMAXSAMXASAMXMMAMASXMM
AMMMMXXAMMMAAAASXSSMAAMXMMXMXMMAMAAXAAMMSSMSMAMMSXXMSASXXMMSASXSSSXMMMMMMMMSSMMAAAXXMMXMXMMMXSXXMAAMAMXMXMASAAXMSSXMASASXMMAXMMMASMMSSMXMAXS
SMMAXMMASXSMSSSMAAASMMMMXSASMSMASMMSSMXXAXAXMASXMMMASAMXXMAXXSAAXMAMXAMAXXAXMAMMSMMXXSXMXXAAXSMSSXMASAXMASMMMSMSAMAAAXAXMASAMSMSMMAAAXMASAMX
XASMSAMAXMXMAXAMMMMAXAXAMSASAXSAMMXMMSXAXMMMMAMAAXSAMXSSMMSSMMMMMSSMSMSMSMMSSXMAAXAXXMASMSMSMMAMMASXMAXSAMXXMAXMASXMSMSMSAMXXAAAXXMMSMSASAMA
SAMAAAMMMSAMMSXMXXXSXMMSAMXMMMMMXSASXSXMAMXAMAXMMMMXXXXAAXAAAXMAMXAAXMAMXASXMAMSSSSXXSXXAMAMAMMMMXMAMAMMAMXSMSMXAMXAMXMXMASMSMSMSXXAAXMXSAMS
XMMXSXMAASAMMAMXSMMMAMAXMMXXSASXMSASAMMXSMASMSSMMXMAMSXSMMSSSMXMASMMMSMSSXXAXXMMAMXMMSXMXMAXASMSSSMAMAMMMMAXMASMXSMMMAXMSMMAAAAASMMMSSMMSMAM
MXSAMAMMMSAMXXASAMAXAMAMASMASASAMMMMXMAAXMAMAAAXSSMMXSAAAAMXMXASAMXAMAMXXMMSMMSXMAMSAXXAMMSSMMASAMSASASAMMMMSAMXMMAMSSMMAAMSMMXSXSAAAAAAMXSX
XXMAMXMXAXMSMMSAMXSXXMMSAMMAMXSAMSXAMXMXSMAXMXMMAMAXAMSMMXSAMMMMXMSMXSSMAAAAAAXAXSXMASMSAXAAAMMMAMXXMAXAXAAAMASXASMMAMAXSSMXAMSMMMXMSXMMSAMX
MASMMMMMMSMAAMXMXMMXSAMMMMMMMAXAMSMMSASXAMMSXSAAXSAMXXMMMMMMMAAAAMXMAXAXMSSSMMSSMXAMMSAMMMSSMMXMXMSSMSMSSMSMSMMAMAAMMSSMAMMMAMAAXAAMMMSSMASX
AXASXAXAMXXMSMMXXAAAMAMAAAASMSSSMXAXSASXXAAXMAXSXSMAMSAMAAAXXSMSMSAXMSAMMXXAMAMXASAMSMXMXAMAXXXMAMXAAAAXMAXMAXSMSMMMMAXMMMXSXSSSMSXSAAXXMSMX
SSXASMSMMMXMMMMMSMMSSSMSSSMSAAAASXXMMAMMSMXSAMXXASXSXMAXSSSSMXAMASASMMXMSSXMMSSMMMSMAAAXMASXMXAXAXXMMMSMSAMXAMXASXXSMSMSAMXSMAAXAMAMMMSMXMAA
XAXMAMAAASAMMAAAXAMAXMAXAXAMMMSMXMXAMAMAXXMAMXSMAMAMMSSMAMAAAMMMAMMMMASMAAASAXXXMAAXMSMMXXAAASASXXXXMAMASXMMASMAMSMMAAAMASASMMMMSMAMASAAAMSM
MAMXASXXMMASXSMSSSMMAMXMSMXMXXMXAMXMXMSSMMXSAXAAAMSMAAXMAMSMMMSMSMXSXAXMMSMMASMSMSSSXMXXSMSSMMAXAASASAMMMAMSAMMAMXXMMMSMAMASXAXXMXXMXMMXMMAX
MAMMMSASXXXMMAAMAAXMXMAMAMMSMMMSMSASAMXMASAMXMMSXXXMMSSSSMXAMXMAMMMMMSXMAXAMAMAMMAMXAAXXXAAAXMXMMMMAMSSXMAMMASXXSXSXMAMMMMMMMMSMMMSMMMSAMXMX
SSMMAMXMAASXSMSMMMXSMMSSMXAAAAAAXMASMSASXMAMXAXXXASAMXMMMAXMASMAMAMXAMMMMSAMAMXXMMSSMMMMMMMSAAAAXAMXMAXAMAMMXSMAXASXXAXMASAAAXAAAAAAMAMASXXM
MAXMAMXMMMMAMXXAXXMXSAMXXMSSSMSSSMAMASAMXMAMXSAXAXMAMMMASXMMAXMAMAMMXSASXSMSMSMASMAAAAXAMXMAXXMSSXSAMAXXSAXXAMMSMAMXMMMSASMXXSMSMSSMMASMMMAS
MAMSSMAXMAMXMASMMXAXMXMAXXAAAAXAMXAMXMSMAMXSXMMMMASXMASAMAAXXSSMSSSMMSASAMXMAAMMMMXSSMSMMASAMXXAAMSXSMSAMXMMASAMMMMXASXMMSXSAAXAAXXMSASASXXA
MXXAAMMMXAMAMMSAMSMSMAMSMMMSMMMAMASXSAMXMMSAMXAXXXMAMXMMSSMMMMAAAXAAAMAMXMXMSMMAXXAMAMXMSXXAAMMMXMMMAMMASAMSAMMXMAMMSAAXSXAASMMMMMMMSASMSASM
SXMXMASXSMSASASAMAMAMAXAASXXXMSMMMMAMAMAXMSASXSSSMSAMXMAAMAXASMMMSSMMMAXXMSAXMSMSMMSAMAXAMMSMMXXAMAXAXSXMAAMXXMAXMMMXSMMAMSMXXXXMAAXMAMASAMS
MASASAAAAASXMAXAXASASMSSSMMXMAMSMXMAMSXXMASAMAMAMASAMAMMSSMMXXAAAXMASXSMMXSAXAAASAAXXSXSXMAXASAMSSMSSMSAMXMSMASMSMMAMAMAMAXXMXMASXSSMMMMMAMX
MMMASMMMMMMXXMMXSASXMMMXAAXAMAAASXXAMXASMMMMMMMAMMXMMSSMXAASASXMMSMMMAAAXAMXMXMXMSMSMXASMMAMAMASAAMAXAMXMAXXMAMMAXMASAMAXXMXAXXAXMAMAAXXMAMX
SXMAXASAXXXSMSAXAAXASXSSSMSXSMSASMSSSMAMAMXXAXMXMSMSAAAMMSAMXMAMSAAAMXMSMXSAMASXMMXAAAMSAMMSXSMMXMMMSSXXMXMXMAMSMMMAMAXMSMAXSMMASXSSSMSSSMSX
AXMASAMMMMMMAAMMMMMMMMAAAAXAXAMAMAAAXXAMAMAMMMAAAAAMMSMMAMSMXAAXSSSMSAXAAASMSASAMXSMSSXSMXMAXXMAMSAXAAMXMAMSMXMAASMSSSMASAMMAMSAAXXAXXXXAMXM
SXSAMMXAAAAMXMSMMAAXAMMMMAMMMXMSMMMXMMSXSMSAAMSSMMMMAXAMXXAXSSMXXAXASXSMSMSXMAXAMMAAAXXMXXSMMMMAXMAMXSAAXAXMAMSMMMAAAXMXMAXSAMMMMSMAMMMMSAMX
XAMXMASMXXXXAXAASXXSXSXAXAMXAAMXMAXMAXMAMAXMMXAAMSSMASAMSMSAAAXMMMMASXMAXXMXXMXSMAMMMSSXSASAAMMSSSSMAAXXMMMMAMMMSMMMSMSAMXMMASXSAXMXMXAAAASX
MXMXAXXMASMMMXMXMXXMAMXSSMSXSASXSAMASXMAMAMASMMMAAAMAMXMSAMXSMAMMAMASAMAMAMSMSAMASXMXXMASMSSMSMAAAXMXSMMMSASAXSASAMXMAMXMAXSXMAMAXSAMSSXMAMX
SAMXMXMMXAAASMSSSXSMAMAMAMXMXAXXMAMAMAMXMSMAMASXMSSMMXSAMXMMMMXASASXSMMXSAMXAMAMAXAMXAMXMAMXMAMMMMMMAAAMAMASXMMASAMXMSMSSMXMMSAMXMMAXAAMMSXM
MXXAMMXMXMSMSAAAAAAMAMMSSMAMMMMSSMMSSMMSMAMMSAMMMAMASASXMAMAAXMMSASMMMAASASMSMSMXSAMSSMASXMASAMMASAMSSSMASMMMSMMMMSMMMAMAMMSASASMMSMMMSXMMAM
SSSXSAASMXMXMMMSMMMSXSAAASASXAAAAAAAAMAASASMMAXAMASAMASASASMMSAXMXMAAMMMSAMXXAMAAMAMAASXSAAAMSMSASXXXAXXXMASAAAXMASAXSASAMAMMSXMXAAAAXMAASMM
XMASAMMSAXMASAMXMXXAAMMSMSSSXMSSSMMSSMSXSXSXSMMXSASMMXMMSMSXMAMXMASMMSSXMAMXMMMMMSAMSMMXSMMMSXXMASMXMXMASMMMSSSMMMSSMSAMXMXSMMMXMSSSMSSSMAMM";
