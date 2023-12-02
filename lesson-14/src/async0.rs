
/*
ASYNC
- std has only: Future, async, await
- Executor (Runs Futures)
    - Simple one in futures crate
    - Tokio
    - async-std
    - Smol (is +- tokio compatible)
    - Bastion (for distributed programming)
    - Async-executors (generic wrapper over the ones listed above)
    
- Reactor
    - Means of providing subscription mechanisms for events like IO, inter-process communication and timers
- Task
    - possibly self-standing aync future scheduled by the runtime (behaves similarly to an OS thread)
    
Tokio / async-std

Send + Sync only if all data carried accross .await points is
    - Eg. Carrying an Rc or a RefCell across will make your Future neither

Since async futures desugar into enums with variants that carry data across await points, try not to carry big variables
Watch out with Mutexes/RwLocks
    - You can create a situation where you dead-lock yourself
        - One task holds a lock, another is scheduled on the same thread while the other is dormant



PINNING
- futures are self-referential types
- because they contain pointers to themselves, moving them would invalidate data
- Pin<T> prevents data from being moved
    - types that can be moved anyway implement Unpin
- If we want to have a trait object of a Future, we need not just a pointer, but a pin as well!

Unpin types:
i32: Unpin, bool: Unpin, char: Unpin



STREAMS
- async equivalent of iterators
- the .next() method returns a Future instead of a value out-right
- can be created from iterators




ASYNC TRAITS
- async traits are trouble because Futures are opaque types
- we need to use trait objects
    - async-trait crate
*/
