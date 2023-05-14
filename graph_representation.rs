//Defining a structure to denote the Graph Data Structure
// We are using Vertex index to denote a vertex and usize as it's data

struct Graph{
    num_vertices: usize,
    adj: Vec<Vec<usize>>,
}

//Now some methods on Graph Structure are defined
impl Graph{
    pub fn new(v: usize) -> Graph{
        Graph{
            num_vertices: v,
            adj: vec![vec![]; v]
        }
    }
    pub fn add_edge(&mut self, v1: usize, v2: usize){
        self.adj[v1].push(v2);
        self.adj[v2].push(v1);
    }

    pub fn print_graph(&self){
        println!("Adjacency List of Graph: ");
        for i in 0..self.num_vertices{
            println!("[{}] -> {:?}", i, self.adj[i]);
        }
    }
}

pub fn main(){
    let mut g = Graph::new(5);
    g.add_edge(0,1);
    g.add_edge(0,2);
    g.add_edge(0,3);
    g.add_edge(3,4);
    g.add_edge(4,2);
    g.print_graph();
}

