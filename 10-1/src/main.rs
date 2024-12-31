use std::{
    collections::{HashMap, HashSet},
    fmt,
    io::Read,
    rc::Rc,
    sync::RwLock,
};

fn main() {
    let input = get_input().unwrap();
    let map = Map::parse(&input).unwrap();
    println!("Number of paths: {}", map.num_paths());
}

fn get_input() -> Result<String, Error> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input)?;

    if input.is_empty() {
        input = DEFAULT_INPUT.to_string();
    }

    Ok(input)
}

struct Map {
    trail_heads: Vec<Rc<RwLock<Location>>>,
}

impl Map {
    fn parse(input: &str) -> Result<Self, Error> {
        let mut trail_heads: Vec<Rc<RwLock<Location>>> = Vec::new();
        let mut map: HashMap<(usize, usize), Rc<RwLock<Location>>> = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, location) in line.chars().enumerate() {
                if let Some(e) = location.to_digit(10) {
                    let elevation = e as u8;
                    let mut current = Location::new((x, y), elevation);

                    // Set North
                    if y > 0 {
                        if let Some(north) = map.get(&(x, y - 1)) {
                            current.north = Some(Rc::clone(north));
                        } else {
                            return Err(Error::new(format!(
                                "Could not find north for ({}, {})!",
                                x, y
                            )));
                        }
                    }
                    // Set West
                    if x > 0 {
                        if let Some(west) = map.get(&(x - 1, y)) {
                            current.west = Some(Rc::clone(west));
                        } else {
                            return Err(Error::new(format!(
                                "Could not find west for ({}, {})!",
                                x, y
                            )));
                        }
                    }

                    let rc = Rc::new(RwLock::new(current));
                    // Update North's South
                    if y > 0 {
                        if let Some(north) = map.get_mut(&(x, y - 1)) {
                            north.write().unwrap().south = Some(Rc::clone(&rc));
                        } else {
                            return Err(Error::new(format!(
                                "Could not find north for ({}, {})!",
                                x, y
                            )));
                        }
                    }
                    // Update West's East
                    if x > 0 {
                        if let Some(west) = map.get_mut(&(x - 1, y)) {
                            west.write().unwrap().east = Some(Rc::clone(&rc));
                        } else {
                            return Err(Error::new(format!(
                                "Could not find west for ({}, {})!",
                                x, y
                            )));
                        }
                    }

                    if elevation == TRAIL_BEGIN {
                        trail_heads.push(Rc::clone(&rc));
                    }
                    map.insert((x, y), rc);
                }
            }
        }

        Ok(Self { trail_heads })
    }

    fn num_paths(&self) -> usize {
        let mut paths = 0;

        for trail_head in &self.trail_heads {
            let location = trail_head.read().unwrap();
            paths += location.num_peaks();
        }

        paths
    }
}

struct Location {
    location: (usize, usize),
    elevation: u8,
    north: Option<Rc<RwLock<Location>>>,
    south: Option<Rc<RwLock<Location>>>,
    east: Option<Rc<RwLock<Location>>>,
    west: Option<Rc<RwLock<Location>>>,
}

impl Location {
    fn new(location: (usize, usize), elevation: u8) -> Self {
        Self {
            location,
            elevation,
            north: None,
            south: None,
            east: None,
            west: None,
        }
    }

    fn num_peaks(&self) -> usize {
        self.travel().len()
    }

    fn travel(&self) -> HashSet<(usize, usize)> {
        let mut peaks = HashSet::new();
        if self.elevation == TRAIL_END {
            peaks.insert(self.location);
            return peaks;
        }

        if let Some(north) = &self.north {
            let location = north.read().unwrap();
            if location.elevation == self.elevation + 1 {
                for peak in location.travel() {
                    peaks.insert(peak);
                }
            }
        }

        if let Some(west) = &self.west {
            let location = west.read().unwrap();
            if location.elevation == self.elevation + 1 {
                for peak in location.travel() {
                    peaks.insert(peak);
                }
            }
        }

        if let Some(east) = &self.east {
            let location = east.read().unwrap();
            if location.elevation == self.elevation + 1 {
                for peak in location.travel() {
                    peaks.insert(peak);
                }
            }
        }

        if let Some(south) = &self.south {
            let location = south.read().unwrap();
            if location.elevation == self.elevation + 1 {
                for peak in location.travel() {
                    peaks.insert(peak);
                }
            }
        }

        peaks
    }
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Self {
        Error { message }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::new(value.to_string())
    }
}

const TRAIL_END: u8 = 9;
const TRAIL_BEGIN: u8 = 0;
const DEFAULT_INPUT: &str = "7651234510103430129656654566543012101234544567676545
8340987643212345038743783087452143450925655438585432
9205676544569416545032192196567854567810710129891201
0114323456678507896123010105432945498789878778100323
1023210987897612387654981234941834354308909689011414
4321301010767887434767876547870921267210218574326505
5650452121456996545897987676589810898876123465467876
4781965432348765698018734585476701017965019894894966
9890874321089450367129653090365432676874320767743987
0781103016567321458934532101270121085401221678652156
1692012923498212367012983501989789098320034569565043
2543127874321003450143477652876652167012123430676530
3432234566543234321894568943905443256587656321089421
4391001057872145430765410878912321101498943243989321
0189432326981056741696321767010630122367050112078930
3276593410456747890781011054321545012454198703163210
4566782561267832781652322123765436723543217654254301
9687601670358901692343123098890127834984503234348911
8794512385443210543510004567734323941076678105067410
0123410496554381235671212321065419452345549876154323
4320321587010190134587389412176108760632438963235432
5411289698121285421098498503489954321541328754556741
6700348787232376332567567694567869325670019669430890
7889465676543101221410698587645678410581204578521321
4976534587894123430321587450784349525890323467670410
3098621298765036541543676521698237656765412896789567
2180590178981047897632101434532118949896106701693458
3671287065432156788912012321089001232787765432542109
4576396101276549874301015401672101301289850343432101
0985405234387632965432109872543985423477891296508765
1256714985496101876343234563837676510566780187219656
0349823076567987654894303474988543412365443210389345
1256712187458978703765412985679812305430301223478234
0210603492324569812654501874876703276321212984560143
4389896501215498701967012565945432181430365806543231
5676287618900398632878123474236701090568456717632100
5432109807651217540169010089147898101279894328986521
6701018712349106543254322128056763012389765010147432
7891127689678100145101013438987652121450110981238901
8980334574549233236232304587012343210969231476545432
8901265463032147867347212698743476521878542345696543
7898378352104056998478923787654389430677653218787656
6987499243693456787567014568785203434587965109690145
5496587156782321893459985439690112521097874065432234
0381016045071040134578376326541045671010123678321101
1272325432101056923667289017432434982234234349830765
0365430340342347810756108128921125543145641256545854
6674321201259659876843201234210014698076450127894903
7889210398768760145998910165436786787189361898765212
6901165409694321234107323876325495698321276345654303
5432076210585012321236454989414324321230985432783212
4345189823476523410345567898701015450101234321098101";
