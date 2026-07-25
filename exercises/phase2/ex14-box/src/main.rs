use std::fmt::{self, Display, Formatter};

fn main() {

    enum FsNode {
       File (String, u64),
       Folder (String, Vec<Box<FsNode>>),
    }

    impl FsNode {
        fn total_size(&self) -> u64 {
            match self {
                FsNode::File(_, size) => *size,
                FsNode::Folder(_, content) => {
                    content.iter().map(|f|  f.total_size()).sum()
                },
            }
        }

        fn depth(&self) -> usize {
            match self {
                FsNode::File(_,_) => 0,
                FsNode::Folder(_,content ) => {
                    if content.is_empty() { return 0; }
                    else {
                        return 1 + content.iter().map(|f| f.depth()).max().unwrap()
                    }
                }
            }
        }
    }

    impl FsNode {
        fn fmt_indent(&self, f: &mut Formatter<'_>, level: usize) -> fmt::Result {
            match self {
                FsNode::File(name,size) => write!(f,"{}[file] {} ({} bytes)\n", " ".repeat(level*2), name, size),
                FsNode::Folder(name, fs_nodes) => {

                    let _ = write!(f, "{}[dir] {}\n", " ".repeat(level*2), name);
                    for node in fs_nodes.iter() {
                        let _ = node.fmt_indent(f, level + 1);
                    }
                    Ok(())
                },
            }
        }
    }

    impl Display for FsNode  {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.fmt_indent(f, 0)
        }
    }

    let app = FsNode::File(String::from("app"), 1_048_576 );
    let readme = FsNode::File(String::from("README.md"), 512 );
    let librs = FsNode::File(String::from("lib.rs"), 2_048 );
    let mainrs = FsNode::File(String::from("main.rs"), 1_024 );

    let debug = FsNode::Folder(String::from("debug"), vec![Box::new(app)]);
    let target = FsNode::Folder(String::from("target"), vec![Box::new(debug)]);
    let src = FsNode::Folder(String::from("src"), vec![Box::new(mainrs),Box::new(librs)]);

    
    
    let target = FsNode::Folder(String::from("/"), vec![Box::new(src), Box::new(readme), Box::new(target)]);

    print!("{}",target);
    println!("Total size: {} bytes", target.total_size());
    println!("Depth: {}", target.depth());

}