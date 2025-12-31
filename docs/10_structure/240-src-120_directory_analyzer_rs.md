# Structure Group: src/120_directory_analyzer.rs

## File: src/120_directory_analyzer.rs

- Layer(s): 120_directory_analyzer.rs
- Language coverage: Rust (4)
- Element types: Function (2), Impl (1), Struct (1)
- Total elements: 4

### Elements

- [Rust | Struct] `DirectoryAnalyzer` (line 0, pub)
  - Signature: `pub struct DirectoryAnalyzer { root : PathBuf , }`
- [Rust | Impl] `impl DirectoryAnalyzer { pub fn new (root : PathBuf) -> Self { Self { root } } pub fn analyze (& self) -> Result < DirectoryAnalysis > { self . analyze_directory (& self . root , None) } fn analyze_directory (& self , path : & Path , parent : Option < PathBuf >) -> Result < DirectoryAnalysis > { let mut files = Vec :: new () ; let mut subdirectories = Vec :: new () ; let mut entries : Vec < PathBuf > = fs :: read_dir (path) ? . filter_map (| entry | entry . ok () . map (| e | e . path ())) . collect () ; entries . sort () ; for entry_path in entries { if entry_path . is_dir () && ! allow_analysis_dir (& self . root , & entry_path) { continue ; } if should_skip_path (& entry_path) { continue ; } if entry_path . is_dir () { let child = self . analyze_directory (& entry_path , Some (path . to_path_buf ())) ? ; if child . has_sources || ! child . subdirectories . is_empty () { subdirectories . push (child) ; } } else if is_source_file (& entry_path) { files . push (entry_path . clone ()) ; } } let has_sources = ! files . is_empty () ; Ok (DirectoryAnalysis { path : path . to_path_buf () , layer : detect_layer (path) , parent , files , subdirectories , has_sources , }) } } . self_ty` (line 0, priv)
- [Rust | Function] `is_source_file` (line 0, priv)
  - Signature: `fn is_source_file (path : & Path) -> bool { matches ! (path . extension () . and_then (| e | e . to_str ()) , Some ("...`
- [Rust | Function] `should_skip_path` (line 0, priv)
  - Signature: `fn should_skip_path (path : & Path) -> bool { let Some (name) = path . file_name () else { return false ; } ; name ==...`
  - Calls: file_name

