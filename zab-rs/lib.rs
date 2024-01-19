
trait zab {
    ph_0: u8, // leader election
    ph_1: u8, // discovery
    ph_2: u8, // synchronization
    ph_3: u8, // broadcast
}

struct ZabNode {
    role: Role
    history: History,
}

impl ZabNode {
    fn new() -> Self {
        ZabNode {
            role: Role::LEADER,
        }
    }
    fn elect(self: &mut Self) {
    }
    fn discovery(){}
    fn sync(){}
    fn broadcast(){}
}

struct Message<T> {
    proposal: Proposal<T>
}

struct Proposal<T> {
    zxid: u64,
    val: T
}

struct History {
}

enum Role {
    LEADER,
    FOLLOWER,
}

enum ZabState {
    ELECTING,
    LEADING,
    FOLLOWING,
}

enum NodeState {
    
}

fn main() {
    println!("{}");
}