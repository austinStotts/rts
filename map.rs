use rand::prelude::*;
use std::collections::HashMap;

type RoomIndex = usize;

fn generate_map(num_rooms: usize) -> HashMap<RoomIndex, Vec<RoomIndex>> {
    let mut map: HashMap<RoomIndex, Vec<RoomIndex>> = HashMap::new();
    let mut rng = thread_rng();

    loop { // Keep regenerating maps until a valid one is found
        map.clear(); // Reset for a new attempt

        // Ensure start and end rooms
        let start_room = 0;
        let end_room = num_rooms - 1;
        map.insert(start_room, vec![]);
        map.insert(end_room, vec![]);

        // Generate connections with potential dead-ends
        for room_index in 1..(num_rooms - 1) {
            map.insert(room_index, vec![]); // Initialize room's connections

            let num_connections = rng.gen_range(1..3); // 1-2 connections per room
            for _ in 0..num_connections {
                let neighbor = rng.gen_range(0..num_rooms);
                map.get_mut(&room_index).unwrap().push(neighbor);
                map.get_mut(&neighbor).unwrap().push(room_index);
            }
        }

        if path_exists(&map, start_room, end_room) {
            break; // Valid map found, exit the loop
        } 
    }

    map
}

fn path_exists(map: &HashMap<RoomIndex, Vec<RoomIndex>>, start: RoomIndex, end: RoomIndex) -> bool {
    let mut visited = vec![false; map.len()];
    let mut queue = std::collections::VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while !queue.is_empty() {
        let current_room = queue.pop_front().unwrap();

        if current_room == end {
            return true; 
        }

        if let Some(neighbors) = map.get(&current_room) {
            for neighbor in neighbors {
                if !visited[*neighbor] {
                    visited[*neighbor] = true;
                    queue.push_back(*neighbor);
                }
            }
        }
    }

    false 
}

fn print_map(map: &HashMap<RoomIndex, Vec<RoomIndex>>, start: RoomIndex, end: RoomIndex) {
    for room_index in 0..map.len() {
        if room_index == start {
            print!("🚪"); 
        } else if room_index == end {
            print!("🚩"); 
        } else {
            print!("⬛"); 
        }
        if let Some(neighbors) = map.get(&room_index) {
            for neighbor in neighbors {
                print!("-");
            }
        }
        println!();
    }
}

fn main() {
    let map = generate_map(10);  
    print_map(&map, 0, 9);
}
