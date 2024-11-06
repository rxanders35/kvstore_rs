# kvstore_rs

#### Rust-Based Distributed Key-Value Store ####
This is my own implementation of a distributed key-value store. My goal with this project is to take inspiration from MIT's "Distributed Systems" class, also known as MIT6.824. In that class is discussed RPC, Raft, sharding, and key-value stores.

I personally am very interested in this topic, and since I love learning Rust, I figured I'd starting building this with it.

#### To Do ####
- [x] Core HashMap data structure (might change to BTreeMap or write my own)
- [x] Protocol Buffer methods
- [ ] Server and Client for Put, Get, Update, and Delete methods
- [ ] Write-Ahead Log (WAL) for persistent storage 
- [ ] Raft Consensus Algorithm for data consistency across nodes
- [ ] ???