#[allow(dead_code)]
use std::collections::HashMap;
use std::hash::Hash;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

// Utility types
/// Generic defaultdict equivalent with keys of type `T` and values of type `U` 
pub struct DefaultHashMap<T,U> {
    map:HashMap<T,U>,
    default:U
}
impl<T: Eq + Hash + Copy,U: Copy> DefaultHashMap<T,U> {
    pub fn new(default:U) -> DefaultHashMap<T,U> {
        return DefaultHashMap { map:HashMap::<T,U>::new(), default };
    }
    /// Insert or update the `val` for the given `key` 
    pub fn insert(&mut self, key:T, val:U) -> Option<U> {
        return self.map.insert(key, val);
    }
    pub fn len(&self) -> usize {
        return self.map.len();
    }
    pub fn get(&self, key:&T) -> &U {
        if self.map.contains_key(key) {
            return self.map.get(key).unwrap();
        }
        else {
            return &self.default;
        }
    }
    /// Returns a mutable reference to the value for a given `key`. If the 
    /// entry does not already exist, one is created with the `default` value.
    pub fn get_mut(&mut self, key:&T) -> &mut U {
        if self.map.contains_key(key) {
            return self.map.get_mut(key).unwrap();
        }
        else {
            self.insert(*key, self.default);
            return self.map.get_mut(key).unwrap();
        }
    }
}
/// 2D Vector struct
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Vec2 {
    pub x:isize,
    pub y:isize
}
impl Vec2 {
    pub fn new(x:isize, y:isize) -> Vec2 {
        return Vec2{x:x, y:y};
    }
    pub fn newu(x:usize, y:usize) -> Vec2 {
        return Vec2{x:x as isize, y:y as isize};
    }
    /// Tests if the coordinate is within the bounds of a zero-based rectangle 
    /// with dimensions `dim_x`, `dim_y`.
    pub fn in_bounds(self, dim_x:usize, dim_y:usize) -> bool {
        return self.x >= 0 && self.x < dim_x as isize && self.y >= 0 && self.y < dim_y as isize;
    }
}
impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
		write!(f,"({},{})", self.x, self.y)
    }
}
impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs:Vec2) -> Vec2 {
        return Vec2{x:self.x + rhs.x, y:self.y + rhs.y};
    }
}
impl std::ops::Mul<isize> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs:isize) -> Vec2 {
        return Vec2{x:self.x * rhs, y:self.y * rhs};
    }
}
/// 3D vector struct
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Vec3 {
    pub x:isize,
    pub y:isize,
    pub z:isize
}
impl Vec3 {
    pub fn new(x:isize, y:isize, z:isize) -> Vec3 {
        return Vec3{x:x, y:y, z:z};
    }
    pub fn newu(x:usize, y:usize, z:usize) -> Vec3 {
        return Vec3{x:x as isize, y:y as isize, z:z as isize};
    }
    /// Tests if the coordinate is within the bounds of a zero-based volume 
    /// with dimensions `dim_x`, `dim_y`, `dim_z`.
    pub fn in_bounds(self, dim_x:usize, dim_y:usize, dim_z:usize) -> bool {
        return self.x >= 0 && self.x < dim_x as isize && self.y >= 0 && self.y < dim_y as isize && self.z >= 0 && self.z < dim_z as isize;
    }
}
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
		write!(f,"({},{},{})", self.x, self.y, self.z)
    }
}
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs:Vec3) -> Vec3 {
        return Vec3{x:self.x + rhs.x, y:self.y + rhs.y, z:self.z + rhs.z};
    }
}
impl std::ops::Mul<isize> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs:isize) -> Vec3 {
        return Vec3{x:self.x * rhs, y:self.y * rhs, z:self.z * rhs};
    }
}
// General helper functions
// File input
/// Read an input file into lines
pub fn read_input(file_path:&str) -> Result<Vec<String>, String>{
    match File::open(file_path) {
        Err(_) => Err(format!("Unable to open file at path {}", file_path)),
        Ok(file) => {
            let mut vec:Vec<String> = Vec::new();
            let reader = BufReader::new(file);
            for (_index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                vec.push(line);
            }
	        return Ok(vec);
        }
    }
}
/// Break input lines into sections on empty lines
pub fn sections(lines:&Vec<String>) -> Vec<Vec<String>> {
    let mut sections:Vec<Vec<String>> = Vec::new();
    sections.push(Vec::new());
    let mut section_count = 1;
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            sections.push(Vec::new());
            section_count += 1;
        }
        else {
            sections[section_count - 1].push(lines[i].clone());
        }
    }
    return sections;
}
/// Extract all base 10 integers in a string
pub fn ints_in_string(string:&String) -> Vec<isize> {
    let re = Regex::new(r"-?\d+\.?\d*").unwrap();
    let string_matches: Vec<&str> = re.find_iter(string).map(|m| m.as_str()).collect();
    let mut result:Vec<isize> = Vec::new();
    for s in string_matches {
        match s.parse::<isize>() {
            Ok(x) => result.push(x),
            _ => {}
        }
    }
    return result;
}
/// Extract all base 10 integers from a list of strings. Performs better than 
/// the single string version due to the relative slowness of compiling a 
/// `Regex`
pub fn ints_in_strings(strings:&Vec<String>) -> Vec<Vec<isize>> {
    let re = Regex::new(r"-?\d+\.?\d*").unwrap();
    let mut result = Vec::new();
    for string in strings {
        let string_matches: Vec<&str> = re.find_iter(string).map(|m| m.as_str()).collect();
        let mut ints:Vec<isize> = Vec::new();
        for s in string_matches {
            match s.parse::<isize>() {
                Ok(x) => ints.push(x),
                _ => {}
            }
        }
        result.push(ints);
    }
    return result;
}
/// Extract all base 10 floats in a string (integers will be parsed as floats)
pub fn floats_in_string(string:&String) -> Vec<f64> {
    let re = Regex::new(r"-?\d+\.?\d*").unwrap();
    let string_matches: Vec<&str> = re.find_iter(string).map(|m| m.as_str()).collect();
    let mut result:Vec<f64> = Vec::new();
    for s in string_matches {
        match s.parse::<f64>() {
            Ok(x) => result.push(x),
            _ => {}
        }
    }
    return result;
}
/// Extract all base 10 floats from a list of strings. Performs better than 
/// the single string version due to the relative slowness of compiling a 
/// `Regex`
pub fn floats_in_strings(strings:&Vec<String>) -> Vec<Vec<f64>> {
    let re = Regex::new(r"-?\d+\.?\d*").unwrap();
    let mut result = Vec::new();
    for string in strings {
        let string_matches: Vec<&str> = re.find_iter(string).map(|m| m.as_str()).collect();
        let mut floats:Vec<f64> = Vec::new();
        for s in string_matches {
            match s.parse::<f64>() {
                Ok(x) => floats.push(x),
                _ => {}
            }
        }
        result.push(floats);
    }
    return result;
}
// 2D grid functions
/// Reads a grid as a 2D vector of `char`s
pub fn read_grid(lines:&Vec<String>) -> Vec<Vec<char>> {
    let mut grid:Vec<Vec<char>> = Vec::new();
    for i in 0..lines.len() {
        grid.push(Vec::new());
        for c in lines[i].chars() {
            grid[i].push(c);
        }
    }
    return grid;
}
/// Print a 2D vector grid
pub fn print_grid(grid:&Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{} ", grid[y][x]);
        }
        println!();
    }
}
/// Reads the grid as a `DefaultHashMap`, plus the grid width and height
pub fn read_grid_map(lines:&Vec<String>, default_char:char) -> Result<(DefaultHashMap<Vec2, char>, usize, usize), String> {
    let mut map = DefaultHashMap::<Vec2, char>::new(default_char);
    if lines.len() == 0 {
        return Err("lines are empty".to_string());
    }
    let width = lines[0].len();
    let height = lines.len();
    for y in 0..lines.len() {
        if lines[y].len() != width {
            return Err(format!("Irregular grid: expecting width {} at line {}, found {}", width, y, lines[y].len()));
        }
        let line_chars:Vec<char> = lines[y].chars().collect();
        for x in 0..width {
            if line_chars[x] != default_char {
                map.insert(Vec2::newu(x,y), line_chars[x]);
            }
        }
    }
    return Ok((map, width, height));
}
/// Print a DefaultHashMap grid
pub fn print_grid_map(grid_map:&DefaultHashMap<Vec2, char>, width:usize, height:usize) {
    for y in 0..height {
        for x in 0..width {
            let coord = Vec2::newu(x,y);
            print!("{}", grid_map.get(&coord));
        }
        println!();
    }
}
/// Delta vectors to orthogonally adjacent coords N,E,S,W
pub fn adjacent4() -> Vec<Vec2> {
    return vec![Vec2::new(0,-1), Vec2::new(1,0), Vec2::new(0,1), Vec2::new(-1,0)];
}
/// Delta vectors to orthogonally adjacent coords N,E,S,W + self
pub fn adjacent5() -> Vec<Vec2> {
    return vec![Vec2::new(0,-1), Vec2::new(1,0), Vec2::new(0,1), Vec2::new(-1,0), Vec2::new(0,0)];
}
/// Delta vectors to adjacent coords including diagonals N,NE,E,SE,S,SW,W,NW
pub fn adjacent8() -> Vec<Vec2> {
    return vec![Vec2::new(0,-1), Vec2::new(1,-1), Vec2::new(1,0), Vec2::new(1,1), Vec2::new(0,1), Vec2::new(-1,1), Vec2::new(-1,0), Vec2::new(-1,-1)];
}
/// Delta vectors to adjacent coords including diagonals 
/// N,NE,E,SE,S,SW,W,NW + self
pub fn adjacent9() -> Vec<Vec2> {
    return vec![Vec2::new(0,-1), Vec2::new(1,-1), Vec2::new(1,0), Vec2::new(1,1), Vec2::new(0,1), Vec2::new(-1,1), Vec2::new(-1,0), Vec2::new(-1,-1), Vec2::new(0,0)];
}
/// Return HashMap of directional characters `^>v<` to direction deltas
pub fn arrow_dirs() -> HashMap<char, Vec2> {
    return HashMap::<char, Vec2>::from([('^', Vec2::new(0,-1)), ('>', Vec2::new(1,0)), ('v', Vec2::new(0,1)), ('<', Vec2::new(-1,0))]);
}
// Math functions - using i128s for most parameters. May revise if it causes performance 
// difficulties.
/// Return `a` modulo `m`, but with the result always non-negative
pub fn abs_mod(a:i128, m:usize) -> i128 {
    let mut result = a % m as i128;
    if result < 0 {
        result += m as i128;
    } 
    return result;
}
/// Basic Sieve of Eratosthenes. Returns all integers less than `limit` and 
/// greater than 1 that are prime (more or less, no extraordinary measures 
/// regarding scale)
pub fn eratosthenes(limit:i128) -> Vec<i128> { 
    let mut local_limit = limit;
    if local_limit < 2 {
        local_limit = 2;
    }
    let mut primes = Vec::new();
    let mut sieve = vec![false; local_limit as usize];
    let mut increment = 2;
    loop {
        for i in (2*increment..local_limit).step_by(increment as usize) {
            sieve[i as usize] = true;
        }
        let mut done = true;
        for i in (increment+1)..local_limit {
            if !sieve[i as usize] {
                increment = i;
                done = false;
                break;
            }
        }
        if done {
            break;
        }
    }
    for i in 2..local_limit {
        if !sieve[i as usize] {
            primes.push(i);
        }
    }
    return primes;
}
/// Return the digits of `x` in base `n` read left to right, optionally padded 
/// to `required_len`
pub fn base_n_digits(x:i128, n:usize, required_len:Option<usize>) -> Vec<i128> {
    let mut digits = Vec::new();
    let mut x_temp = x;
    while x_temp > 0 {
        // prepend next most significant digit
        let digit = x_temp % (n as i128);
        digits.insert(0, digit);
        x_temp -= digit;
        x_temp /= n as i128;
    }
    match required_len {
        Some(len) => {
            // prepend with zeroes
            while digits.len() < len {
                digits.insert(0, 0);
            }
        },
        None => {}
    }
    return digits;
}
/// Compute the GCD of `a` and `b`
pub fn gcd(a:i128, b:i128) -> i128 {
    if a == b { 
        return a; 
    }
    let mut a_local = a;
    let mut b_local = b;
    if b > a {
        a_local = b;
        b_local = a;
    }
    while b_local > 0 {
        let temp = a_local;
        a_local = b_local;
        b_local = temp % b_local;
    }
    return a_local;
}
/// Compute  (GCD(`a`,`b`), `x`, `y`) such that `ax` + `by` = GCD(`a`,`b`)
/// via the extended Euclidean algorithm
pub fn extended_gcd(a:i128, b:i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }
    else {
        let (gcd, x, y) = extended_gcd(b % a, a);
        return (gcd, y - (b/a) * x, x);
    }
}
/// Construct `x`^`n` % `m`
pub fn mod_exp(x:i128 , n:i128 , m:i128) -> i128{
    if n <= 0 { 
        return 1; 
    }
    let mut result = 1;
    let mut n_local = n;
    let mut x_local = x;
    loop {
        if n_local == 1 { 
            return (result * x_local) % m; 
        }
        // if n even, square x, halve n
        if n_local % 2 == 0 {
            x_local = (x_local * x_local ) % m;
            n_local /= 2;
        }
        // if n odd, mult result by x, decrement n
        else {
            result = (result * x_local) % m;
            n_local -= 1;
        }
    }
}
/// Compute the modular multiplicative inverse of `x` modulo `p` if `p` >= 1
/// and `x`,`p` coprime
pub fn mod_inv(x:i128, p:i128) -> Option<i128> {
    if p < 1 {
        return None;
    }
    let (gcd, i, _) = extended_gcd(x, p);
    if gcd != 1 {
        return None;
    }
    return Some((i % p + p) % p);
}
/// Compute the least common multiple of `a`,`b`
pub fn lcm(a:i128, b:i128) -> i128 {
    return (a*b).abs() / gcd(a, b);
}
/// Calculate a remainder satisfying all given `congruences` (having pairwise 
/// coprime moduli) using the Chinese Remainder Theorem. Congruences are passed
/// as a tuples of (modulus, remainder)
pub fn crt(congruences:&Vec<(i128, i128)>) -> Option<i128> {
    // require all moduli to be pairwise coprime
    for i in 0..congruences.len() {
        for j in i+1..congruences.len() {
            if gcd(congruences[i].0, congruences[j].0) != 1 {return None};
        }
    }
    let mut sum = 0;
    let mut product = 1;
    for i in 0..congruences.len() {
        product *= congruences[i].0;
    }
    for i in 0..congruences.len() {
        let m = product / congruences[i].0;
        match mod_inv(m, congruences[i].0) {
            Some(inv) => {
                sum += congruences[i].1 * inv * m; 
            }
            None => {
                return None
            }
        }
    }
    return Some(sum % product);
}
