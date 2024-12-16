use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    io::{BufRead, BufReader, Read},
    ops::DerefMut,
    sync::Mutex,
};

use grid::{Direction, Grid, Location};

use crate::Aoc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    End,
    Wall,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Cell::Empty => " ",
            Cell::Start => "S",
            Cell::End => "E",
            Cell::Wall => "#",
        };
        write!(f, "{}", char)
    }
}

fn parse(buf: &mut dyn Read) -> Grid<Cell> {
    let reader = BufReader::new(buf);
    let mut line_length = 0;
    let lines = reader.lines().map_while(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            None
        } else {
            line_length = line.len();
            Some(line)
        }
    });
    Grid::new(
        lines.collect::<String>().chars().map(|c| match c {
            '.' => Cell::Empty,
            'S' => Cell::Start,
            'E' => Cell::End,
            '#' => Cell::Wall,
            _ => panic!("Invalid map cell: {}", c),
        }),
        line_length,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Step {
    Forward,
    TurnLeft,
    TurnRight,
}

impl Step {
    fn cost(&self) -> usize {
        match self {
            Step::Forward => 1,
            Step::TurnLeft | Step::TurnRight => 1000,
        }
    }
}

#[derive(Debug)]
enum FoldableValue<T> {
    Fresh(T),
    Previous(T),
    Unknown,
}

impl<T> FoldableValue<T>
where
    T: Clone,
{
    fn degrade(&self) -> FoldableValue<T> {
        match self {
            FoldableValue::Fresh(value) => FoldableValue::Previous(value.clone()),
            FoldableValue::Previous(_) => FoldableValue::Unknown,
            FoldableValue::Unknown => FoldableValue::Unknown,
        }
    }
}

#[derive(Debug)]
struct Route {
    steps: Vec<Step>,
    cost: Mutex<FoldableValue<usize>>,
    end_state: Mutex<FoldableValue<(Location, Direction)>>,
}

impl Route {
    fn new(steps: Vec<Step>) -> Route {
        Route {
            steps,
            cost: Mutex::new(FoldableValue::Unknown),
            end_state: Mutex::new(FoldableValue::Unknown),
        }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.steps.eq(&other.steps)
    }
}

impl Eq for Route {}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost().cmp(&other.cost())
    }
}

impl Route {
    fn cost(&self) -> usize {
        let mut guard = self.cost.lock().unwrap();
        let cost_ref = guard.deref_mut();
        match cost_ref {
            FoldableValue::Fresh(cost) => *cost,
            FoldableValue::Previous(cost) => {
                let new_cost = *cost + self.steps.last().unwrap().cost();
                *cost_ref = FoldableValue::Fresh(new_cost);
                new_cost
            }
            FoldableValue::Unknown => {
                let new_cost = self.steps.iter().fold(0, |acc, step| acc + step.cost());
                *cost_ref = FoldableValue::Fresh(new_cost);
                new_cost
            }
        }
    }

    fn end_state(&self, start: &Location) -> Option<(Location, Direction)> {
        let mut guard = self.end_state.lock().unwrap();
        let end_state_ref = guard.deref_mut();
        match end_state_ref {
            FoldableValue::Fresh(end_state) => Some(end_state.clone()),
            FoldableValue::Previous(previous_end_state) => {
                let (previous_location, previous_direction) = previous_end_state.clone();
                let last_step = self.steps.last().unwrap();
                let new_end_state = match last_step {
                    Step::Forward => previous_location
                        .to(previous_direction)
                        .map(|loc| (loc, previous_direction)),
                    Step::TurnLeft => Some((previous_location, previous_direction.turn_left())),
                    Step::TurnRight => Some((previous_location, previous_direction.turn_right())),
                };

                if let Some(new_end_state) = new_end_state.clone() {
                    *end_state_ref = FoldableValue::Fresh(new_end_state)
                }

                new_end_state
            }
            FoldableValue::Unknown => {
                let new_end_state = self.steps.iter().try_fold(
                    (start.clone(), Direction::Right),
                    |(location, direction), step| match step {
                        Step::Forward => location.to(direction).map(|loc| (loc, direction)),
                        Step::TurnLeft => Some((location, direction.turn_left())),
                        Step::TurnRight => Some((location, direction.turn_right())),
                    },
                );
                if let Some(new_end_state) = new_end_state.clone() {
                    *end_state_ref = FoldableValue::Fresh(new_end_state)
                }

                new_end_state
            }
        }
    }

    fn clone_with_additional_step(&self, step: Step) -> Route {
        let mut steps = self.steps.clone();
        steps.push(step);
        Route {
            steps,
            cost: Mutex::new(self.cost.lock().unwrap().degrade()),
            end_state: Mutex::new(self.end_state.lock().unwrap().degrade()),
        }
    }

    fn cells<'a>(&'a self, start: &'a Location) -> impl Iterator<Item = Location> + 'a {
        let mut position = start.clone();
        let mut direction = Direction::Right;

        self.steps.iter().map_while(move |step| match step {
            Step::Forward => {
                let new_position = position.to(direction)?;
                position = new_position;
                Some(position.clone())
            }
            Step::TurnLeft => {
                direction = direction.turn_left();
                Some(position.clone())
            }
            Step::TurnRight => {
                direction = direction.turn_right();
                Some(position.clone())
            }
        })
    }
}

fn find_routes(map: &Grid<Cell>, start: &Location, end: &Location) -> Vec<Route> {
    let mut routes = VecDeque::new();
    routes.push_front(Route::new(vec![]));

    let mut min_score_location: HashMap<(Location, Direction), usize> = HashMap::new();

    let mut minimum_route = None;

    while let Some(route) = routes.pop_front() {
        let Some(final_state) = route.end_state(start) else {
            continue;
        };
        if &final_state.0 == end {
            minimum_route = Some(route);
            break;
        }

        if let Some(min_score) = min_score_location.get(&final_state) {
            if *min_score < route.cost() {
                continue;
            }
        }
        min_score_location.insert(final_state.clone(), route.cost());

        let forward = final_state.0.to(final_state.1);
        if matches!(
            forward.and_then(|loc| map.get(&loc)),
            Some(Cell::Empty | Cell::Start | Cell::End)
        ) {
            routes.push_front(route.clone_with_additional_step(Step::Forward));
        }
        if route
            .steps
            .last()
            .map(|last_step| last_step == &Step::Forward)
            .unwrap_or(true)
        {
            routes.push_back(route.clone_with_additional_step(Step::TurnRight));
            routes.push_back(route.clone_with_additional_step(Step::TurnLeft));
        }

        routes.make_contiguous().sort();
    }

    let Some(minimum_route) = minimum_route else {
        return vec![];
    };

    let target_score = minimum_route.cost();
    let mut shortest_routes = vec![minimum_route];

    while let Some(route) = routes.pop_front() {
        if route.cost() > target_score {
            break;
        }
        if route.end_state(start).map(|v| &v.0 == end).unwrap_or(false) {
            shortest_routes.push(route)
        }
    }

    shortest_routes
}

fn part1(buf: &mut dyn Read) {
    let map = parse(buf);

    let start_location = map
        .iter_locations()
        .find(|location| map.get(location) == Some(&Cell::Start))
        .unwrap();
    let end_location = map
        .iter_locations()
        .find(|location| map.get(location) == Some(&Cell::End))
        .unwrap();

    let routes = find_routes(&map, &start_location, &end_location);
    let route = routes.first();
    let result = route.unwrap().cost();
    println!("Part 1: {}", result);
}

fn part2(buf: &mut dyn Read) {
    let map = parse(buf);

    let start_location = map
        .iter_locations()
        .find(|location| map.get(location) == Some(&Cell::Start))
        .unwrap();
    let end_location = map
        .iter_locations()
        .find(|location| map.get(location) == Some(&Cell::End))
        .unwrap();

    let routes = find_routes(&map, &start_location, &end_location);

    let mut cells = HashSet::new();
    for route in routes {
        for cell in route.cells(&start_location) {
            cells.insert(cell);
        }
    }

    let result = cells.len();
    println!("Part 2: {}", result);
}

inventory::submit!(Aoc::new(
    2024,
    16,
    part1,
    part2,
    include_bytes!("./inputs/day16")
));
