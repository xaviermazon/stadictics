use rand::Rng;
use std::collections::HashMap;

// Quicksort
fn particionador(v: &mut Vec<i32>, inicio: u32, fin: u32) -> u32 {
    let pivote = v[inicio as usize];
    let mut i: u32 = inicio + 1;
    let mut j: u32 = fin;
    while i < j + 1 {
        while i < j + 1 && v[i as usize] <= pivote {
            i += 1;
        }
        while i < j + 1 && v[j as usize] >= pivote {
            j -= 1;
        }
        if i < j + 1 {
            v.swap(i as usize, j as usize);
        }
    }
    v.swap(inicio as usize, j as usize);
    j
}

fn quick_sort(v: &mut Vec<i32>, inicio: u32, fin: u32) {
    if inicio < fin {
        let pivote: u32 = particionador(v, inicio, fin);
        quick_sort(v, inicio, pivote - 1);
        quick_sort(v, pivote + 1, fin);
    }
}

fn buscador(map: HashMap<i32, i32>, v: Vec<i32>) -> i32 {
    let mut key = 0;
    let mut value = 0;
    for k in v.iter() {
	if value < *map.get(k).unwrap_or(&0) {
            value = *map.get(k).unwrap_or(&0);
            key = *k;
        }
    }
    key
}

fn rellenar_vector() -> Vec<i32> {
    let mut aux = Vec::new();
    for _i in 0..50 {
        aux.push(rand::thread_rng().gen_range(0..250));
    }
    aux
}

fn main() {
    let mut v = rellenar_vector();
    let length: u32 = (v.len() - 1) as u32;
    quick_sort(&mut v, 0, length);
    let mut acum = 0;
    let mut map: HashMap<i32,i32> = HashMap::new();
    for element in v.iter() {
        let count = map.entry(*element).or_insert(0);
        *count += 1;
        acum += element;
    }
    println!("La media es: {}", acum / (length + 1) as i32);
    println!("El mediano es: {}", v[((length + 1) / 2) as usize]);
    println!("La moda es: {}", buscador(map,v));
}
